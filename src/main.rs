use mlua::prelude::*;
use std::fs;
use std::path::Path;

fn jump(_lua: &Lua, name: String) -> LuaResult<()> {
    println!("Jump, {}!", name);
    Ok(())
}

fn change_weapon(_lua: &Lua, name: String) -> LuaResult<()> {
    println!("Weapon Changed, {}!", name);
    Ok(())
}

fn run_lua_file(lua: &Lua, file_path: &Path) -> LuaResult<()> {
    match fs::read_to_string(file_path) {
        Ok(lua_code) => {
            println!("Executing Lua file: {}", file_path.display());
            lua.load(&lua_code).exec()?;
            Ok(())
        }
        Err(e) => {
            eprintln!("Error reading Lua file '{}': {}", file_path.display(), e);
            Err(LuaError::RuntimeError(format!("Failed to read Lua file: {}", e)))
        }
    }
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

    let change_weapon_fn = lua.create_function(change_weapon)?;
    lua.globals().set("change_weapon", change_weapon_fn)?;

    // run lua file, path relative to project root
    let lua_file_path = Path::new("src/event.lua");
    
    run_lua_file(&lua, lua_file_path)?;

    Ok(())
}
