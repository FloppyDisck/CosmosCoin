pub mod fungible;
pub mod non_fungible;

use cosmwasm_std::{to_json_string, Attribute, Response, StdResult};
pub use fungible::*;
pub use non_fungible::*;
use serde::Serialize;

// TODO: implement custom JsonSchema

// TODO: implement batch send for native token
// TODO: implement Auth for native
// TODO: implement Perms for Cw20
// TODO: implement Send from Cw20 that allows sending from a message
// TODO: implement mint for Cw20

pub type TokenKey = (u8, String);

pub trait AttributeBuilder: Serialize {
    fn write_json_attributes(&self, key: &str, resp: &mut Response) -> StdResult<()> {
        resp.attributes.push(self.json_attributes(key)?);
        Ok(())
    }
    fn write_attributes(&self, resp: &mut Response) -> StdResult<()> {
        resp.attributes.append(&mut self.attributes()?);
        Ok(())
    }
    fn json_attributes(&self, key: &str) -> StdResult<Attribute> {
        let value = to_json_string(self)?;
        Ok(Attribute::new(key, value))
    }
    fn attributes(&self) -> StdResult<Vec<Attribute>>;
}
