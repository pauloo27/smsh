use std::collections::HashMap;

use mlua::{Function, Lua};

fn main() {
    let lua = Lua::new();
    let globals = lua.globals();

    let ffff = lua
        .create_function(|_, dict: HashMap<String, Function>| {
            println!("i have been called with {dict:?}");
            let cb = dict.get("cb").unwrap();
            cb.call::<()>(()).unwrap();
            Ok(())
        })
        .expect("Fn");

    globals.set("window", ffff).expect("ffff");

    let c = lua.load(
        r#"
        window {
            title = "bar";
            cb = function()
                print(_VERSION)
            end
        }
        "#,
    );

    c.exec().unwrap();

    /*
    let res = c
        .eval::<HashMap<String, String>>()
        .expect("lower your expectations");
    println!("{:?}", res);
    */
}
