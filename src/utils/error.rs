use thiserror::Error;

/// Custom error type for Aurum
#[derive(Error, Debug)]
pub enum AurumError {
    /// Error when interacting with Solana
    #[error("Solana error: {0}")]
    SolanaError(#[from] solana_client::client_error::ClientError),

    /// Error with strategy execution
    #[error("Strategy error: {0}")]
    StrategyError(String),

    /// Error with data providers
    #[error("Data provider error: {0}")]
    DataProviderError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Security-related error
    #[error("Security error: {0}")]
    SecurityError(String),
}

/// Result type for Aurum operations
pub type AurumResult<T> = Result<T, AurumError>;