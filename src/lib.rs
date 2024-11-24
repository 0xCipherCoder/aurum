//! Aurum: A highly scalable bot framework for Solana
#![warn(missing_docs)]
#![forbid(unsafe_code)]

pub mod core;
pub mod providers;
pub mod plugins;
pub mod security;
pub mod metrics;
pub mod utils;

pub use core::Strategy;

/// Version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod prelude {
    //! Convenient imports for common types
    pub use crate::core::{Strategy, ExecutionContext};
    pub use crate::utils::error::{AurumError, AurumResult};
}
