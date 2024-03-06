use super::container::Container;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Window {
    pub title: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub container: Container,
    pub enable_vim_keys: Option<bool>,
    pub enable_esc_as_exit: Option<bool>,
}
