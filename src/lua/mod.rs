use anyhow::Result as AnyResult;
use mlua::Lua;
use std::path::Path;

use crate::{schema::Window, ui::UI};

pub struct LuaEnv {
    lua: Lua,
}

impl LuaEnv {
    pub fn new(ui: &UI) -> AnyResult<Self> {
        let lua = Lua::new();
        let globals = lua.globals();

        let sender = ui.get_sender();

        let window_fn = lua
            .create_function(move |_, window: Window| {
                sender
                    .send(window)
                    .map_err(|_| mlua::Error::runtime("Failed to send window to UI thread"))?;
                Ok(())
            })
            .map_err(|e| anyhow::anyhow!("Failed to create window function: {}", e))?;

        globals
            .set("window", window_fn)
            .map_err(|e| anyhow::anyhow!("Failed to set window global: {}", e))?;

        Ok(Self { lua })
    }

    pub fn run_file<P: AsRef<Path>>(&self, path: P) -> AnyResult<()> {
        self.lua
            .load(&std::fs::read_to_string(path.as_ref())?)
            .exec()
            .map_err(|e| anyhow::anyhow!("Failed to execute Lua file: {}", e))?;

        Ok(())
    }
}
