use std::{env::current_dir, path::PathBuf};

use crate::{Message, Pane, State, file_tree, screens::Screen, subwindows::Subwindow, update};
use iced::{
    Alignment::Center,
    Border, Element,
    Length::{self, Fill},
    Task, Theme,
    border::Radius,
    widget::{
        self, Column, Space, TextInput, button, column, container, pane_grid, row, scrollable,
        text, text_input,
    },
};

#[derive(Debug, Default, Clone)]
pub(crate) struct NewProjState {
    name: String,
    top_path: PathBuf,
}

#[derive(Debug, Clone)]
pub(crate) enum NewProjEvent {
    NameChange(String),
    PickPath,
    SetPath(PathBuf),
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
    state.file_tree_state.path = state.new_proj_state.top_path.clone();
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
        NewProjEvent::PickPath => Task::perform(rfd::AsyncFileDialog::new().pick_folder(), |x| {
            if let Some(f) = x.as_ref() {
                return Message::NewProjMessage(NewProjEvent::SetPath(f.path().to_path_buf()));
            }
            Message::None
        }),
        NewProjEvent::SetPath(path_buf) => {
            state.top_path = path_buf;
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
            button("Select directory").on_press(Message::NewProjMessage(NewProjEvent::PickPath)),
            TextInput::new("path", &state.top_path.to_string_lossy()),
        ],
        row![
            Space::new(40, 0),
            button("Create")
                .style(button::primary)
                .on_press(Message::NewProj),
            Space::new(Fill, 0),
            button("Cancel")
                .style(button::secondary)
                .on_press(Message::CloseWindow(Subwindow::NewProject)),
            Space::new(40, 0),
        ]
    ]
    .align_x(Center)
    .padding(40)
    .spacing(25)
    .into()
}

pub(crate) fn project_page(state: &State) -> widget::Container<'_, Message> {
    container(
        pane_grid(&state.panes, |pane, current_pane, _| {
            pane_grid::Content::new(
                scrollable(
                    match current_pane {
                        Pane::FileList => container(file_tree::file_tree(&state.file_tree_state)),
                        Pane::DataEntry => container("Data Entry"),
                        Pane::Viewer => container("Viewer"),
                    }
                    .padding(8),
                )
                .height(Length::Fill)
                .width(Length::Fill),
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
                pane_grid::TitleBar::new(widget::stack![match current_pane {
                    Pane::FileList => container(file_tree::title_bar(&state.file_tree_state)),
                    Pane::DataEntry => container("Metadata"),
                    Pane::Viewer => container("Viewer"),
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
    )
    .center_x(Length::Fill)
}
