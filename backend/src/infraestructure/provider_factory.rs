use super::uniswap_provider::UniswapProvider;
use crate::domain::traits::price_provider::PriceProvider;
use anyhow::{anyhow, Error};
pub struct ProviderFactory {}
use crate::domain::token_pair::TokenPair;
impl ProviderFactory {
    pub fn get(
        provider_name: &str,
        tokens_pairs: &[&TokenPair],
    ) -> Result<Box<dyn PriceProvider>, Error> {
        match provider_name {
            "uniswap" => Ok(Box::new(UniswapProvider::new(tokens_pairs))),
            _ => Err(anyhow!("unknouwn provider name {}", provider_name)),
        }
    }
}
