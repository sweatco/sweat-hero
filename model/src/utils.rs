//! Convenience traits for serialization and deserialization.

use std::{any::type_name, fmt::Debug};

use near_sdk::{
    env::panic_str,
    serde::{de::DeserializeOwned, Serialize},
    serde_json::{from_str, to_string},
};

pub trait ToStringOrPanic {
    fn to_string(&self) -> String;
}

impl<T: Serialize + Debug> ToStringOrPanic for T {
    fn to_string(&self) -> String {
        to_string(self).unwrap_or_else(|err| {
            panic_str(&format!(
                "Failed to serialize '{}': {self:?}. Error: {err}",
                type_name::<T>()
            ))
        })
    }
}

pub trait FromStringOrPanic {
    fn from_string(s: &str) -> Self;
}

impl<T: DeserializeOwned + Debug> FromStringOrPanic for T {
    fn from_string(s: &str) -> Self {
        from_str(s).unwrap_or_else(|err| {
            panic_str(&format!(
                "Failed to serialize '{}' from: {s:?}. Error: {err}",
                type_name::<T>()
            ))
        })
    }
}
