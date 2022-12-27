pub mod sequence;
use crate::triggers::triggers::TriggerSource;
use crate::ui::fs_utils::LoadError;
use crate::{sequencer, triggers, ThreadActions};
use crate::{sequencer::device::DeviceTrait, ui::fs_utils::SaveError};
use futures_util::select;
use futures_util::{future, FutureExt};
use iced::futures::channel::oneshot;
use iced::{
    self, button, keyboard, scrollable, Button, Column, Length, Row, Rule, Scrollable, Text,
};
use iced::{Command, Element};
use iced_native::{window, Event};
use sequence::{Sequence, SequenceMessage};
use tokio::sync::watch;

use std::collections::HashMap;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct SequencesState {
    sequences: Vec<sequence::Sequence>,
    scroll: scrollable::State,
    add_sequence_button: button::State,
    save_button: button::State,
    devices: HashMap<String, Box<dyn DeviceTrait>>,
    triggers: HashMap<String, Box<dyn TriggerSource>>,
    tainted: bool,
    start_button: button::State,
    stop_button: button::State,
    listener_sender: Option<tokio::sync::mpsc::Sender<ThreadActions>>,
}

#[derive(Debug, Clone)]
pub enum SequencesMessage {
    Loaded(Result<SequencesState, LoadError>),
    SequenceMessage(usize, SequenceMessage),
    AddSequence,
    SequenceDeleted(Option<String>),
    SequenceCreated(Sequence),
    EventOccurred(iced_native::Event),
    Saved(Option<SaveError>),
    Save,
    StartListeners,
    StopListeners,
    StoppedListeners(()),
    TriggerComplete,
}

#[derive(Debug, Clone)]
pub enum Sequences {
    Loading,
    Error(String),
    Ready(SequencesState),
    UnsavedCloseRequested(SequencesState),
    ShouldExit,
    Running(SequencesState),
}

pub async fn bury(v: tokio::sync::mpsc::Sender<ThreadActions>) -> () {
    v.send(ThreadActions::Stop).await.unwrap();
    return ();
}

impl Sequences {
    pub fn new(
        flags: (
            HashMap<String, Box<dyn DeviceTrait>>,
            HashMap<String, Box<dyn TriggerSource>>,
        ),
    ) -> (Sequences, Command<SequencesMessage>) {
        (
            Sequences::Loading,
            Command::perform(load_sequences(flags.0, flags.1), SequencesMessage::Loaded),
        )
    }

    pub fn should_exit(&self) -> bool {
        match self {
            Sequences::ShouldExit => true,
            _ => false,
        }
    }

    pub fn title(&self) -> String {
        let tainted = match self {
            Sequences::Loading => false,
            Sequences::Ready(state) | Sequences::UnsavedCloseRequested(state) => state.tainted,
            _ => false,
        };

        format!(
            "Twitch Reaction Sequencer{}",
            if tainted { "*" } else { "" }
        )
    }

