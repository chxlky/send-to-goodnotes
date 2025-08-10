#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod email;

use app::{AppState, Message};
use iced::{Result as IcedResult, Size, Task};
use std::path::{Path, PathBuf};

pub fn main() -> IcedResult {
    iced::application("Send to Goodnotes", update, view)
        .centered()
        .window_size(Size::new(800.0, 600.0))
        .run_with(|| {
            let mut state = AppState::default();

            // Initialize config manager and load settings
            let config_manager = config::ConfigManager::new().ok();
            #[allow(clippy::collapsible_if)]
            if let Some(ref manager) = config_manager {
                if let Ok(settings) = manager.load_settings() {
                    state.settings = settings;
                }
            }
            state.config_manager = config_manager;

            (state, Task::none())
        })
}

fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::OpenFiles => Task::perform(
            async move {
                rfd::FileDialog::new()
                    .add_filter("PDF", &["pdf"])
                    .set_title("Select PDF file(s)")
                    .pick_files()
            },
            Message::FilesPicked,
        ),
        Message::FilesPicked(selection) => {
            if let Some(files) = selection {
                let mut new_files: Vec<PathBuf> = files
                    .into_iter()
                    .filter(|p| {
                        p.extension().map(|e| e.to_string_lossy().to_lowercase())
                            == Some("pdf".into())
                    })
                    .collect();

                new_files.retain(|f| !state.selected_files.iter().any(|e| e == f));

                for f in &new_files {
                    state.file_names.push(default_display_name(f));
                }

                state.selected_files.extend(new_files);
            }
            Task::none()
        }
        Message::Send => {
            if state.sending {
                return Task::none();
            }

            if state.selected_files.len() > 5 {
                state.status = Some(format!(
                    "Too many attachments: {} (max 5)",
                    state.selected_files.len()
                ));
                return Task::none();
            }

            if let Some(i) = state.editing_index.take() {
                commit_edit(state, i);
            }

            state.sending = true;
            state.status = Some("Sending...".into());

            let files_with_names: Vec<(PathBuf, String)> = state
                .selected_files
                .iter()
                .cloned()
                .zip(state.file_names.clone())
                .collect();

            let settings = state.settings.clone();
            Task::perform(
                async move { email::send_pdfs(files_with_names, &settings).map_err(|e| e.to_string()) },
                Message::Sent,
            )
        }
        Message::Sent(result) => {
            state.sending = false;
            state.status = Some(match result {
                Ok(c) => format!("Sent {} attachment(s)", c),
                Err(e) => format!("Error: {}", e),
            });

            Task::none()
        }
        Message::Remove(path) => {
            if let Some(idx) = state.selected_files.iter().position(|p| p == &path) {
                state.selected_files.remove(idx);
                state.file_names.remove(idx);

                if matches!(state.editing_index, Some(ei) if ei == idx) {
                    state.editing_index = None;
                }
            }

            Task::none()
        }
        Message::EditName(i) => {
            if i < state.file_names.len() {
                if let Some(prev) = state.editing_index.take() {
                    commit_edit(state, prev);
                }

                state.editing_index = Some(i);

                let mut base = state.file_names[i].clone();
                if base.len() >= 4 && base[base.len() - 4..].eq_ignore_ascii_case(".pdf") {
                    base.truncate(base.len() - 4);
                }

                state.editing_buffer = base;
            }
            Task::none()
        }
        Message::NameChanged(val) => {
            let mut v = val;
            if v.len() >= 4 && v[v.len() - 4..].eq_ignore_ascii_case(".pdf") {
                v.truncate(v.len() - 4);
            }

            state.editing_buffer = v;
            Task::none()
        }
        Message::CommitEdit => {
            if let Some(i) = state.editing_index.take() {
                commit_edit(state, i);
            }

            Task::none()
        }
        Message::CancelEdit => {
            state.editing_index = None;
            state.editing_buffer.clear();

            Task::none()
        }
        Message::OpenSettings => {
            state.show_settings = true;
            Task::none()
        }
        Message::CloseSettings => {
            state.show_settings = false;
            Task::none()
        }
        Message::SmtpHostChanged(value) => {
            state.settings.smtp_host = value;
            state.settings_changed = true;
            Task::none()
        }
        Message::SmtpPortChanged(value) => {
            state.settings.smtp_port = value;
            state.settings_changed = true;
            Task::none()
        }
        Message::FromEmailChanged(value) => {
            state.settings.from_email = value;
            state.settings_changed = true;
            Task::none()
        }
        Message::ToEmailChanged(value) => {
            state.settings.to_email = value;
            state.settings_changed = true;
            Task::none()
        }
        Message::AppPasswordChanged(value) => {
            state.settings.app_password = value;
            state.settings_changed = true;
            Task::none()
        }
        Message::SaveSettings => {
            let settings = state.settings.clone();
            Task::perform(
                async move {
                    if let Ok(manager) = config::ConfigManager::new() {
                        manager.save_settings(&settings).map_err(|e| e.to_string())
                    } else {
                        Err("Failed to create config manager".to_string())
                    }
                },
                Message::SettingsSaved,
            )
        }
        Message::SettingsSaved(result) => {
            match result {
                Ok(()) => {
                    state.status = Some("Settings saved successfully".to_string());
                    state.settings_changed = false;
                    state.show_settings = false; // Go back to main page
                }
                Err(e) => {
                    state.status = Some(format!("Error saving settings: {}", e));
                }
            }
            Task::none()
        }
    }
}

fn commit_edit(state: &mut AppState, index: usize) {
    if index < state.file_names.len() {
        let trimmed = state.editing_buffer.trim();

        if !trimmed.is_empty() {
            let mut final_name = trimmed.to_string();

            let has_pdf_extension = final_name.len() >= 4
                && final_name[final_name.len() - 4..].eq_ignore_ascii_case(".pdf");
            if !has_pdf_extension {
                final_name.push_str(".pdf");
            }

            state.file_names[index] = final_name;
        }
    }

    state.editing_buffer.clear();
}

fn default_display_name(p: &Path) -> String {
    p.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("attachment.pdf")
        .to_string()
}

fn view(state: &AppState) -> iced::Element<'_, Message> {
    app::view(state)
}
