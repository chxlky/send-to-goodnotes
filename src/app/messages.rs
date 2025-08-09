#[derive(Debug, Clone)]
pub enum Message {
    OpenFiles,
    FilesPicked(Option<Vec<std::path::PathBuf>>),
    Send,
    Sent(std::result::Result<usize, String>),
    Remove(std::path::PathBuf),
    EditName(usize),
    NameChanged(String),
    CommitEdit,
    CancelEdit,
}
