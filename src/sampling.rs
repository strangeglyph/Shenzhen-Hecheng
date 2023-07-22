use rlua::{Lua, StdLib, Value as LuaValue, ToLua, FromLua, Result as LuaResult, Context, Error};

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
            _ => Err(Error::FromLuaConversionError {
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
            _ => Err(Error::FromLuaConversionError {
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

trait Sampler<Id, Letter> {
    fn next(&mut self) -> Vec<Channel<Id, Letter>>;
}

struct LuaPuzzle {
    content: String
}

impl LuaPuzzle {

}

fn lua_context() -> LuaResult<LuaError> {
    // minimal set of lua modules
    let lua = Lua::new_with(StdLib::BASE | StdLib::TABLE | StdLib::MATH);

    lua.context(|lua_ctx| {
        let globals = lua_ctx.globals();

        globals.set("TYPE_XBUS", PortType::XBUS)?;
        globals.set("TYPE_SIMPLE", PortType::SIMPLE)?;
        globals.set("DIR_INPUT", Owner::ENVIRONMENT)?;
        globals.set("DIR_OUTPUT", Owner::AGENT)?;
    })?;

    Ok(lua)
}


impl Sampler<u8, Value> for LuaPuzzle {
    fn next(&mut self) -> Vec<Channel<u8, Value>> {
        todo!()
    }
}