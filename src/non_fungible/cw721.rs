use crate::non_fungible::NonFungible;
use crate::AttributeBuilder;
use cosmwasm_std::{to_json_binary, Addr, Attribute, CosmosMsg, Deps, StdResult, Uint128, WasmMsg};
use cw721::{NumTokensResponse, OwnerOfResponse, TokensResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Cw721Token {
    pub address: Addr,
}

impl Serialize for Cw721Token {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.address.as_str())
    }
}

impl<'de> Deserialize<'de> for Cw721Token {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(|addr| Self::new(Addr::unchecked(addr)))
    }
}

impl Cw721Token {
    pub fn new(address: Addr) -> Self {
        Self { address }
    }
}

impl NonFungible for Cw721Token {
    fn send(&self, target: impl Into<String>, token: impl Into<String>) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: self.address.to_string(),
            msg: to_json_binary(&cw721::Cw721ExecuteMsg::TransferNft {
                recipient: target.into(),
                token_id: token.into(),
            })?,
            funds: vec![],
        }))
    }

    fn burn(&self, token: impl Into<String>) -> StdResult<CosmosMsg> {
        Ok(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: self.address.to_string(),
            msg: to_json_binary(&cw721::Cw721ExecuteMsg::Burn {
                token_id: token.into(),
            })?,
            funds: vec![],
        }))
    }

    fn owner_of(&self, token: impl Into<String>, deps: Deps) -> StdResult<OwnerOfResponse> {
        deps.querier.query_wasm_smart::<OwnerOfResponse>(
            &self.address,
            &cw721::Cw721QueryMsg::OwnerOf {
                token_id: token.into(),
                include_expired: None,
            },
        )
    }

    fn total_tokens(&self, deps: Deps) -> StdResult<u64> {
        deps.querier
            .query_wasm_smart::<NumTokensResponse>(
                &self.address,
                &cw721::Cw721QueryMsg::NumTokens {},
            )
            .map(|num| num.count)
    }

    fn tokens(
        &self,
        owner: Option<String>,
        start_after: Option<String>,
        limit: Option<u32>,
        deps: Deps,
    ) -> StdResult<Vec<String>> {
        if let Some(owner) = owner {
            deps.querier.query_wasm_smart::<TokensResponse>(
                &self.address,
                &cw721::Cw721QueryMsg::Tokens {
                    owner: owner.into(),
                    start_after,
                    limit,
                },
            )
        } else {
            deps.querier.query_wasm_smart::<TokensResponse>(
                &self.address,
                &cw721::Cw721QueryMsg::AllTokens { start_after, limit },
            )
        }
        .map(|tokens| tokens.tokens)
    }
}

impl AttributeBuilder for Cw721Token {
    fn attributes(&self) -> StdResult<Vec<Attribute>> {
        Ok(vec![
            Attribute::new("type", "cw721"),
            Attribute::new("address", &self.address),
        ])
    }
}
