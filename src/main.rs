use mlua::prelude::*;

fn main() -> LuaResult<()> {
    // create a new lua state
    let lua = Lua::new();

    // load and execute a simple lua chunk
    lua.load("print('Hello from Lua!')").exec()?;

    // create a rust function that can be called from lua    
    let hello_rust = lua.create_function(|_, name: String| {
        println!("Hello from Rust, {}!", name);
        Ok(())
    })?;
    
    // set the rust function as a global in lua
    lua.globals().set("hello_rust", hello_rust)?;

    // execute lua code that calls the rust function
    lua.load("hello_rust('Lua User')").exec()?;

    Ok(())
}
