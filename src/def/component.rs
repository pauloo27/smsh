use serde::{Deserialize, Serialize};

use super::action::Action;

#[derive(Serialize, Deserialize)]
pub enum ComponentType {
    Label,
    Button,
    Entry,
}

#[derive(Serialize, Deserialize)]
pub struct Component {
    pub r#type: ComponentType,
    pub text: String,
    pub tooltip: String,
    pub actions: Option<Vec<Action>>,
}
