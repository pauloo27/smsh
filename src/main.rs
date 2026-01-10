use mlua::Lua;

fn main() {
    let lua = Lua::new();
    let c = lua.load(
        r#"
        return _VERSION
        "#,
    );
    let res = c.eval::<String>().expect("lower your expectations");
    println!("{}", res);
}
