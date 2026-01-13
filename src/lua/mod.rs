use anyhow::Result as AnyResult;
use mlua::Lua;
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;

use crate::{
    schema::{UICommands, Window},
    ui::UI,
};

pub struct LuaEnv {
    lua: Lua,
    sender: Sender<UICommands>,
}

impl LuaEnv {
    pub fn new(ui: &UI) -> AnyResult<Self> {
        let lua = Lua::new();
        let globals = lua.globals();

        let sender = ui.get_sender();
        let sender_clone = sender.clone();

        let window_fn = lua
            .create_function(move |_, window: Window| {
                sender
                    .send(UICommands::NewWindow(window))
                    .map_err(|_| mlua::Error::runtime("Failed to send window to UI thread"))?;
                Ok(())
            })
            .map_err(|e| anyhow::anyhow!("Failed to create window function: {}", e))?;

        let load_css_fn = lua
            .create_function(move |_, path: String| {
                let expanded_path = shellexpand::full(&path)
                    .map_err(|e| mlua::Error::runtime(format!("Failed to expand path: {}", e)))?;
                sender_clone
                    .send(UICommands::LoadCSS(PathBuf::from(expanded_path.as_ref())))
                    .map_err(|_| {
                        mlua::Error::runtime("Failed to send load_css command to UI thread")
                    })?;
                Ok(())
            })
            .map_err(|e| anyhow::anyhow!("Failed to create load_css function: {}", e))?;

        globals
            .set("window", window_fn)
            .map_err(|e| anyhow::anyhow!("Failed to set window global: {}", e))?;

        globals
            .set("load_css", load_css_fn)
            .map_err(|e| anyhow::anyhow!("Failed to set load_css global: {}", e))?;

        Ok(Self {
            lua,
            sender: ui.get_sender(),
        })
    }

    pub fn run_file<P: AsRef<Path>>(&self, path: P) -> AnyResult<()> {
        self.lua
            .load(&std::fs::read_to_string(path.as_ref())?)
            .exec()
            .map_err(|e| anyhow::anyhow!("Failed to execute Lua file: {}", e))?;

        Ok(())
    }
}
