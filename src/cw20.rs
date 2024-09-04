use crate::GenericToken;
use cosmwasm_std::{to_json_binary, Addr, Attribute, CosmosMsg, Deps, StdResult, Uint128, WasmMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Cw20Token {
    pub address: Addr,
}

impl Serialize for Cw20Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.address.as_str())
    }
}

impl<'de> Deserialize<'de> for Cw20Token {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        String::deserialize(deserializer).map(|addr| Self::new(Addr::unchecked(addr)))
    }
}

impl Cw20Token {
    pub fn new(address: Addr) -> Self {
        Self { address }
    }

    pub fn transfer_from(&self, owner: &Addr, recipient: &Addr, amount: Uint128) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: self.address.to_string(),
            msg: to_json_binary(&cw20_base::msg::ExecuteMsg::TransferFrom {
                owner: owner.to_string(),
                recipient: recipient.to_string(),
                amount,
            })?,
            funds: vec![],
        }))
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

#[cfg(test)]
mod test {
    use cosmwasm_std::Addr;
    use crate::Cw20Token;

    #[test]
    fn serde() {
        let cw20 = Cw20Token::new(Addr::unchecked("some_token"));
        let got_serialized = serde_json::to_string(&cw20).unwrap();
        let expected_serialized = "\"some_token\"".to_string();

        assert_eq!(got_serialized, expected_serialized);
        assert_eq!(serde_json::from_str::<Cw20Token>(&expected_serialized).unwrap(), cw20);
    }
}