use mlua::{FromLuaMulti, Function, Lua, LuaSerdeExt, Result as LuaResult};
use serde::Serialize;

pub struct LuaRule {
    ctx: Lua,
    reformat: Function,
}

impl LuaRule {
    pub fn new(script: &str) -> LuaResult<Self> {
        let ctx = Lua::new();
        let globals = ctx.globals();
        ctx.load(script).exec()?;
        Ok(Self {
            ctx,
            reformat: globals.get("reformat")?,
        })
    }

    pub fn call_ruleset<T, R>(&self, in_rec: &T) -> LuaResult<R>
    where
        T: Serialize,
        R: FromLuaMulti + std::fmt::Debug,
    {
        let table = self.ctx.to_value(in_rec)?;
        self.reformat.call::<R>(table)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use super::*; // Bring add function into scope

    #[test]
    fn test_ruleset_call() {
        let lua = LuaRule::new(LUA_SCRIPT_1).unwrap();

        let ruleset_result: InterestRate = lua.call_ruleset(&PEOPLE[0]).unwrap();
        assert_eq!(ruleset_result.interest_rate, 0.48);
        assert_eq!(ruleset_result.full_name, "Chris Vogeler");

        let ruleset_result: InterestRate = lua.call_ruleset(&PEOPLE[2]).unwrap();
        assert_eq!(ruleset_result.interest_rate, 0.48);
        assert_eq!(ruleset_result.full_name, "Johannes Vogeler");
    }

    #[test]
    #[should_panic]
    fn test_ruleset_call_failure() {
        let lua = LuaRule::new(LUA_SCRIPT_1).unwrap();
        lua.call_ruleset::<_, InterestRate>(&PEOPLE[1]).unwrap();
    }

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

    const LUA_SCRIPT_1: &'static str = r#"
call_count = 0

function reformat (person)
    call_count = call_count + 1
    local interest_rate = 0.6
    if person.home_owner then
        interest_rate = 0.1
    end
    if person.age < 22 then
        interest_rate = 1
    end
    if person.age >=22 and person.age < 60 then
        interest_rate = interest_rate * .8
    end
    if person.age >= 60 and person.age < 75 then
        interest_rate = interest_rate * 1.3
        error("Too old person")
    end
    if person.age >=75 then
        interest_rate = .8
    end
    return {
        full_name = person.first_name .. " " .. person.last_name,
        interest_rate = interest_rate,
        call_count = call_count,
    }
end
    "#;
}