    pub fn update(&mut self, message: SequencesMessage) -> Command<SequencesMessage> {
        match self {
            Sequences::Loading => match message {
                SequencesMessage::Loaded(Ok(state)) => {
                    *self = Sequences::Ready(state);
                }

                SequencesMessage::Loaded(Err(load_error)) => match load_error {
                    LoadError::FormatError(msg) => *self = Sequences::Error(msg),
                    _ => {}
                },
                _ => {}
            },

            Sequences::Running(state) => match message {
                SequencesMessage::StopListeners => {
                    if state.listener_sender.is_some() {
                        let listender_sender = state.clone().listener_sender.unwrap().clone();
                        return Command::perform(
                            bury(listender_sender.clone()),
                            SequencesMessage::StoppedListeners,
                        );
                    }
                }

                SequencesMessage::StoppedListeners(()) => {
                    *self = Sequences::Ready(SequencesState { ..state.clone() });

                    return Command::none();
                }
                _ => {}
            },

            Sequences::UnsavedCloseRequested(state) | Sequences::Ready(state) => match message {
                SequencesMessage::SequenceMessage(i, sequence_message) => match sequence_message {
                    SequenceMessage::Delete => {
                        let removed_item = state.sequences.remove(i);
                        return Command::perform(
                            delete_file(removed_item.get_filename()),
                            SequencesMessage::SequenceDeleted,
                        );
                    }
                    _ => {
                        if let Some(sequence) = state.sequences.get_mut(i) {
                            sequence.update(sequence_message);
                        }
                    }
                },

                SequencesMessage::AddSequence => {
                    return Command::perform(
                        Sequence::new(state.devices.clone(), state.triggers.clone()),
                        SequencesMessage::SequenceCreated,
                    );
                }

                SequencesMessage::SequenceCreated(sequence) => {
                    state.sequences.push(sequence);
                    *self = Sequences::Ready(SequencesState {
                        tainted: true,
                        ..state.clone()
                    })
                }

                SequencesMessage::SequenceDeleted(msg) => {
                    if msg.is_some() {
                        *self = Sequences::Error(msg.unwrap())
                    } else {
                        *self = Sequences::Ready(SequencesState {
                            tainted: true,
                            ..state.clone()
                        });
                    }
                }

                SequencesMessage::EventOccurred(event) => {
                    if let Event::Window(window::Event::CloseRequested) = event {
                        if state.tainted {
                            *self = Sequences::UnsavedCloseRequested(state.clone());
                        } else {
                            *self = Sequences::ShouldExit
                        }
                    } else if let Event::Keyboard(keyboard::Event::KeyPressed {
                        key_code: keyboard::KeyCode::S,
                        modifiers: keyboard::Modifiers::CTRL,
                    }) = event
                    {
                        return try_save(state);
                    }
                }
                SequencesMessage::Save => return try_save(state),

                SequencesMessage::Saved(_) => {
                    *self = Sequences::Ready(SequencesState {
                        tainted: false,
                        ..state.clone()
                    })
                }

                SequencesMessage::StartListeners => {
                    let (sender, reciever) = tokio::sync::mpsc::channel(1);
                    tokio::spawn(start_listener(
                        state.devices.clone(),
                        state.triggers.clone(),
                        reciever,
                    ));

                    *self = Sequences::Running(SequencesState {
                        listener_sender: Option::Some(sender),
                        ..state.clone()
                    });

                    return Command::none();
                }
                _ => {}
            },

            _ => {}
        }

        Command::none()
    }

    pub fn view(&mut self) -> Element<SequencesMessage> {
        match self {
            Sequences::Loading => Text::new("Loading").into(),
            Sequences::Error(msg) => Text::new(msg.clone()).into(),
            Sequences::Ready(state) => render_when_ready(state).into(),
            Sequences::ShouldExit => Text::new("exiting").into(),
            Sequences::Running(state) => running(state).into(),
            Sequences::UnsavedCloseRequested(state) => {
                let mut c = Column::new().width(Length::Fill).spacing(1);

                let seqs: Element<_> = state
                    .sequences
                    .iter_mut()
                    .enumerate()
                    .fold(
                        Column::new().spacing(20).padding(10),
                        |column: Column<_>, (i, sequence)| {
                            column
                                .push(sequence.view().map(move |message| {
                                    SequencesMessage::SequenceMessage(i, message)
                                }))
                                .push(Rule::horizontal(5))
                        },
                    )
                    .into();

                c = c.push(seqs);

                c = c.push(
                    Button::new(
                        &mut state.add_sequence_button,
                        Text::new("Add Sequence +").size(20),
                    )
                    .on_press(SequencesMessage::AddSequence),
                ); // Add Sequence Button

                let contents = Scrollable::new(&mut state.scroll).push(c);
                return Column::new()
                    .push(
                        Button::new(
                            &mut state.save_button,
                            Row::new()
                                .push(Text::new("You just tried to exit when unsaved"))
                                .push(Text::new("+")),
                        )
                        .on_press(SequencesMessage::Save),
                    )
                    .push(contents)
                    .into(); //TODO change this to a modal
            }
        }
    }
}

fn running(state: &mut SequencesState) -> Element<SequencesMessage> {
    return Column::new()
        .push(Text::new("Running"))
        .push(
            Button::new(&mut state.stop_button, Text::new("Stop"))
                .on_press(SequencesMessage::StopListeners),
        )
        .into();
}

