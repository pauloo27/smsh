use super::action::Action;
use mlua::{FromLua, Lua, Value};

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
    Container {
        orientation: ContainerOrientation,
        children: Vec<Component>,
    },
}

impl FromLua for Component {
    fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
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
                    "container" => {
                        let orientation: ContainerOrientation = t.get("orientation")?;
                        let children: Vec<Component> = t.get("children")?;
                        Ok(Component::Container {
                            orientation,
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
