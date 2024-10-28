use crate::non_fungible::info::NonFungibleTokenInfo;
use crate::non_fungible::NonFungible;
use crate::{AttributeBuilder, TokenKey};
use cosmwasm_std::{Addr, Attribute, CosmosMsg, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NonFungibleToken {
    pub info: NonFungibleTokenInfo,
    pub token: String,
}

impl NonFungibleToken {
    pub fn new(info: NonFungibleTokenInfo, token: impl Into<String>) -> Self {
        Self {
            info,
            token: token.into(),
        }
    }

    pub fn send(&self, target: &Addr) -> StdResult<CosmosMsg> {
        self.info.send(target, &self.token)
    }

    pub fn burn(&self) -> StdResult<CosmosMsg> {
        self.info.burn(&self.token)
    }

    pub fn key(&self) -> TokenKey {
        self.info.key()
    }

    pub fn from_key(key: TokenKey, token: impl Into<String>) -> Self {
        Self {
            info: NonFungibleTokenInfo::from_key(key),
            token: token.into(),
        }
    }

    pub fn base(address: Addr, token: impl Into<String>) -> Self {
        Self {
            info: NonFungibleTokenInfo::base(address),
            token: token.into(),
        }
    }
}

impl AttributeBuilder for NonFungibleToken {
    fn attributes(&self) -> StdResult<Vec<Attribute>> {
        let mut attributes = self.info.attributes()?;
        attributes.push(Attribute::new("token", &self.token));
        Ok(attributes)
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use cosmwasm_std::Addr;

    #[test]
    fn serde_base() {
        let base = NonFungibleTokenInfo::base(Addr::unchecked("some_token"));
        let got_serialized = serde_json::to_string(&base).unwrap();
        let expected_serialized = "{\"base\":\"some_token\"}".to_string();

        assert_eq!(got_serialized, expected_serialized);
        assert_eq!(
            serde_json::from_str::<NonFungibleTokenInfo>(&expected_serialized).unwrap(),
            base
        );
    }
}
