use crate::{Cw20Token, GenericToken, NativeToken};
use cosmwasm_std::{Addr, Attribute, CosmosMsg, Deps, StdResult, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TokenInfo {
    Cw20(Cw20Token),
    Native(NativeToken),
}

pub type TokenKey = (u8, String);

impl TokenInfo {
    pub fn cw20(address: Addr) -> Self {
        Self::Cw20(Cw20Token::new(address))
    }

    pub fn native(denom: String) -> Self {
        Self::Native(NativeToken::new(denom))
    }

    pub fn key(&self) -> TokenKey {
        match &self {
            TokenInfo::Cw20(info) => (0, info.address.as_ref().to_string()),
            TokenInfo::Native(info) => (1, info.denom.clone()),
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

impl GenericToken for TokenInfo {
    fn send(&self, target: &Addr, amount: Uint128) -> StdResult<CosmosMsg> {
        match self {
            TokenInfo::Cw20(info) => info.send(target, amount),
            TokenInfo::Native(info) => info.send(target, amount),
        }
    }

    fn burn(&self, amount: Uint128) -> StdResult<CosmosMsg> {
        match self {
            TokenInfo::Cw20(info) => info.burn(amount),
            TokenInfo::Native(info) => info.burn(amount),
        }
    }

    fn balance(&self, address: &Addr, deps: Deps) -> StdResult<Uint128> {
        match self {
            TokenInfo::Cw20(info) => info.balance(address, deps),
            TokenInfo::Native(info) => info.balance(address, deps),
        }
    }

    fn attributes(&self) -> Vec<Attribute> {
        match self {
            TokenInfo::Cw20(info) => info.attributes(),
            TokenInfo::Native(info) => info.attributes(),
        }
    }
}
