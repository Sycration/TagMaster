use std::fmt::Debug;
use std::process::id;

use iced::Alignment::Center;
use iced::Background;
use iced::Border;
use iced::Element;
use iced::Length;
use iced::Length::Fill;
use iced::Pixels;
use iced::Subscription;
use iced::Task;
use iced::Theme;
use iced::advanced::graphics::futures::MaybeSend;
use iced::alignment::Horizontal;
use iced::alignment::Horizontal::Left;
use iced::application::Update;
use iced::border::Radius;
use iced::theme::palette;
use iced::widget;
use iced::widget::PaneGrid;
use iced::widget::Space;
use iced::widget::button;
use iced::widget::button::Style;
use iced::widget::column;
use iced::widget::container;
use iced::widget::horizontal_rule;
use iced::widget::pane_grid;
use iced::widget::pane_grid::Target;
use iced::widget::pane_grid::TitleBar;
use iced::widget::row;
use iced::widget::rule;
use iced::widget::scrollable;
use iced::widget::text;
use iced::window;
use iced::window::Id;
use iced::window::Settings;
use iced_aw::Quad;
use iced_aw::style::colors::WHITE;

#[derive(Debug, Clone)]
enum Message {
    None,
    Debug(String),
    OpenWindow(Subwindow),
    CloseWindow(Subwindow),
    CloseWinById(Id),
    ChangeScreen(Screen),
    NewProj,
    CloseProj,
    PaneResized(pane_grid::ResizeEvent),
    PaneSwap(pane_grid::DragEvent),
}

