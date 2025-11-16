use iced::Element;
use iced::Length;
use iced::Theme;
use iced::alignment::Horizontal;
use iced::alignment::Horizontal::Left;
use iced::theme::palette;
use iced::widget;
use iced::widget::PaneGrid;
use iced::widget::button;
use iced::widget::button::Style;
use iced::widget::column;
use iced::widget::container;
use iced::widget::horizontal_rule;
use iced::widget::pane_grid;
use iced::widget::pane_grid::TitleBar;
use iced::widget::row;

#[derive(Debug, Clone)]
enum Message {
    ChangeScreen(Screen),
    NewProj,
    CloseProj,
    PaneResized(pane_grid::ResizeEvent),
}

#[derive(Debug, Clone, Default, PartialEq)]
enum Screen {
    #[default]
    Home,
    Project,
}

#[derive(Debug, Clone, PartialEq)]
enum Pane {
    FileList,
    DataEntry, 
    Viewer
}

#[derive(Debug, Clone)]
struct State {
    screen: Screen,
    panes: pane_grid::State<Pane>,
    project: Option<Project>,
}

impl Default for State {
    fn default() -> Self {
        let mut flist = pane_grid::State::new(Pane::FileList);
        let viewer = flist.0.split(pane_grid::Axis::Vertical, flist.1, Pane::Viewer).unwrap();
        flist.0.split(pane_grid::Axis::Horizontal, flist.1, Pane::DataEntry).unwrap();
        Self { 
            panes: flist.0,
            project: None,
            screen: Screen::Home
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Project;

pub fn main() -> iced::Result {
    iced::application("TagMaster", update, view).theme(theme).run()
}

fn theme(state: &State) -> Theme {
    Theme::TokyoNight
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::ChangeScreen(screen) => {
            if state.project.is_some() && screen == Screen::Project {
                state.screen = Screen::Project;
            } else {
                state.screen = Screen::Home;
            }
        }
        Message::CloseProj => {
            state.screen = Screen::Home;
            state.project = None;
        }
        Message::NewProj => {
            update(state, Message::CloseProj);
            state.project = Some(Project);
            state.screen = Screen::Project;
        }
        Message::PaneResized(resize_event) => state.panes.resize(resize_event.split, resize_event.ratio),
    }
}

fn view(state: &State) -> Element<Message> {
    let top_bar = row(
        (0..=(if state.project.is_some() { 1 } else { 0 })).map(|i| {
            if i == 0 {
                widget::button("Home")
                    .style(|theme: &Theme, _| {
                        let palette = theme.extended_palette();
                        if state.screen == Screen::Home {
                            button::Style::default().with_background(palette.primary.strong.color)
                        } else {
                            button::Style::default().with_background(palette.secondary.strong.color)
                        }
                    })
                    .on_press(Message::ChangeScreen(Screen::Home))
                    .into()
            } else {
                widget::button("Project")
                    .style(|theme: &Theme, _| {
                        let palette = theme.extended_palette();
                        if state.screen == Screen::Project {
                            button::Style::default().with_background(palette.primary.strong.color)
                        } else {
                            button::Style::default().with_background(palette.secondary.strong.color)
                        }
                    })
                    .on_press(Message::ChangeScreen(Screen::Project))
                    .into()
            }
        }),
    );

    let body = match state.screen {
        Screen::Home => container(column![
            "Welcome to TagMaster",
            button("New Project").on_press(Message::NewProj)
        ])
        .center_x(Length::Fill),
        Screen::Project => container(
            pane_grid(&state.panes, |pane, state, _| {
        pane_grid::Content::new(iced_aw::Card::new(match state {
            Pane::FileList => "File Tree",
            Pane::DataEntry => "Metadata",
            Pane::Viewer => "Viewer",
        }, container("data").height(Length::Fill)).height(Length::Fill))
            }).on_resize(6, Message::PaneResized)
            .spacing(3)
        )
        .center_x(Length::Fill),
    }
    .height(Length::Fill);

    widget::container(widget::column![top_bar, horizontal_rule(2), body].spacing(10))
        .padding(10)
        .into()
}
