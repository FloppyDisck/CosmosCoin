use crate::GenericToken;
use cosmwasm_std::{to_json_binary, Addr, Attribute, CosmosMsg, Deps, StdResult, Uint128, WasmMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Cw20Token {
    pub address: Addr,
}

impl Cw20Token {
    pub fn new(address: Addr) -> Self {
        Self { address }
    }
}

impl GenericToken for Cw20Token {
    fn send(&self, target: &Addr, amount: Uint128) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: self.address.to_string(),
            msg: to_json_binary(&cw20_base::msg::ExecuteMsg::Transfer {
                recipient: target.to_string(),
                amount,
            })?,
            funds: vec![],
        }))
    }

    fn burn(&self, amount: Uint128) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: self.address.to_string(),
            msg: to_json_binary(&cw20_base::msg::ExecuteMsg::Burn { amount })?,
            funds: vec![],
        }))
    }

    fn balance(&self, address: &Addr, deps: Deps) -> StdResult<Uint128> {
        deps.querier
            .query_wasm_smart::<BalanceResponse>(
                &self.address,
                &cw20_base::msg::QueryMsg::Balance {
                    address: address.to_string(),
                },
            )
            .map(|res| res.balance)
    }

    fn attributes(&self) -> Vec<Attribute> {
        vec![
            Attribute::new("type", "cw20"),
            Attribute::new("address", &self.address),
        ]
    }
}

/// For some reason the cw20 library has differing versions for types
#[derive(Deserialize)]
struct BalanceResponse {
    pub balance: Uint128,
}
