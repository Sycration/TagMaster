use crate::{Message, State};
use iced::Task;

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) enum Screen {
    #[default]
    Home,
    Project,
}

pub(crate) fn change_screen(state: &mut State, screen: Screen) -> Task<Message> {
    if state.project.is_some() && screen == Screen::Project {
        state.screen = Screen::Project;
    } else {
        state.screen = Screen::Home;
    }
    Task::none()
}
