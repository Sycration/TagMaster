use crate::{Message, Pane, State, screens::Screen, subwindows::Subwindow, update};
use iced::{Alignment::Center, Border, Element, Length::{self, Fill}, Task, Theme, border::Radius, widget::{self, Space, button, column, container, pane_grid, row, scrollable, text_input}};


pub(crate) fn program_settings(state: &State) -> Element<Message> {
    button("close")
        .on_press(Message::CloseWindow(Subwindow::ProgramSettings))
        .into()
}
