use crate::{Message, State, subwindows::Subwindow};
use iced::{Element, widget::button};

pub(crate) fn project_settings(state: &State) -> Element<Message> {
    button("close")
        .on_press(Message::CloseWindow(Subwindow::ProjectSettings))
        .into()
}
