use mlua::prelude::*;

fn main() -> LuaResult<()> {
    // create a new lua state
    let lua = Lua::new();

    // load and execute a simple lua chunk
    lua.load("print('Hello from Lua!')").exec()?;

    Ok(())
}
