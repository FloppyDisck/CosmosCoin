mod cw721;
mod info;
mod token;

pub use cw721::*;
pub use info::*;
pub use token::*;

use ::cw721::OwnerOfResponse;
use cosmwasm_std::{CosmosMsg, Deps, StdResult};

pub trait NonFungible {
    type Extension;
    fn send(&self, target: impl Into<String>, token: impl Into<String>) -> StdResult<CosmosMsg>;
    fn burn(&self, token: impl Into<String>) -> StdResult<CosmosMsg>;
    fn mint(
        &self,
        token: impl Into<String>,
        owner: impl Into<String>,
        token_uri: Option<String>,
        extension: Self::Extension,
    ) -> StdResult<CosmosMsg>;
    fn owner_of(&self, token: impl Into<String>, deps: Deps) -> StdResult<OwnerOfResponse>;
    fn total_tokens(&self, deps: Deps) -> StdResult<u64>;
    fn tokens(
        &self,
        owner: Option<String>,
        start_after: Option<String>,
        limit: Option<u32>,
        deps: Deps,
    ) -> StdResult<Vec<String>>;
}
