use cosmwasm_std::{CosmosMsg, Deps, StdResult, Uint128};

mod cw20;
mod info;
mod native;
mod token;

pub use cw20::*;
pub use info::*;
pub use native::*;
pub use token::*;

pub trait Fungible {
    fn send(&self, target: impl Into<String>, amount: impl Into<Uint128>) -> StdResult<CosmosMsg>;
    fn burn(&self, amount: impl Into<Uint128>) -> StdResult<CosmosMsg>;
    fn balance(&self, address: impl Into<String>, deps: Deps) -> StdResult<Uint128>;
}
