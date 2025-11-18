use crate::{Message, State, subwindows::Subwindow};
use iced::{Element, widget::{button}};


pub(crate) fn program_settings(state: &State) -> Element<Message> {
    button("close")
        .on_press(Message::CloseWindow(Subwindow::ProgramSettings))
        .into()
}
