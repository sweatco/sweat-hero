pub mod api;
pub mod contract;
mod event;
mod internal;

pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

use crate::contract::{Contract, ContractExt};
