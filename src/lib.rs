mod cw20;
mod info;
mod native;
mod token;

pub use crate::cw20::Cw20Token;
pub use crate::info::{TokenInfo, TokenKey};
pub use crate::native::NativeToken;
pub use crate::token::Token;
use cosmwasm_std::{Addr, Attribute, CosmosMsg, Deps, StdResult, Uint128};

pub trait GenericToken {
    fn send(&self, target: &Addr, amount: Uint128) -> StdResult<CosmosMsg>;
    fn burn(&self, amount: Uint128) -> StdResult<CosmosMsg>;
    fn balance(&self, address: &Addr, deps: Deps) -> StdResult<Uint128>;
    fn attributes(&self) -> Vec<Attribute>;
    // TODO: add grant function and disclaimer that CW20 granting is the one that works on smart contracts
    // TODO:
}
