use super::component::Component;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ContainerOrientation {
    Vertical,
    Horizontal,
}

#[derive(Serialize, Deserialize)]
pub struct Container {
    pub orientation: ContainerOrientation,
    pub children: Vec<Component>,
}
