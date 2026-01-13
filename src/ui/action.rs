use crate::schema::Action;
use std::process;

pub(super) fn call_actions(actions: &[Action], value: String) {
    for action in actions {
        match action {
            Action::PrintValueToStdOut => {
                println!("{}", value);
            }
            Action::ExitWithCode(code_str) => {
                let code: i32 = code_str.parse().expect("Invalid status code");
                process::exit(code);
            }
            Action::Shell(command) => {
                let _ = process::Command::new("sh").arg("-c").arg(command).spawn();
            }
            Action::LuaCallback(_func) => {
                // TODO: Call Lua function
                eprintln!("LuaCallback not yet implemented");
            }
        }
    }
}
