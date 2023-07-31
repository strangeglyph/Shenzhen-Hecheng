use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::str;

use rlua::{Function, StdLib};
use rlua::Lua;
use rlua::Value as LuaValue;
use rlua::ToLua;
use rlua::FromLua;
use rlua::Result as LuaResult;
use rlua::Context;
use rlua::Error as LuaError;

use anyhow::Result;
use anyhow::Context as AnyhowContext;

enum PortType {
    SIMPLE,
    XBUS,
}

impl<'lua> ToLua<'lua> for PortType {
    fn to_lua(self, lua: Context<'lua>) -> LuaResult<LuaValue<'lua>> {
        match self {
            PortType::SIMPLE => Ok(LuaValue::Boolean(true)),
            PortType::XBUS => Ok(LuaValue::Boolean(false))
        }
    }
}

impl<'lua> FromLua<'lua> for PortType {
    fn from_lua(lua_value: LuaValue<'lua>, lua: Context<'lua>) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Boolean(true) => Ok(PortType::SIMPLE),
            LuaValue::Boolean(false) => Ok(PortType::XBUS),
            _ => Err(LuaError::FromLuaConversionError {
                from: "non-bool",
                to: "PortType",
                message: Some("Conversion to PortType is only supported for bools (true for simple, false for xbus)".into()),
            })
        }
    }
}

enum Value {
    SIMPLE(u8),
    XBUS(Vec<i16>),
}

impl<'lua> FromLua<'lua> for Value {
    fn from_lua(lua_value: LuaValue<'lua>, lua: Context<'lua>) -> LuaResult<Self> {
        match lua_value {
            v@LuaValue::Integer(_) => {
                let x = u8::from_lua(v, lua)?;
                if x < 0 || x > 100 {
                    return Err(LuaError::FromLuaConversionError {
                        from: "integer",
                        to: "simple value",
                        message: Some("Out-of-bounds conversion from integer to simple value (valid range: [0,100]".to_string())
                    })
                }
                Ok(Value::SIMPLE(x))
            },
            v@LuaValue::Table(_) => {
                let arr = Vec::<i16>::from_lua(v, lua)?;
                if arr.iter().any(|&x| x < -999_i16 || x > 999_i16) {
                    return Err(LuaError::FromLuaConversionError {
                        from: "integer",
                        to: "xbus value",
                        message: Some("Out-of-bounds conversion from integer to xbus value (valid range: [-999, 999]".to_string())
                    })
                }
                Ok(Value::XBUS(arr))
            }
            _ => Err(LuaError::FromLuaConversionError {
                from: "non-integer non-array type",
                to: "value",
                message: Some("Conversion to Value is only supporter for Integers (for Simple) and Array of Integers (for Xbus)".to_string())
            })
        }
    }
}

enum Owner {
    AGENT,
    ENVIRONMENT,
}


impl<'lua> ToLua<'lua> for Owner {
    fn to_lua(self, lua: Context<'lua>) -> LuaResult<LuaValue<'lua>> {
        match self {
            Owner::ENVIRONMENT => Ok(LuaValue::Boolean(true)),
            Owner::AGENT => Ok(LuaValue::Boolean(false))
        }
    }
}

impl<'lua> FromLua<'lua> for Owner {
    fn from_lua(lua_value: LuaValue<'lua>, lua: Context<'lua>) -> LuaResult<Self> {
        match lua_value {
            LuaValue::Boolean(true) => Ok(Owner::ENVIRONMENT),
            LuaValue::Boolean(false) => Ok(Owner::AGENT),
            _ => Err(LuaError::FromLuaConversionError {
                from: "non-bool",
                to: "Owner",
                message: Some("Conversion to Owner is only supported for bools (true for input/environment, false for output/agent)".into()),
            })
        }
    }
}


struct Channel<Id, Letter> {
    id: Id,
    owner: Owner,
    values: Vec<Letter>,
}

struct Sample<Id, Letter> {
    channels: Vec<Channel<Id, Letter>>,
}

trait Sampler<Id, Letter> {
    fn next(&mut self) -> Result<Sample<Id, Letter>>;
    fn sample(&mut self, n: usize) -> Result<Vec<Sample<Id, Letter>>>;
    fn sample_into(&mut self, n: usize, buffer: &mut Vec<Sample<Id, Letter>>) -> Result<()>;
}

struct LuaPuzzle {
    path: String,
    name: String,
    lua: Lua
}

impl LuaPuzzle {
    fn open<P: AsRef<Path> + Display>(path: P) -> Result<LuaPuzzle> {
        let content_as_bytes = fs::read(path).with_context(|| format!("Failed to read lua file at {}", path))?;
        let content_as_utf8 = str::from_utf8(&content_as_bytes).with_context(|| format!("Failed to read lua file at {}", path))?;

        let lua = Lua::new_with(StdLib::BASE | StdLib::TABLE | StdLib::MATH);

        lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();

            globals.set("TYPE_XBUS", PortType::XBUS)?;
            globals.set("TYPE_SIMPLE", PortType::SIMPLE)?;
            globals.set("DIR_INPUT", Owner::ENVIRONMENT)?;
            globals.set("DIR_OUTPUT", Owner::AGENT)?;

            Ok(())
        }).with_context(|| format!("Failed to initialize lua state for file {}", path))?;

        lua.context(|lua_ctx| {
            lua_ctx
                .load(content_as_utf8)
                .set_name(&format!("Lua file <{}>", path))?
                .exec()
        }).with_context(|| format!("Failed to load lua file from {}", path))?;

        let name = lua.context(|lua_ctx| {
            let globals = lua_ctx.globals();

            globals.get::<_, Function>("get_name")?.call::<_, String>(())
        }).with_context(|| format!("Failed to get name of lua file from {}", path))?;

        Ok(LuaPuzzle {
            path: format!("{}", path),
            name,
            lua
        })
    }
}

impl Sampler<u8, Value> for LuaPuzzle {
    fn next(&mut self) -> Result<Sample<u8, Value>> {
        Ok(self.sample(1)?.pop().unwrap())
    }

    fn sample(&mut self, n: usize) -> Result<Vec<Sample<u8, Value>>> {
        let mut buffer = Vec::with_capacity(n);
        self.sample_into(n, &mut buffer)?;
        Ok(buffer)
    }

    fn sample_into(&mut self, n: usize, buffer: &mut Vec<Sample<u8, Value>>) -> Result<()> {

        lua.with_context(|lua_ctx| {
            let mut channels = Vec::with_capacity(10);
            let create_terminal = |name: &str, id: &str, port_type: PortType, owner: Owner, values: Vec<Value>| {
                let id = id.parse::<u8>().with_context(|| format!("invalid id {}", id))?;
                ensure!(channels.iter().all(|chan| chan.id != id), format!("port {} redefined", id));
                ensure!(values.iter().all(|val| match port_type | val {
                    PortType::SIMPLE | Value::SIMPLE(_) => true,
                    PortType::XBUS | Value::XBUS(_) => true,
                    _ => false
                }), format!("value mismatch for port {}, defined as {}", id, port_type));
                channels.push(values);

                Ok(())
            };

            let globals = lua_ctx.globals();
            globals.create_function()

            Ok(())
        })
    }
}