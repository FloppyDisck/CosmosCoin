use crate::non_fungible::cw721::Cw721Token;
use crate::non_fungible::NonFungible;
use crate::{AttributeBuilder, TokenKey};
use cosmwasm_std::{Addr, Attribute, CosmosMsg, Deps, StdResult};
use cw721::OwnerOfResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NonFungibleTokenInfo {
    Base(Cw721Token),
}

impl NonFungibleTokenInfo {
    pub fn base(address: Addr) -> Self {
        Self::Base(Cw721Token::new(address))
    }

    pub fn is_base(&self) -> bool {
        matches!(self, Self::Base(_))
    }

    pub fn key(&self) -> TokenKey {
        match &self {
            Self::Base(info) => (0, info.address.to_string()),
        }
    }

    pub fn from_key(key: TokenKey) -> Self {
        let (id, data) = key;

        match id {
            _ => Self::base(Addr::unchecked(data)),
        }
    }
}

impl NonFungible for NonFungibleTokenInfo {
    fn send(&self, target: impl Into<String>, token: impl Into<String>) -> StdResult<CosmosMsg> {
        match self {
            Self::Base(cw721) => cw721.send(target, token),
        }
    }

    fn burn(&self, token: impl Into<String>) -> StdResult<CosmosMsg> {
        match self {
            Self::Base(cw721) => cw721.burn(token),
        }
    }

    fn owner_of(&self, token: impl Into<String>, deps: Deps) -> StdResult<OwnerOfResponse> {
        match self {
            Self::Base(cw721) => cw721.owner_of(token, deps),
        }
    }

    fn total_tokens(&self, deps: Deps) -> StdResult<u64> {
        match self {
            Self::Base(cw721) => cw721.total_tokens(deps),
        }
    }

    fn tokens(
        &self,
        owner: Option<String>,
        start_after: Option<String>,
        limit: Option<u32>,
        deps: Deps,
    ) -> StdResult<Vec<String>> {
        match self {
            Self::Base(cw721) => cw721.tokens(owner, start_after, limit, deps),
        }
    }
}

impl AttributeBuilder for NonFungibleTokenInfo {
    fn attributes(&self) -> StdResult<Vec<Attribute>> {
        match self {
            Self::Base(cw721) => cw721.attributes(),
        }
    }
}
