use std::path::PathBuf;

#[derive(Default)]
pub struct AppState {
    pub selected_files: Vec<PathBuf>,
    pub file_names: Vec<String>,
    pub status: Option<String>,
    pub sending: bool,
    pub editing_index: Option<usize>,
    pub editing_buffer: String,
}
