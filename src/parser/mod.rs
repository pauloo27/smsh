use super::def::window::Window;
use anyhow::Result as AnyResult;
use std::fs;
use std::path::PathBuf;

pub fn load_window_from_file(path: PathBuf) -> AnyResult<Window> {
    let file_content = fs::read_to_string(path)?;
    let win: Window = serde_json::from_str(file_content.as_str())?;

    Ok(win)
}
