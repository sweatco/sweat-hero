use near_sdk::{
    serde::{Deserialize, Serialize},
    AccountId,
};

use crate::{token::Token, token_metadata::TokenMetadata, TokenId};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenView {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: TokenMetadata,
}

impl TokenView {
    pub fn from_token(token: &Token, token_id: TokenId) -> Self {
        Self {
            token_id,
            owner_id: token.owner_id.clone(),
            metadata: token.metadata.clone(),
        }
    }
}
