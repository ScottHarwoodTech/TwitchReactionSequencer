mod native;

pub mod horizontal_scrollable {
    //! Navigate an endless amount of content with a scrollbar.
    pub use super::native::horizontal_scrollable::State;
    pub use iced_native::widget::scrollable::{style::Scrollbar, style::Scroller, StyleSheet};
    /// A widget that can vertically display an infinite amount of content
    /// with a scrollbar.
    pub type HorizontalScrollable<'a, Message> =
        crate::custom_widgets::native::horizontal_scrollable::HorizontalScrollable<
            'a,
            Message,
            iced::Renderer,
        >;
}
