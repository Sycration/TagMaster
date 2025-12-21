use std::{env::current_dir, path::PathBuf};

use crate::{Message, Pane, State, file_tree, screens::Screen, subwindows::Subwindow, update};
use iced::{
    Alignment::Center,
    Border, Element,
    Length::{self, Fill},
    Task, Theme,
    advanced::graphics::text::cosmic_text::Font,
    border::Radius,
    widget::{
        self, Button, Column, Row, Space, TextInput, button, column, container, pane_grid, row,
        scrollable, text, text_input,
    },
};

#[derive(Debug, Default, Clone)]
pub(crate) struct NewProjState {
    top_url: String,
}

#[derive(Debug, Clone)]
pub(crate) enum NewProjEvent {
    SetUrl(String),
}

pub(crate) fn close_project(state: &mut State) -> Task<Message> {
    state.screen = Screen::Home;
    state.project = None;
    Task::none()
}

pub(crate) fn new_project(state: &mut State) -> Task<Message> {
    let go = update(state, Message::CloseProj)
        .chain(update(state, Message::CloseWindow(Subwindow::NewProject)));
    state.project = Some(crate::project::Project {});
    //state.file_tree_state.path = state.new_proj_state.top_url.clone();
    state.new_proj_state = NewProjState::default();
    state.screen = Screen::Project;
    go
}

pub(crate) fn handle_new_proj_ev(state: &mut NewProjState, ev: NewProjEvent) -> Task<Message> {
    match ev {
        NewProjEvent::SetUrl(url) => {
            state.top_url = url;
            Task::none()
        }
    }
}

pub(crate) fn new_project_view(state: &State) -> Element<Message> {
    column![
        "Create a new project",
        column![
            TextInput::new(
                "https://berkeley.app.box.com/folder/123456789",
                &state.new_proj_state.top_url
            )
            .on_input(|u| Message::NewProjMessage(NewProjEvent::SetUrl(u))),
            text("Copy and paste the box folder URL here"),
        ]
        .spacing(10),
        row![
            Space::new(40, 0),
            button("Create")
                .style(button::primary)
                .on_press_maybe(state.box_token.as_ref().map(|_| Message::NewProj)),
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
