use mlua::{FromLua, Function, Lua, Value};

#[derive(Debug, Clone)]
pub enum Action {
    Shell(String),
    ExitWithCode(String),
    PrintValueToStdOut,
    LuaCallback(Function),
}

impl FromLua for Action {
    fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
        match value {
            Value::Table(t) => {
                let type_str: String = t.get("type")?;

                match type_str.as_str() {
                    "shell" => {
                        let value: String = t.get("value")?;
                        Ok(Action::Shell(value))
                    }
                    "exit_with_code" => {
                        let value: String = t.get("value")?;
                        Ok(Action::ExitWithCode(value))
                    }
                    "print_value_to_stdout" => Ok(Action::PrintValueToStdOut),
                    "lua_callback" => {
                        let func: Function = t.get("value")?;
                        Ok(Action::LuaCallback(func))
                    }
                    _ => Err(mlua::Error::FromLuaConversionError {
                        from: "string",
                        to: "Action".to_string(),
                        message: Some(format!("unknown action type: {}", type_str)),
                    }),
                }
            }
            _ => Err(mlua::Error::FromLuaConversionError {
                from: value.type_name(),
                to: "Action".to_string(),
                message: Some("expected table".to_string()),
            }),
        }
    }
}
