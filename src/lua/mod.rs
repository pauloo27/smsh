use anyhow::Result as AnyResult;
use mlua::Lua;
use std::path::{Path, PathBuf};

use crate::{
    schema::{UICommands, Window},
    ui::UI,
};

pub struct LuaEnv {
    lua: Lua,
}

fn get_config_dir() -> PathBuf {
    if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg_config).join("smsh")
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".config").join("smsh")
    } else {
        PathBuf::from(".config/smsh")
    }
}

impl LuaEnv {
    pub fn new(ui: &UI) -> AnyResult<Self> {
        let lua = Lua::new();
        let globals = lua.globals();

        // Set up Lua package.path to include config directory
        let config_dir = get_config_dir();
        let config_dir_str = config_dir.to_string_lossy();
        let package_path = format!("{}/?.lua;{}/?/init.lua;", config_dir_str, config_dir_str);
        lua.load(format!("package.path = '{}' .. package.path", package_path))
            .exec()
            .map_err(|e| anyhow::anyhow!("Failed to set package.path: {}", e))?;

        let sender = ui.get_sender();
        let sender_window = sender.clone();
        let sender_css = sender.clone();

        let window_fn = lua
            .create_function(move |_, window: Window| {
                sender_window
                    .send(UICommands::NewWindow(window))
                    .map_err(|_| mlua::Error::runtime("Failed to send window to UI thread"))?;
                Ok(())
            })
            .map_err(|e| anyhow::anyhow!("Failed to create window function: {}", e))?;

        let config_dir_clone = config_dir.clone();
        let load_css_fn = lua
            .create_function(move |_, path: String| {
                let expanded_path = shellexpand::full(&path)
                    .map_err(|e| mlua::Error::runtime(format!("Failed to expand path: {}", e)))?;

                let path_buf = PathBuf::from(expanded_path.as_ref());
                let final_path = if path_buf.is_absolute() {
                    path_buf
                } else {
                    config_dir_clone.join(path_buf)
                };

                sender_css
                    .send(UICommands::LoadCSS(final_path))
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
