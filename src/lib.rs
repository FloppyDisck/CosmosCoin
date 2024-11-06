pub mod fungible;
pub mod non_fungible;

use cosmwasm_std::{
    to_json_binary, to_json_string, Attribute, CosmosMsg, Response, StdResult, WasmMsg,
};
pub use fungible::*;
pub use non_fungible::*;
use serde::Serialize;

pub type TokenKey = (u8, String);

pub(crate) fn execute_wasm<T: Serialize + ?Sized>(
    contract: impl Into<String>,
    msg: &T,
) -> StdResult<CosmosMsg> {
    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: contract.into(),
        msg: to_json_binary(msg)?,
        funds: vec![],
    }))
}

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
