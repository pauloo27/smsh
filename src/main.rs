use crate::ui::UI;
use std::path::PathBuf;

mod lua;
mod schema;
mod ui;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <lua_path> [css_path]", args[0]);
        std::process::exit(1);
    }

    let lua_path = &args[1];
    let css_path = args.get(2).map(PathBuf::from);

    let ui = UI::new(css_path);

    let lua_env = lua::LuaEnv::new(&ui).expect("Failed to create lua env");
    lua_env.run_file(lua_path).expect("Failed to run lua file");

    ui.run();
}
