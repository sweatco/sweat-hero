use near_sdk::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
    json_types::Base64VecU8,
    serde::{Deserialize, Serialize},
};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: String,                   // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: String,             // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub copies: Option<u64>,             // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<u64>,          // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>,         // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>,          // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>,         // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>,           // anything extra the NFT wants to store on-chain. Can be stringified JSON.
}

impl TokenMetadata {
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            media: None,
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
        }
    }
}
