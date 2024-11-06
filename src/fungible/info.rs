use crate::{AttributeBuilder, Cw20Token, Fungible, NativeToken, TokenKey};
use cosmwasm_std::{Addr, Attribute, CosmosMsg, Deps, StdResult, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FungibleTokenInfo {
    Cw20(Cw20Token),
    Native(NativeToken),
}

impl FungibleTokenInfo {
    pub fn cw20(address: Addr) -> Self {
        Self::Cw20(Cw20Token::new(address))
    }

    pub fn native(denom: impl Into<String>) -> Self {
        Self::Native(NativeToken::new(denom))
    }

    pub fn is_native(&self) -> bool {
        matches!(self, Self::Native(_))
    }

    pub fn is_cw20(&self) -> bool {
        matches!(self, Self::Cw20(_))
    }

    pub fn key(&self) -> TokenKey {
        match &self {
            FungibleTokenInfo::Cw20(info) => (0, info.address.as_ref().to_string()),
            FungibleTokenInfo::Native(info) => (1, info.denom.clone()),
        }
    }

    pub fn from_key(key: TokenKey) -> Self {
        let (id, data) = key;

        match id {
            0 => Self::cw20(Addr::unchecked(data)),
            _ => Self::native(data),
        }
    }
}

impl Fungible for FungibleTokenInfo {
    fn send(&self, target: impl Into<String>, amount: impl Into<Uint128>) -> StdResult<CosmosMsg> {
        match self {
            FungibleTokenInfo::Cw20(info) => info.send(target, amount),
            FungibleTokenInfo::Native(info) => info.send(target, amount),
        }
    }

    fn burn(&self, amount: impl Into<Uint128>) -> StdResult<CosmosMsg> {
        match self {
            FungibleTokenInfo::Cw20(info) => info.burn(amount),
            FungibleTokenInfo::Native(info) => info.burn(amount),
        }
    }

    fn balance(&self, address: impl Into<String>, deps: Deps) -> StdResult<Uint128> {
        match self {
            FungibleTokenInfo::Cw20(info) => info.balance(address, deps),
            FungibleTokenInfo::Native(info) => info.balance(address, deps),
        }
    }
}

impl AttributeBuilder for FungibleTokenInfo {
    fn attributes(&self) -> StdResult<Vec<Attribute>> {
        match self {
            FungibleTokenInfo::Cw20(info) => info.attributes(),
            FungibleTokenInfo::Native(info) => info.attributes(),
        }
    }
}
