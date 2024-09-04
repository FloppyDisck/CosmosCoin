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

#[cfg(test)]
mod test {
    use cosmwasm_std::Addr;
    use crate::TokenInfo;

    #[test]
    fn serde_cw20() {
        let cw20 = TokenInfo::cw20(Addr::unchecked("some_token"));
        let got_serialized = serde_json::to_string(&cw20).unwrap();
        let expected_serialized = "{\"cw20\":\"some_token\"}".to_string();

        assert_eq!(got_serialized, expected_serialized);
        assert_eq!(serde_json::from_str::<TokenInfo>(&expected_serialized).unwrap(), cw20);
    }

    #[test]
    fn serde_native() {
        let native = TokenInfo::native("some_token");
        let got_serialized = serde_json::to_string(&native).unwrap();
        let expected_serialized = "{\"native\":\"some_token\"}".to_string();

        assert_eq!(got_serialized, expected_serialized);
        assert_eq!(serde_json::from_str::<TokenInfo>(&expected_serialized).unwrap(), native);
    }
}