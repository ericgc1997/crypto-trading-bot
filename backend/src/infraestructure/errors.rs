// use thiserror::Error;

// //this derive and error before struct is a macro to avoid boilerplate to have to make impl std::error::Error for InfraError { ... }
// #[derive(Debug, thiserror::Error)]
// pub enum InfraError {
//     #[error("Unknown provider type: {0}")]
//     UnknownProvider(#[from] ProviderError),
// }
