#![doc = include_str!("../README.md")]

#[cfg(not(any(feature = "grammers-session", feature = "grammers-session-0.9.0")))]
compile_error!("No \"grammers-session\" feature chosen. Either \"grammers-session\" or \"grammers-session-0.9.0\" must be enabled");

#[cfg(all(feature = "grammers-session", feature = "grammers-session-0.9.0"))]
compile_error!("\"grammers-session\" and \"grammers-session-0.9.0\" features are mutually exclusive and cannot be enabled at the same time. You may want to disable the default \"grammers-session\" feature");

mod utils;
mod version;

pub use utils::{DecodeError, export, restore};

#[cfg(feature = "grammers-session")]
pub use grammers_session;
#[cfg(feature = "grammers-session-0.9.0")]
pub use grammers_session_0_9_0 as grammers_session;
