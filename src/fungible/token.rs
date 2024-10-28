use crate::fungible::{Fungible, FungibleTokenInfo};
use crate::{AttributeBuilder, TokenKey};
use cosmwasm_std::{Addr, Attribute, Coin, CosmosMsg, StdResult, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FungibleToken {
    pub info: FungibleTokenInfo,
    pub amount: Uint128,
}

impl From<Coin> for FungibleToken {
    fn from(value: Coin) -> Self {
        Self {
            info: FungibleTokenInfo::native(value.denom),
            amount: value.amount,
        }
    }
}

impl FungibleToken {
    pub fn new(info: FungibleTokenInfo, amount: impl Into<Uint128>) -> Self {
        Self {
            info,
            amount: amount.into(),
        }
    }

    pub fn send(&self, target: &Addr) -> StdResult<CosmosMsg> {
        self.info.send(target, self.amount)
    }

    pub fn burn(&self) -> StdResult<CosmosMsg> {
        self.info.burn(self.amount)
    }

    pub fn key(&self) -> TokenKey {
        self.info.key()
    }

    pub fn from_key(key: TokenKey, amount: Uint128) -> Self {
        Self {
            info: FungibleTokenInfo::from_key(key),
            amount,
        }
    }

    pub fn native(denom: impl Into<String>, amount: impl Into<Uint128>) -> Self {
        Self {
            info: FungibleTokenInfo::native(denom),
            amount: amount.into(),
        }
    }

    pub fn cw20(address: Addr, amount: impl Into<Uint128>) -> Self {
        Self {
            info: FungibleTokenInfo::cw20(address),
            amount: amount.into(),
        }
    }
}

impl AttributeBuilder for FungibleToken {
    fn attributes(&self) -> StdResult<Vec<Attribute>> {
        let mut attributes = self.info.attributes()?;
        attributes.push(Attribute::new("amount", self.amount));
        Ok(attributes)
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use cosmwasm_std::Addr;
    #[test]
    fn serde_cw20() {
        let cw20 = FungibleTokenInfo::cw20(Addr::unchecked("some_token"));
        let got_serialized = serde_json::to_string(&cw20).unwrap();
        let expected_serialized = "{\"cw20\":\"some_token\"}".to_string();

        assert_eq!(got_serialized, expected_serialized);
        assert_eq!(
            serde_json::from_str::<FungibleTokenInfo>(&expected_serialized).unwrap(),
            cw20
        );
    }

    #[test]
    fn serde_native() {
        let native = FungibleTokenInfo::native("some_token");
        let got_serialized = serde_json::to_string(&native).unwrap();
        let expected_serialized = "{\"native\":\"some_token\"}".to_string();

        assert_eq!(got_serialized, expected_serialized);
        assert_eq!(
            serde_json::from_str::<FungibleTokenInfo>(&expected_serialized).unwrap(),
            native
        );
    }
}
