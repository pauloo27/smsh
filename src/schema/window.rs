use super::component::Component;
use mlua::{FromLua, Lua, Value};

#[derive(Debug)]
pub struct Window {
    pub title: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub root: Component,
    pub enable_vim_keys: Option<bool>,
    pub enable_esc_as_exit: Option<bool>,
    pub present: Option<bool>,
    pub exit_on_close: Option<bool>,
}

impl FromLua for Window {
    fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
        match value {
            Value::Table(t) => {
                let title = t.get("title")?;
                let width = t.get("width").ok();
                let height = t.get("height").ok();
                let root = t.get("root")?;
                let enable_vim_keys = t.get("enable_vim_keys").ok();
                let enable_esc_as_exit = t.get("enable_esc_as_exit").ok();
                let present = t.get("present").ok();
                let exit_on_close = t.get("exit_on_close").ok();

                Ok(Window {
                    title,
                    width,
                    height,
                    root,
                    enable_vim_keys,
                    enable_esc_as_exit,
                    present,
                    exit_on_close,
                })
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Window".to_string(),
                message: Some("expected table".to_string()),
            }),
        }
    }
}
