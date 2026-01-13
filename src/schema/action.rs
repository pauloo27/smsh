use mlua::{FromLua, Function, Lua, Value};

#[derive(Debug, Clone)]
pub struct Action {
    pub callback: Function,
}

impl FromLua for Action {
    fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
        match value {
            Value::Function(f) => Ok(Action { callback: f }),
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Action".to_string(),
                message: Some("expected function".to_string()),
            }),
        }
    }
}
