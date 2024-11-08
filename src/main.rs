use std::{fs, io};

use mlua::{FromLuaMulti, Function, Lua, LuaSerdeExt, Result as LuaResult, Table};
// use mlua::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    last_name: &'static str,
    first_name: &'static str,
    age: i64,
    home_owner: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct ReformatOut {
    full_name: String,
    interest_rate: f64,
    call_count: i64,
}

impl FromLuaMulti for ReformatOut {
    fn from_lua_multi(values: mlua::MultiValue, _: &Lua) -> LuaResult<Self> {
        if let Some(table) = values.get(0).unwrap().as_table() {
            Ok(ReformatOut {
                full_name: table.get::<String>("full_name")?,
                interest_rate: table.get::<f64>("interest_rate")?,
                call_count: table.get::<i64>("call_count")?,
            })
        } else {
            Err(mlua::Error::DeserializeError(
                "Invalid Return Value".to_string(),
            ))
        }
    }
}

const PEOPLE: [Person; 3] = [
    Person {
        last_name: "Vogeler",
        first_name: "Chris",
        age: 34,
        home_owner: false,
    },
    Person {
        last_name: "Vogeler",
        first_name: "Heiko",
        age: 62,
        home_owner: true,
    },
    Person {
        last_name: "Vogeler",
        first_name: "Johannes",
        age: 29,
        home_owner: false,
    },
];

fn main() -> LuaResult<()> {
    let lua = Lua::new();
    let lua_script = fs::read_to_string("lua/data.lua")?;
    let loaded_script = lua.load(lua_script);

    loaded_script.exec()?;
    let globals = lua.globals();
    let reformat: Function = globals.get("reformat")?;
    for person in PEOPLE {
        let table = lua.to_value(&person)?;
        let r = reformat.call::<ReformatOut>(table)?;
        println!("r = {:?}", r);
    }
    Ok(())
}
