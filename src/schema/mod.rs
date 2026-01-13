mod action;
mod component;
mod window;

pub use component::{Component, ContainerOrientation};
pub use window::Window;

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum UICommands {
    NewWindow(Window),
    LoadCSS(PathBuf),
}
