mod cw721;
mod info;
mod token;

use ::cw721::OwnerOfResponse;
use cosmwasm_std::{CosmosMsg, Deps, StdResult};

pub trait NonFungible {
    fn send(&self, target: impl Into<String>, token: impl Into<String>) -> StdResult<CosmosMsg>;
    fn burn(&self, token: impl Into<String>) -> StdResult<CosmosMsg>;
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
