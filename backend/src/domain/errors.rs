use thiserror::Error;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Unknown provider type: {0}")]
    UnknownProvider(#[from] ProviderError),
}
