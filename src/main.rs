use mlua::prelude::*;

fn jump(_lua: &Lua, name: String) -> LuaResult<()> {
    println!("Jump, {}!", name);
    Ok(())
}

fn main() -> LuaResult<()> {
    // create a new lua state
    let lua = Lua::new();

    // load and execute a simple lua chunk
    lua.load("print('Hello from Lua!')").exec()?;

    // create a rust closure that can be called from lua    
    let hello_rust = lua.create_function(|_, name: String| {
        println!("Hello from Rust, {}!", name);
        Ok(())
    })?;
    
    // set the rust closure as a global in lua
    lua.globals().set("hello_rust", hello_rust)?;

    // execute lua code that calls the rust closure
    lua.load("hello_rust('Lua User')").exec()?;

    // create callable rust function from lua
    let jump_fn = lua.create_function(jump)?;
    lua.globals().set("jump", jump_fn)?;
    lua.load("jump('Lua')").exec()?;

    Ok(())
}