#[derive(Debug, Clone, Default, PartialEq)]
enum Screen {
    #[default]
    Home,
    Project,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Subwindow {
    Main,
    ProjectSettings,
    ProgramSettings,
}

#[derive(Debug, Clone, PartialEq)]
enum Pane {
    FileList,
    DataEntry,
    Viewer,
}

#[derive(Debug, Clone)]
struct State {
    windows: Vec<(Id, Subwindow)>,
    screen: Screen,
    panes: pane_grid::State<Pane>,
    project: Option<Project>,
}

impl Default for State {
    fn default() -> Self {
        let mut flist = pane_grid::State::new(Pane::FileList);
        let viewer = flist
            .0
            .split(pane_grid::Axis::Vertical, flist.1, Pane::Viewer)
            .unwrap();
        flist
            .0
            .split(pane_grid::Axis::Horizontal, flist.1, Pane::DataEntry)
            .unwrap();
        Self {
            windows: vec![],
            panes: flist.0,
            project: None,
            screen: Screen::Home,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Project;

pub fn main() -> iced::Result {
    iced::daemon("TagMaster", update, view)
        .theme(theme)
        .subscription(subscription)
        .run_with(|| {
            (
                State::default(),
                Task::done(Message::OpenWindow(Subwindow::Main)),
            )
        })
}

fn subscription(state: &State) -> Subscription<Message> {
    window::events().map(|(id, ev)| {
        match ev {
            window::Event::Closed => Message::CloseWinById(id),

            window::Event::CloseRequested => Message::CloseWinById(id),
            _ => Message::None,
        }
    })
}

fn theme(_state: &State, _id: Id) -> Theme {
    Theme::TokyoNight
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::None => Task::none(),
        Message::Debug(s) => {
            Task::none()
        }
        Message::ChangeScreen(screen) => {
            if state.project.is_some() && screen == Screen::Project {
                state.screen = Screen::Project;
            } else {
                state.screen = Screen::Home;
            }
            Task::none()
        }
        Message::OpenWindow(sw) => {
            let window = match sw {
                Subwindow::Main => {
                    if state.windows.iter().find(|x| x.1 == sw).is_none() {
                        let window = window::open(Settings::default());
                        state.windows.push((window.0, sw));
                        window.1
                    } else {
                        Task::none()
                    }
                }
                Subwindow::ProjectSettings => {
                    if state.windows.iter().find(|x| x.1 == sw).is_none() {
                        let window = window::open(Settings {
                            size: iced::Size {
                                width: 300.0,
                                height: 500.0,
                            },
                            ..Default::default()
                        });
                        state.windows.push((window.0, sw));
                        window.1
                    } else {
                        Task::none()
                    }
                }
                Subwindow::ProgramSettings => {
                    if state.windows.iter().find(|x| x.1 == sw).is_none() {
                        let window = window::open(Settings {
                            size: iced::Size {
                                width: 300.0,
                                height: 500.0,
                            },
                            ..Default::default()
                        });
                        state.windows.push((window.0, sw));
                        window.1
                    } else {
                        Task::none()
                    }
                }
            };
            window.map(|x| Message::Debug(format!("{:?}", x)))
        }

        Message::CloseWindow(sw) => {
            let old_windows = state.windows.clone();
            if let Some(id) = old_windows.iter().find(|x| x.1 == sw) {
                if id.1 == Subwindow::Main {
                    update(state, Message::CloseProj);
                    window::close(id.0).chain(iced::exit())
                } else {
                    let window = window::close(id.0);
                    state.windows.retain(|w| w.1 != id.1);
                    window
                }
            } else {
                Task::none()
            }
        }

        Message::CloseWinById(id) => {
            if Some(Subwindow::Main) == state.windows.iter().find(|x| x.0 == id).map(|x|x.1) {
                update(state, Message::CloseProj);
                window::close(id).chain(iced::exit())
            } else {
                let window = window::close(id);
                state.windows.retain(|w| w.0 != id);
                window
            }
        }
        Message::CloseProj => {
            state.screen = Screen::Home;
            state.project = None;
            Task::none()
        }
        Message::NewProj => {
            update(state, Message::CloseProj);
            state.project = Some(Project);
            state.screen = Screen::Project;
            Task::none()
        }
        Message::PaneResized(resize_event) => {
            state.panes.resize(resize_event.split, resize_event.ratio);
            Task::none()
        }
        Message::PaneSwap(pane_grid::DragEvent::Dropped { pane, target }) => {
            state.panes.drop(pane, target);
            Task::none()
        }
        Message::PaneSwap(_) => Task::none(),
    }
}

fn view(state: &State, window_id: window::Id) -> Element<Message> {
    if let Some(window) = &state.windows.iter().find(|x| x.0 == window_id) {
        match window.1 {
            Subwindow::Main => main_window(state),
            Subwindow::ProjectSettings => project_settings(state),
            Subwindow::ProgramSettings => program_settings(state),
        }
    } else {
        "no render".into()
    }
}

fn main_window(state: &State) -> Element<Message> {
    let top_bar = row![
        row(
            (0..=(if state.project.is_some() { 1 } else { 0 })).map(|i| {
                if i == 0 {
                    widget::button("Home")
                        .style(|theme: &Theme, _| {
                            let palette = theme.extended_palette();

                            button::Style {
                                background: Some(Background::Color(
                                    if state.screen == Screen::Home {
                                        palette.primary.strong.color
                                    } else {
                                        palette.secondary.strong.color
                                    },
                                )),
                                ..Default::default()
                            }
                        })
                        .on_press(Message::ChangeScreen(Screen::Home))
                        .into()
                } else {
                    widget::button("Project")
                        .style(|theme: &Theme, _| {
                            let palette = theme.extended_palette();

                            button::Style {
                                background: Some(Background::Color(
                                    if state.screen == Screen::Project {
                                        palette.primary.strong.color
                                    } else {
                                        palette.secondary.strong.color
                                    },
                                )),
                                ..Default::default()
                            }
                        })
                        .on_press(Message::ChangeScreen(Screen::Project))
                        .into()
                }
            }),
        )
        .spacing(10),
        Space::new(Fill, 0),
        row((0..=(if state.project.is_some() { 1 } else { 0 }))
            .rev()
            .map(|i| {
                if i == 0 {
                    widget::button("Program settings")
                        .on_press(Message::OpenWindow(Subwindow::ProgramSettings))
                        .into()
                } else {
                    widget::button("Project options")
                        .on_press(Message::Debug("projopt".to_string()))
                        .into()
                }
            }),)
        .spacing(10),
    ]
    .width(Length::Fill)
    .align_y(Center);

    let body = match state.screen {
        Screen::Home => container(column![
            "Welcome to TagMaster",
            button("New Project").on_press(Message::NewProj)
        ])
        .center_x(Length::Fill),
        Screen::Project => container(
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
        )
        .center_x(Length::Fill),
    }
    .height(Length::Fill);

    widget::container(widget::column![top_bar, horizontal_rule(2), body].spacing(10))
        .padding(10)
        .into()
}

fn program_settings(state: &State) -> Element<Message> {
    button("close")
        .on_press(Message::CloseWindow(Subwindow::ProgramSettings))
        .into()
}

fn project_settings(state: &State) -> Element<Message> {
    button("close")
        .on_press(Message::CloseWindow(Subwindow::ProjectSettings))
        .into()
}
