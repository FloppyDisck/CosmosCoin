use crate::{AttributeBuilder, Fungible};
use cosmwasm_std::{Attribute, BankMsg, Coin, CosmosMsg, Deps, StdResult, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, JsonSchema)]
pub struct NativeToken {
    pub denom: String,
}

impl Serialize for NativeToken {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.denom)
    }
}

impl<'de> Deserialize<'de> for NativeToken {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(|denom| Self::new(denom))
    }
}

impl NativeToken {
    pub fn new(denom: impl Into<String>) -> Self {
        Self {
            denom: denom.into(),
        }
    }
}

impl Fungible for NativeToken {
    fn send(&self, target: impl Into<String>, amount: impl Into<Uint128>) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Bank(BankMsg::Send {
            to_address: target.into(),
            amount: vec![Coin {
                denom: self.denom.clone(),
                amount: amount.into(),
            }],
        }))
    }

    fn burn(&self, amount: impl Into<Uint128>) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Bank(BankMsg::Burn {
            amount: vec![Coin {
                denom: self.denom.clone(),
                amount: amount.into(),
            }],
        }))
    }

    fn balance(&self, address: impl Into<String>, deps: Deps) -> StdResult<Uint128> {
        deps.querier
            .query_balance(address, &self.denom)
            .map(|coin| coin.amount)
    }
}

impl AttributeBuilder for NativeToken {
    fn attributes(&self) -> StdResult<Vec<Attribute>> {
        Ok(vec![
            Attribute::new("type", "native"),
            Attribute::new("denom", &self.denom),
        ])
    }
}

#[cfg(test)]
mod test {
    use crate::fungible::NativeToken;

    #[test]
    fn serde() {
        let native = NativeToken::new("some_token");
        let got_serialized = serde_json::to_string(&native).unwrap();
        let expected_serialized = "\"some_token\"".to_string();

        assert_eq!(got_serialized, expected_serialized);
        assert_eq!(
            serde_json::from_str::<NativeToken>(&expected_serialized).unwrap(),
            native
        );
    }
}
