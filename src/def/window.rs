use serde::{Deserialize, Serialize};
use super::container::Container;

#[derive(Serialize, Deserialize)]
pub struct Window {
    pub title: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub container: Container,
}
