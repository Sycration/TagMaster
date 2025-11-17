use crate::{Message, Pane, State, screens::Screen, subwindows::Subwindow, update};
use iced::{Alignment::Center, Border, Element, Length::{self, Fill}, Task, Theme, border::Radius, widget::{self, Space, button, column, container, pane_grid, row, scrollable, text_input}};

#[derive(Debug, Default, Clone)]
pub(crate) struct NewProjState {
    name: String,
}

#[derive(Debug, Clone)]
pub(crate) enum NewProjEvent {
    NameChange(String),
}

pub(crate) fn close_project(state: &mut State) -> Task<Message> {
    state.screen = Screen::Home;
    state.project = None;
    Task::none()
}

pub(crate) fn new_project(state: &mut State) -> Task<Message> {
    let go = update(state, Message::CloseProj)
        .chain(update(state, Message::CloseWindow(Subwindow::NewProject)));
    state.project = Some(crate::project::Project {
        name: state.new_proj_state.name.clone(),
    });
    state.new_proj_state = NewProjState::default();
    state.screen = Screen::Project;
    go
}

pub(crate) fn handle_new_proj_ev(state: &mut NewProjState, ev: NewProjEvent) -> Task<Message> {
    match ev {
        NewProjEvent::NameChange(n) => {
            state.name = n;
            Task::none()
        }
    }
}

pub(crate) fn new_project_view(state: &NewProjState) -> Element<Message> {
    column![
        "Create a new project",
        text_input("Project Name", &state.name)
            .on_input(|s| Message::NewProjMessage(NewProjEvent::NameChange(s))),
        row![
            button("Create")
                .style(button::primary)
                .on_press(Message::NewProj),
            Space::new(Fill, 0),
            button("Cancel")
                .style(button::secondary)
                .on_press(Message::CloseWindow(Subwindow::NewProject)),
        ]
        .padding(40)
    ]
    .align_x(Center)
    .padding(40)
    .spacing(25)
    .into()
}


pub(crate) fn project_page(state: &State) -> widget::Container<'_, Message> {
    container(
        pane_grid(&state.panes, |pane, state, _| {
            pane_grid::Content::new(
                scrollable(container("data").padding(8)).height(Length::Fill),
            )
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();

                container::Style {
                    border: Border {
                        color: palette.background.strong.color,
                        width: 2.0,
                        radius: Radius::new(0),
                    },
                    ..Default::default()
                }
            })
            .title_bar(
                pane_grid::TitleBar::new(widget::stack![match state {
                    Pane::FileList => "File Tree",
                    Pane::DataEntry => "Metadata",
                    Pane::Viewer => "Viewer",
                },])
                .style(|theme: &Theme| {
                    let palette = theme.extended_palette();

                    container::Style {
                        text_color: Some(palette.background.strong.text),
                        background: Some(palette.background.strong.color.into()),
                        ..Default::default()
                    }
                })
                .padding(4),
            )
        })
        .on_resize(6, Message::PaneResized)
        .on_drag(Message::PaneSwap)
        .spacing(3),
    ).center_x(Length::Fill)
}