async fn load_sequences(
    devices: HashMap<String, Box<dyn DeviceTrait>>,
    triggers: HashMap<String, Box<dyn TriggerSource>>,
) -> Result<SequencesState, LoadError> {
    let paths = fs::read_dir("./sequences").await; // TODO: this path should be relative to a userdata folder
    let mut sequences = Vec::<Sequence>::new();
    if paths.is_ok() {
        let mut paths = paths.unwrap();

        while let Ok(Some(entry)) = paths.next_entry().await {
            if entry.metadata().await.unwrap().is_dir() {
                continue;
            }

            let path = entry.path();
            if let Ok(file_content) = fs::read(&path).await {
                let get_sequencer = serde_json::from_slice(&file_content);
                if get_sequencer.is_ok() {
                    let sequencer = get_sequencer.unwrap();
                    sequences.push(Sequence::from_existing(
                        sequencer,
                        path.clone(),
                        devices.clone(),
                        triggers.clone(),
                    ));
                } else {
                    return Err(LoadError::FormatError(
                        get_sequencer.err().unwrap().to_string(),
                    ));
                }
            } else {
                return Err(LoadError::FileError);
            }
        }
    } else {
        return Err(LoadError::FileError);
    };

    return Ok(SequencesState {
        sequences: sequences,
        scroll: scrollable::State::new(),
        add_sequence_button: button::State::new(),
        save_button: button::State::new(),
        start_button: button::State::new(),
        stop_button: button::State::new(),
        devices: devices.clone(),
        triggers: triggers.clone(),
        tainted: false,
        listener_sender: Option::None,
    });
}

async fn save_sequences(sequences: Vec<Sequence>) -> Option<SaveError> {
    for sequence in sequences {
        let json = serde_json::to_string_pretty(&sequence.to_reaction_seqeunce());

        if json.is_err() {
            return Some(SaveError::FormatError(json.unwrap_err().to_string()));
        }

        fs::write(&sequence.get_filename(), &json.unwrap())
            .await
            .unwrap();
    }

    return None;
}

async fn delete_file(filename: String) -> Option<String> {
    if !fs::metadata(filename.clone()).await.is_ok() {
        return None;
    }
    if let Err(v) = fs::remove_file(filename).await {
        return Some(v.to_string());
    };

    return None;
}

fn render_when_ready(state: &mut SequencesState) -> Scrollable<SequencesMessage> {
    let mut c = Column::new().width(Length::Fill).spacing(1).push(
        Button::new(&mut state.start_button, Text::new("Start"))
            .on_press(SequencesMessage::StartListeners),
    );

    let seqs: Element<_> = state
        .sequences
        .iter_mut()
        .enumerate()
        .fold(
            Column::new().spacing(20).padding(10),
            |column: Column<_>, (i, sequence)| {
                column
                    .push(
                        sequence
                            .view()
                            .map(move |message| SequencesMessage::SequenceMessage(i, message)),
                    )
                    .push(Rule::horizontal(5))
            },
        )
        .into();

    c = c.push(seqs);

    c = c.push(
        Button::new(
            &mut state.add_sequence_button,
            Text::new("Add Sequence +").size(20),
        )
        .on_press(SequencesMessage::AddSequence),
    ); // Add Sequence Button

    return Scrollable::new(&mut state.scroll).push(c);
}

fn try_save(state: &mut SequencesState) -> Command<SequencesMessage> {
    if state.tainted {
        return Command::perform(
            save_sequences(state.sequences.clone()),
            SequencesMessage::Saved,
        );
    }

    return Command::none();
}

async fn start_listener(
    device_set: HashMap<String, Box<dyn DeviceTrait>>,
    triggers: HashMap<String, Box<dyn TriggerSource>>,
    mut listener: tokio::sync::mpsc::Receiver<ThreadActions>,
) {
    let (trigger_sequence, trigger_sequence_reciever) = watch::channel(sequencer::QueueEvent {
        sequence_id: String::from("empty"),
    });

    let (task_handler_sender, task_handler_reciever) = watch::channel(());

    let sequencer_queue = sequencer::watch_queue(
        device_set,
        trigger_sequence_reciever,
        task_handler_reciever.clone(),
    );

    let trigger_manager =
        triggers::watch_trigger_sources(triggers, trigger_sequence, task_handler_reciever.clone());
    let mut listeners = Box::pin(future::try_join(trigger_manager, sequencer_queue).fuse());

    let mut l = Box::pin(listener.recv().fuse());

    select! {
        _x = listeners => println!("Listeners finished"),
        _v = l => {
            println!("Told to stop");
            task_handler_sender.send(()).unwrap();
        }
    }

    listeners.await.unwrap();
    println!("Finished listeners")
}
