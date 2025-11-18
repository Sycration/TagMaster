use crate::{Message, subwindows::Subwindow};
use iced::{Alignment::Center, Border, Length::{self, Fill, FillPortion}, Theme, border::Radius, widget::{self, Space, button, column, container, row, scrollable, span, text}};


pub(crate) fn homepage<'a>() -> widget::Container<'a, Message> {
    container(
        column![
            container(text("TagMaster").size(80)).height(80),
            Space::new(0, 25),

            widget::rich_text([
                span("A project by "),
                span("GenEq UC Berkeley")
                    .link(Message::OpenLink(
                        "https://cejce.berkeley.edu/geneq".to_string()
                    ))
                    .underline(true)
            ])
            .height(25),

            row![
                container("New Project")
                    .align_x(Center)
                    .width(FillPortion(2)),
                container("Recent Projects")
                    .width(FillPortion(2))
                    .align_x(Center)
            ],
            row![
                container(
                    column![
                        button("Create Project...")
                            .on_press(Message::OpenWindow(Subwindow::NewProject)),
                    ]
                    .align_x(Center)
                    .align_x(Center)
                )
                .height(Fill)
                .align_x(Center)
                .align_y(Center)
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
                .width(FillPortion(2)),
                container(scrollable(
                    column![
                        //TODO project files
                    ]
                    .align_x(Center)
                    .spacing(10)
                    .padding(10)
                    .width(Fill)
                ))
                .width(FillPortion(2))
                .height(Fill)
                .align_x(Center)
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
            ],
            Space::new(0, 10),

            widget::rich_text([
                span("This software is licenced under the "),
                span("GNU General Public Licence v.3")
                    .link(Message::OpenLink(
                        "https://www.gnu.org/licenses/gpl-3.0.en.html#license-text".to_string()
                    ))
                    .underline(true),
                span("\nSource code is available on "),
                span("GitHub")
                    .link(Message::OpenLink(
                        "https://github.com/Sycration/TagMaster/tree/main".to_string()
                    ))
                    .underline(true)
            ])
            .align_x(Center),
            Space::new(0, 0)
        ]
        .align_x(Center)
        
    )
    .center_x(Length::Fill)
}
