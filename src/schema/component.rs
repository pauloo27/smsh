use super::action::Action;
use mlua::{FromLua, Lua, Value};

#[derive(Debug, Clone)]
pub enum Align {
    Start,
    Center,
    End,
    Fill,
}

impl FromLua for Align {
    fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
        match value {
            Value::String(s) => {
                let str_val = s.to_str()?.to_string();
                match str_val.as_str() {
                    "start" => Ok(Align::Start),
                    "center" => Ok(Align::Center),
                    "end" => Ok(Align::End),
                    "fill" => Ok(Align::Fill),
                    _ => Err(mlua::Error::FromLuaConversionError {
                        from: "string",
                        to: "Align".to_string(),
                        message: Some(format!("unknown alignment: {}", str_val)),
                    }),
                }
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Align".to_string(),
                message: Some("expected string".to_string()),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ContainerOrientation {
    Vertical,
    Horizontal,
}

impl FromLua for ContainerOrientation {
    fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
        match value {
            Value::String(s) => {
                let str_val = s.to_str()?.to_string();
                match str_val.as_str() {
                    "vertical" => Ok(ContainerOrientation::Vertical),
                    "horizontal" => Ok(ContainerOrientation::Horizontal),
                    _ => Err(mlua::Error::FromLuaConversionError {
                        from: "string",
                        to: "ContainerOrientation".to_string(),
                        message: Some(format!("unknown orientation: {}", str_val)),
                    }),
                }
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "ContainerOrientation".to_string(),
                message: Some("expected string".to_string()),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContainerData {
    pub orientation: ContainerOrientation,
    pub halign: Option<Align>,
    pub valign: Option<Align>,
}

impl FromLua for ContainerData {
    fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
        match value {
            Value::Table(t) => {
                let orientation: ContainerOrientation = t.get("orientation")?;
                let halign: Option<Align> = t.get("halign").ok();
                let valign: Option<Align> = t.get("valign").ok();
                Ok(ContainerData {
                    orientation,
                    halign,
                    valign,
                })
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "ContainerData".to_string(),
                message: Some("expected table".to_string()),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ToggleButtonData {
    pub text: String,
    pub tooltip: Option<String>,
    pub active: bool,
    pub action: Option<Action>,
}

impl FromLua for ToggleButtonData {
    fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
        match value {
            Value::Table(t) => {
                let text: String = t.get("text")?;
                let tooltip: Option<String> = t.get("tooltip").ok();
                let active: bool = t.get("active").unwrap_or(false);
                let action: Option<Action> = t.get("action").ok();
                Ok(ToggleButtonData {
                    text,
                    tooltip,
                    active,
                    action,
                })
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "ToggleButtonData".to_string(),
                message: Some("expected table".to_string()),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Component {
    Label {
        text: String,
        tooltip: Option<String>,
    },
    Button {
        text: String,
        tooltip: Option<String>,
        action: Option<Action>,
    },
    Entry {
        text: String,
        tooltip: Option<String>,
        action: Option<Action>,
    },
    ToggleButton(ToggleButtonData),
    ToggleButtonGroup {
        container: ContainerData,
        buttons: Vec<ToggleButtonData>,
    },
    Container {
        container: ContainerData,
        children: Vec<Component>,
    },
}

impl FromLua for Component {
    fn from_lua(value: Value, lua: &Lua) -> mlua::Result<Self> {
        match value {
            Value::Table(t) => {
                let type_str: String = t.get("type")?;

                match type_str.as_str() {
                    "label" => {
                        let text: String = t.get("text")?;
                        let tooltip: Option<String> = t.get("tooltip").ok();
                        Ok(Component::Label { text, tooltip })
                    }
                    "button" => {
                        let text: String = t.get("text")?;
                        let tooltip: Option<String> = t.get("tooltip").ok();
                        let action: Option<Action> = t.get("action").ok();
                        Ok(Component::Button {
                            text,
                            tooltip,
                            action,
                        })
                    }
                    "entry" => {
                        let text: String = t.get("text")?;
                        let tooltip: Option<String> = t.get("tooltip").ok();
                        let action: Option<Action> = t.get("action").ok();
                        Ok(Component::Entry {
                            text,
                            tooltip,
                            action,
                        })
                    }
                    "togglebutton" => {
                        let data = ToggleButtonData::from_lua(Value::Table(t.clone()), lua)?;
                        Ok(Component::ToggleButton(data))
                    }
                    "togglebuttongroup" => {
                        let container = ContainerData::from_lua(Value::Table(t.clone()), lua)?;
                        let buttons: Vec<ToggleButtonData> = t.get("buttons")?;
                        Ok(Component::ToggleButtonGroup { container, buttons })
                    }
                    "container" => {
                        let container = ContainerData::from_lua(Value::Table(t.clone()), lua)?;
                        let children: Vec<Component> = t.get("children")?;
                        Ok(Component::Container {
                            container,
                            children,
                        })
                    }
                    _ => Err(mlua::Error::FromLuaConversionError {
                        from: "string",
                        to: "Component".to_string(),
                        message: Some(format!("unknown component type: {}", type_str)),
                    }),
                }
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Component".to_string(),
                message: Some("expected table".to_string()),
            }),
        }
    }
}
