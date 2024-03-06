use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ActionType {
    Shell,
    ExitWithCode,
    PrintValueToStdOut,
}

#[derive(Serialize, Deserialize)]
pub struct Action {
    pub r#type: ActionType,
    pub value: String,
}
