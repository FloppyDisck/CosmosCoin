use crate::{GenericToken, TokenInfo, TokenKey};
use cosmwasm_std::{Addr, Attribute, Coin, CosmosMsg, StdResult, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Token {
    pub info: TokenInfo,
    pub amount: Uint128,
}

impl From<Coin> for Token {
    fn from(value: Coin) -> Self {
        Self {
            info: TokenInfo::native(value.denom),
            amount: value.amount,
        }
    }
}

impl Token {
    pub fn new(info: TokenInfo, amount: Uint128) -> Self {
        Self { info, amount }
    }

    pub fn send(&self, target: &Addr) -> StdResult<CosmosMsg> {
        self.info.send(target, self.amount)
    }

    pub fn burn(&self) -> StdResult<CosmosMsg> {
        self.info.burn(self.amount)
    }

    pub fn attributes(&self) -> Vec<Attribute> {
        let mut attributes = self.info.attributes();
        attributes.push(Attribute::new("amount", self.amount));
        attributes
    }

    pub fn key(&self) -> TokenKey {
        self.info.key()
    }

    pub fn from_key(key: TokenKey, amount: Uint128) -> Self {
        Self {
            info: TokenInfo::from_key(key),
            amount,
        }
    }

    pub fn native(denom: impl Into<String>, amount: Uint128) -> Self {
        Self {
            info: TokenInfo::native(denom),
            amount,
        }
    }

    pub fn cw20(address: Addr, amount: Uint128) -> Self {
        Self {
            info: TokenInfo::cw20(address),
            amount,
        }
    }
}
