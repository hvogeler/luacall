use lua::LuaRule;
use mlua::{Error, FromLuaMulti, Lua, Result as LuaResult};
use serde::{Deserialize, Serialize};
use std::fs;

mod lua;

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    last_name: &'static str,
    first_name: &'static str,
    age: i64,
    home_owner: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct InterestRate {
    full_name: String,
    interest_rate: f64,
    call_count: i64,
}

impl FromLuaMulti for InterestRate {
    fn from_lua_multi(values: mlua::MultiValue, _: &Lua) -> LuaResult<Self> {
        if let Some(table) = values.get(0).unwrap().as_table() {
            Ok(InterestRate {
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
    let lua_script = fs::read_to_string("lua/data.lua")?;
    let lua = LuaRule::new(&lua_script)?;

    for _ in 0..10000 {
        for person in PEOPLE {
            // let table = lua.to_value(&person)?;
            match lua.call_ruleset::<_, InterestRate>(&person) {
                Ok(r) => println!("r = {:?}", r),
                Err(e) => match e {
                    Error::RuntimeError(re) => {
                        println!("ERROR - Rule Failed: {}", re);
                    }
                    _ => println!("ERROR - unknown: {}", e),
                },
            }
        }
    }
    Ok(())
}
