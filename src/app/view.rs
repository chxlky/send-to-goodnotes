use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length, Shadow, Theme};

use super::widgets::OutsideCommit;
use super::{AppState, Message};

pub fn view(state: &AppState) -> Element<'_, Message> {
    if state.show_settings {
        settings_view(state)
    } else {
        main_view(state)
    }
}

fn settings_view(state: &AppState) -> Element<'_, Message> {
    let header = row![
        text("Settings").size(24),
        container(
            button(text("X"))
                .style(|_theme: &Theme, status| {
                    let base = Color::from_rgb8(80, 80, 80);
                    let hovered = Color::from_rgb8(110, 110, 110);

                    let color = match status {
                        button::Status::Hovered => hovered,
                        _ => base,
                    };

                    button::Style {
                        background: Some(Background::Color(color)),
                        text_color: Color::WHITE,
                        border: Border {
                            radius: 4.0.into(),
                            width: 0.0,
                            color: Color::TRANSPARENT,
                        },
                        shadow: Shadow::default(),
                    }
                })
                .on_press(Message::CloseSettings)
                .padding(8)
        )
        .width(Length::Fill)
        .align_x(Alignment::End)
    ];

    let input_style = |_theme: &iced::Theme, _status| text_input::Style {
        background: Background::Color(Color::from_rgb8(30, 30, 30)),
        border: Border {
            radius: 4.0.into(),
            width: 1.0,
            color: Color::from_rgb8(120, 120, 120),
        },
        icon: Default::default(),
        placeholder: Color::from_rgb8(150, 150, 150),
        value: Color::WHITE,
        selection: Color::from_rgb8(80, 80, 160),
    };

    let smtp_host_input = column![
        text("SMTP Host").size(14),
        text_input("e.g. smtp.gmail.com", &state.settings.smtp_host)
            .on_input(Message::SmtpHostChanged)
            .padding(8)
            .style(input_style)
    ]
    .spacing(4);

    let smtp_port_input = column![
        text("SMTP Port").size(14),
        text_input("e.g. 587", &state.settings.smtp_port)
            .on_input(Message::SmtpPortChanged)
            .padding(8)
            .style(input_style)
    ]
    .spacing(4);

    let from_email_input = column![
        text("From Email").size(14),
        text_input("your.email@gmail.com", &state.settings.from_email)
            .on_input(Message::FromEmailChanged)
            .padding(8)
            .style(input_style)
    ]
    .spacing(4);

    let to_email_input = column![
        text("To Email (GoodNotes)").size(14),
        text_input("your.goodnotes@email", &state.settings.to_email)
            .on_input(Message::ToEmailChanged)
            .padding(8)
            .style(input_style)
    ]
    .spacing(4);

    let app_password_input = column![
        text("App Password").size(14),
        text_input("App-specific password", &state.settings.app_password)
            .on_input(Message::AppPasswordChanged)
            .padding(8)
            .secure(true)
            .style(input_style)
    ]
    .spacing(4);

    let save_button = button(text("Save Settings"))
        .style(|_theme: &iced::Theme, status| {
            let base = Color::from_rgb8(34, 139, 34);
            let hovered = Color::from_rgb8(50, 155, 50);

            let color = match status {
                button::Status::Hovered => hovered,
                _ => base,
            };

            button::Style {
                background: Some(Background::Color(color)),
                text_color: Color::WHITE,
                border: Border {
                    radius: 4.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow::default(),
            }
        })
        .on_press(Message::SaveSettings)
        .padding(12);

    let content = column![
        header,
        text("Configure your email settings:").size(16),
        smtp_host_input,
        smtp_port_input,
        from_email_input,
        to_email_input,
        app_password_input,
        save_button,
    ]
    .spacing(20)
    .padding(16);

    scrollable(content).into()
}

