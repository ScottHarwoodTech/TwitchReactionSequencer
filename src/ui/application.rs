use iced;
use iced::{Column, Command, Element, Text};

#[derive(Debug)]
pub enum Application {
    Ready,
}

#[derive(Debug, Default)]
struct State {}

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<(), LoadError>),
}

#[derive(Debug, Clone)]
enum LoadError {
    FileError,
    FormatError,
}

async fn dummy() -> Result<(), LoadError> {
    return Ok(());
}

impl iced::Application for Application {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Application, Command<Message>) {
        (
            Application::Ready,
            Command::perform(dummy(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        return String::from("hello world");
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Application::Ready => Column::new()
                .max_width(800)
                .spacing(20)
                .push(Text::new("title"))
                .into(),
        }
    }
}
