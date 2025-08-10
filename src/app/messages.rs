use std::{path::PathBuf, result};

#[derive(Debug, Clone)]
pub enum Message {
    OpenFiles,
    FilesPicked(Option<Vec<PathBuf>>),
    FilesDropped(Vec<PathBuf>),
    Send,
    Sent(result::Result<usize, String>),
    Remove(PathBuf),
    EditName(usize),
    NameChanged(String),
    CommitEdit,
    CancelEdit,
    OpenSettings,
    CloseSettings,
    SmtpHostChanged(String),
    SmtpPortChanged(String),
    FromEmailChanged(String),
    ToEmailChanged(String),
    AppPasswordChanged(String),
    SaveSettings,
    SettingsSaved(result::Result<(), String>),
}
