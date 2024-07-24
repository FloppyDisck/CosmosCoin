use crate::GenericToken;
use cosmwasm_std::{Addr, Attribute, BankMsg, Coin, CosmosMsg, Deps, StdResult, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct NativeToken {
    pub denom: String,
}

impl NativeToken {
    pub fn new(denom: String) -> Self {
        Self { denom }
    }
}

impl GenericToken for NativeToken {
    fn send(&self, target: &Addr, amount: Uint128) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Bank(BankMsg::Send {
            to_address: target.to_string(),
            amount: vec![Coin {
                denom: self.denom.clone(),
                amount,
            }],
        }))
    }

    fn burn(&self, amount: Uint128) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Bank(BankMsg::Burn {
            amount: vec![Coin {
                denom: self.denom.clone(),
                amount,
            }],
        }))
    }

    fn balance(&self, address: &Addr, deps: Deps) -> StdResult<Uint128> {
        deps.querier
            .query_balance(address, &self.denom)
            .map(|coin| coin.amount)
    }

    fn attributes(&self) -> Vec<Attribute> {
        vec![
            Attribute::new("type", "native"),
            Attribute::new("denom", &self.denom),
        ]
    }
}
