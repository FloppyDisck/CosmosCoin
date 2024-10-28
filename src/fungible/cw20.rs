use crate::{AttributeBuilder, Fungible};
use cosmwasm_std::{to_json_binary, Addr, Attribute, CosmosMsg, Deps, StdResult, Uint128, WasmMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq)]
pub struct Cw20Token {
    pub address: Addr,
}

impl Cw20Token {
    pub fn new(address: Addr) -> Self {
        Self { address }
    }

    pub fn transfer_from(
        &self,
        owner: &Addr,
        recipient: &Addr,
        amount: Uint128,
    ) -> StdResult<CosmosMsg> {
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

impl Fungible for Cw20Token {
    fn send(&self, target: impl Into<String>, amount: impl Into<Uint128>) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: self.address.to_string(),
            msg: to_json_binary(&cw20_base::msg::ExecuteMsg::Transfer {
                recipient: target.into(),
                amount: amount.into(),
            })?,
            funds: vec![],
        }))
    }

    fn burn(&self, amount: impl Into<Uint128>) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: self.address.to_string(),
            msg: to_json_binary(&cw20_base::msg::ExecuteMsg::Burn {
                amount: amount.into(),
            })?,
            funds: vec![],
        }))
    }

    fn balance(&self, address: impl Into<String>, deps: Deps) -> StdResult<Uint128> {
        deps.querier
            .query_wasm_smart::<BalanceResponse>(
                &self.address,
                &cw20_base::msg::QueryMsg::Balance {
                    address: address.into(),
                },
            )
            .map(|res| res.balance)
    }
}

impl AttributeBuilder for Cw20Token {
    fn attributes(&self) -> StdResult<Vec<Attribute>> {
        Ok(vec![
            Attribute::new("type", "cw20"),
            Attribute::new("address", &self.address),
        ])
    }
}

impl Serialize for Cw20Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.address.as_str())
    }
}

impl<'de> Deserialize<'de> for Cw20Token {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(|addr| Self::new(Addr::unchecked(addr)))
    }
}

impl JsonSchema for Cw20Token {
    fn schema_name() -> String {
        "Cw20Token".to_owned()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(concat!(module_path!(), "::", "Cw20Token"))
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<Addr>()
    }
}

/// For some reason the cw20 library has differing versions for types
#[derive(Deserialize)]
struct BalanceResponse {
    pub balance: Uint128,
}

#[cfg(test)]
mod test {
    use crate::fungible::Cw20Token;
    use cosmwasm_std::Addr;

    #[test]
    fn serde() {
        let cw20 = Cw20Token::new(Addr::unchecked("some_token"));
        let got_serialized = serde_json::to_string(&cw20).unwrap();
        let expected_serialized = "\"some_token\"".to_string();

        assert_eq!(got_serialized, expected_serialized);
        assert_eq!(
            serde_json::from_str::<Cw20Token>(&expected_serialized).unwrap(),
            cw20
        );
    }
}
