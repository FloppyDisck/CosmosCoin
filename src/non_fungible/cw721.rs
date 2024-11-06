use crate::non_fungible::NonFungible;
use crate::{execute_wasm, AttributeBuilder};
use cosmwasm_std::{Addr, Attribute, CosmosMsg, Deps, Empty, StdResult};
use cw721::{NumTokensResponse, OwnerOfResponse, TokensResponse};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::marker::PhantomData;

pub type BaseCw721Token = Cw721Token<Empty, Empty, Empty>;

#[derive(Clone, Debug, PartialEq)]
pub struct Cw721Token<T, E, Q> {
    pub address: Addr,
    phantom_data: PhantomData<(T, E, Q)>,
}

impl<T, E, Q> Cw721Token<T, E, Q> {
    pub fn new(address: Addr) -> Self {
        Self {
            address,
            phantom_data: Default::default(),
        }
    }
}

impl<T, E, Q> NonFungible for Cw721Token<T, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    E: Serialize + DeserializeOwned + Clone,
    Q: Serialize + DeserializeOwned + Clone + JsonSchema,
{
    type Extension = T;
    fn send(&self, target: impl Into<String>, token: impl Into<String>) -> StdResult<CosmosMsg> {
        execute_wasm(
            &self.address,
            &cw721_base::ExecuteMsg::<T, E>::TransferNft {
                recipient: target.into(),
                token_id: token.into(),
            },
        )
    }

    fn burn(&self, token: impl Into<String>) -> StdResult<CosmosMsg> {
        execute_wasm(
            &self.address,
            &cw721_base::ExecuteMsg::<T, E>::Burn {
                token_id: token.into(),
            },
        )
    }

    fn mint(
        &self,
        token: impl Into<String>,
        owner: impl Into<String>,
        token_uri: Option<String>,
        extension: Self::Extension,
    ) -> StdResult<CosmosMsg> {
        execute_wasm(
            &self.address,
            &cw721_base::ExecuteMsg::<T, E>::Mint {
                token_id: token.into(),
                owner: owner.into(),
                token_uri,
                extension,
            },
        )
    }

    fn owner_of(&self, token: impl Into<String>, deps: Deps) -> StdResult<OwnerOfResponse> {
        deps.querier.query_wasm_smart::<OwnerOfResponse>(
            &self.address,
            &cw721_base::QueryMsg::<Q>::OwnerOf {
                token_id: token.into(),
                include_expired: None,
            },
        )
    }

    fn total_tokens(&self, deps: Deps) -> StdResult<u64> {
        deps.querier
            .query_wasm_smart::<NumTokensResponse>(
                &self.address,
                &cw721_base::QueryMsg::<Q>::NumTokens {},
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
                &cw721_base::QueryMsg::<Q>::Tokens {
                    owner,
                    start_after,
                    limit,
                },
            )
        } else {
            deps.querier.query_wasm_smart::<TokensResponse>(
                &self.address,
                &cw721_base::QueryMsg::<Q>::AllTokens { start_after, limit },
            )
        }
        .map(|tokens| tokens.tokens)
    }
}

impl<T, E, Q> AttributeBuilder for Cw721Token<T, E, Q> {
    fn attributes(&self) -> StdResult<Vec<Attribute>> {
        Ok(vec![
            Attribute::new("type", "cw721"),
            Attribute::new("address", &self.address),
        ])
    }
}

impl<T, E, Q> Serialize for Cw721Token<T, E, Q> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.address.as_str())
    }
}

impl<'de, T, E, Q> Deserialize<'de> for Cw721Token<T, E, Q> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(|addr| Self::new(Addr::unchecked(addr)))
    }
}

impl<T, E, Q> JsonSchema for Cw721Token<T, E, Q> {
    fn schema_name() -> String {
        "Cw721Token".to_owned()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed(concat!(module_path!(), "::", "Cw721Token"))
    }
    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        gen.subschema_for::<Addr>()
    }
}

#[cfg(test)]
mod test {
    use crate::non_fungible::cw721::BaseCw721Token;
    use cosmwasm_std::Addr;

    #[test]
    fn serde() {
        let cw721 = BaseCw721Token::new(Addr::unchecked("some_token"));
        let got_serialized = serde_json::to_string(&cw721).unwrap();
        let expected_serialized = "\"some_token\"".to_string();

        assert_eq!(got_serialized, expected_serialized);
        assert_eq!(
            serde_json::from_str::<BaseCw721Token>(&expected_serialized).unwrap(),
            cw721
        );
    }
}
