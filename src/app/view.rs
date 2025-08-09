use iced::widget::{button, column, row, scrollable, text, text_input};
use iced::{Color, Length};

use super::widgets::OutsideCommit;
use super::{AppState, Message};

pub fn view(state: &AppState) -> iced::Element<'_, Message> {
    // List of selected files (or placeholder text)
    let files_column = if state.selected_files.is_empty() {
        column![text("No PDF files selected")]
    } else {
        let mut col = column![];
        for (i, f) in state.selected_files.iter().enumerate() {
            let is_editing = state.editing_index == Some(i);
            let name_widget: iced::Element<'_, Message> = if is_editing {
                let input = text_input("File name (without .pdf)", &state.editing_buffer)
                    .on_input(Message::NameChanged)
                    .on_submit(Message::CommitEdit)
                    .padding(4)
                    .size(16)
                    .style(|_theme, _status| iced::widget::text_input::Style {
                        background: iced::Background::Color(Color::from_rgb8(30, 30, 30)),
                        border: iced::Border {
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
                    .style(|_theme, status| {
                        let base = Color::from_rgb8(80, 80, 80);
                        let hovered = Color::from_rgb8(110, 110, 110);
                        let color = match status {
                            button::Status::Hovered => hovered,
                            _ => base,
                        };
                        button::Style {
                            background: Some(iced::Background::Color(color)),
                            text_color: Color::WHITE,
                            border: iced::Border {
                                radius: 4.0.into(),
                                width: 0.0,
                                color: Color::TRANSPARENT,
                            },
                            shadow: iced::Shadow::default(),
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
                .style(|_theme, status| {
                    let base = Color::from_rgb8(0xD9, 0x2F, 0x2F);
                    let hovered = Color::from_rgb8(0xE5, 0x46, 0x46);
                    let pressed = Color::from_rgb8(0xB8, 0x23, 0x23);
                    let color = match status {
                        button::Status::Hovered => hovered,
                        button::Status::Pressed => pressed,
                        _ => base,
                    };
                    button::Style {
                        background: Some(iced::Background::Color(color)),
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
                    .style(|_theme, _status| button::Style {
                        background: None,
                        text_color: Color::WHITE,
                        border: iced::Border {
                            radius: 0.0.into(),
                            width: 0.0,
                            color: Color::TRANSPARENT,
                        },
                        shadow: iced::Shadow::default(),
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
            text(status).style(|_theme| iced::widget::text::Style {
                color: Some(Color::from_rgb8(0xD9, 0x2F, 0x2F)),
            })
        } else if status.starts_with("Sent ") {
            text(status).style(|_theme| iced::widget::text::Style {
                color: Some(Color::from_rgb8(0x18, 0x7A, 0x3E)),
            })
        } else {
            text(status)
        }
    } else {
        text("")
    };
    let content = column![file_list, status_row, row![open_btn, send_btn].spacing(16)]
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
