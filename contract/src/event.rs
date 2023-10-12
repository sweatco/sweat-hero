use model::TokenId;
use near_sdk::{
    env, log,
    serde::{Deserialize, Serialize},
    serde_json, AccountId,
};

use crate::{PACKAGE_NAME, VERSION};

#[derive(Serialize, Deserialize, Debug)]
#[serde(
    crate = "near_sdk::serde",
    tag = "event",
    content = "data",
    rename_all = "snake_case"
)]
pub enum Event {
    NFTMint(MintData),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde", rename_all = "snake_case")]
struct SweatHeroEvent {
    standard: &'static str,
    version: &'static str,
    #[serde(flatten)]
    event: Event,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct MintData {
    pub receiver_id: AccountId,
    pub token_id: TokenId,
}

impl From<Event> for SweatHeroEvent {
    fn from(event: Event) -> Self {
        Self {
            standard: PACKAGE_NAME,
            version: VERSION,
            event,
        }
    }
}

pub(crate) fn emit(event: Event) {
    SweatHeroEvent::from(event).emit();
}

impl SweatHeroEvent {
    pub(crate) fn emit(&self) {
        log!(self.to_json_event_string());
    }

    fn to_json_string(&self) -> String {
        serde_json::to_string_pretty(self)
            .unwrap_or_else(|err| env::panic_str(&format!("Failed to serialize SweatHeroEvent: {err}")))
    }

    fn to_json_event_string(&self) -> String {
        format!("EVENT_JSON:{}", self.to_json_string())
    }
}