fn main_view(state: &AppState) -> Element<'_, Message> {
    // List of selected files (or placeholder text)
    let files_column = if state.selected_files.is_empty() {
        column![text("No PDF files selected")]
    } else {
        let mut col = column![];
        for (i, f) in state.selected_files.iter().enumerate() {
            let is_editing = state.editing_index == Some(i);

            let name_widget: Element<'_, Message> = if is_editing {
                let input = text_input("File name (without .pdf)", &state.editing_buffer)
                    .on_input(Message::NameChanged)
                    .on_submit(Message::CommitEdit)
                    .padding(4)
                    .size(16)
                    .style(|_theme: &iced::Theme, _status| text_input::Style {
                        background: Background::Color(Color::from_rgb8(30, 30, 30)),
                        border: Border {
                            radius: 4.0.into(),
                            width: 1.0,
                            color: Color::from_rgb8(120, 120, 120),
                        },
                        icon: Default::default(),
                        placeholder: Color::from_rgb8(150, 150, 150),
                        value: Color::WHITE,
                        selection: Color::from_rgb8(80, 80, 160),
                    });

                // Provide small cancel button while editing
                let cancel = button(text("Cancel"))
                    .style(|_theme: &iced::Theme, status| {
                        let base = Color::from_rgb8(80, 80, 80);
                        let hovered = Color::from_rgb8(110, 110, 110);

                        let color = match status {
                            button::Status::Hovered => hovered,
                            _ => base,
                        };

                        button::Style {
                            background: Some(Background::Color(color)),
                            text_color: Color::WHITE,
                            border: Border {
                                radius: 4.0.into(),
                                width: 0.0,
                                color: Color::TRANSPARENT,
                            },
                            shadow: Shadow::default(),
                        }
                    })
                    .on_press(Message::CancelEdit)
                    .padding(2);

                row![input, cancel].spacing(4).into()
            } else {
                text(&state.file_names[i]).into()
            };

            let name_container = iced::widget::container(name_widget).width(Length::Fill);
            let remove_btn = button(text("X"))
                .style(|_theme: &iced::Theme, status| {
                    let base = Color::from_rgb8(0xD9, 0x2F, 0x2F);
                    let hovered = Color::from_rgb8(0xE5, 0x46, 0x46);
                    let pressed = Color::from_rgb8(0xB8, 0x23, 0x23);

                    let color = match status {
                        button::Status::Hovered => hovered,
                        button::Status::Pressed => pressed,
                        _ => base,
                    };

                    button::Style {
                        background: Some(Background::Color(color)),
                        text_color: Color::WHITE,
                        border: iced::Border {
                            radius: 4.0.into(),
                            width: 0.0,
                            color: Color::TRANSPARENT,
                        },
                        shadow: iced::Shadow::default(),
                    }
                })
                .on_press(Message::Remove(f.clone()));

            // Clicking the row when not editing activates edit mode
            let base_row = row![name_container, remove_btn].spacing(8);
            let row_item: iced::Element<'_, Message> = if !is_editing {
                let edit_btn = button(base_row)
                    .style(|_theme: &iced::Theme, _status| button::Style {
                        background: None,
                        text_color: Color::WHITE,
                        border: Border {
                            radius: 0.0.into(),
                            width: 0.0,
                            color: Color::TRANSPARENT,
                        },
                        shadow: Shadow::default(),
                    })
                    .on_press(Message::EditName(i))
                    .padding(0);

                edit_btn.into()
            } else {
                base_row.into()
            };

            col = col.push(row_item);
        }

        col
    };

    let file_list = scrollable(files_column.spacing(4)).height(Length::Fill);
    let open_btn = button("Open file(s)").on_press(Message::OpenFiles);
    let can_send =
        !state.selected_files.is_empty() && state.selected_files.len() <= 5 && !state.sending;

    let send_label = if state.selected_files.len() > 5 {
        format!("Too many ({} > 5)", state.selected_files.len())
    } else if state.sending {
        "Sending...".to_string()
    } else {
        "Send".to_string()
    };

    let mut send_btn = button(text(send_label.clone()));
    if can_send {
        send_btn = send_btn.on_press(Message::Send);
    }

    let status_row = if let Some(status) = &state.status {
        let is_error = status.starts_with("Error:") || status.starts_with("Too many");

        if is_error {
            text(status).style(|_theme: &iced::Theme| text::Style {
                color: Some(Color::from_rgb8(0xD9, 0x2F, 0x2F)),
            })
        } else if status.starts_with("Sent ") {
            text(status).style(|_theme: &iced::Theme| text::Style {
                color: Some(Color::from_rgb8(0x18, 0x7A, 0x3E)),
            })
        } else {
            text(status)
        }
    } else {
        text("")
    };

    // Settings button for bottom right
    let settings_button = button(text("Settings"))
        .style(|_theme: &iced::Theme, status| {
            let base = Color::from_rgb8(80, 80, 80);
            let hovered = Color::from_rgb8(110, 110, 110);

            let color = match status {
                button::Status::Hovered => hovered,
                _ => base,
            };

            button::Style {
                background: Some(Background::Color(color)),
                text_color: Color::WHITE,
                border: Border {
                    radius: 4.0.into(),
                    width: 0.0,
                    color: Color::TRANSPARENT,
                },
                shadow: Shadow::default(),
            }
        })
        .on_press(Message::OpenSettings)
        .padding(8);

    let content = column![
        file_list,
        status_row,
        row![
            open_btn,
            send_btn,
            container(settings_button)
                .width(Length::Fill)
                .align_x(Alignment::End)
        ]
        .spacing(16)
    ]
    .spacing(12)
    .padding(16);

    // Wrap with OutsideCommit so clicks outside will commit edit
    let editing_active = state.editing_index.is_some();
    OutsideCommit::new(
        content.into(),
        editing_active,
        if editing_active {
            Some(Message::CommitEdit)
        } else {
            None
        },
    )
    .into()
}
