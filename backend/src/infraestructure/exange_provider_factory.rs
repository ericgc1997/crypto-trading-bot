use super::uniswap_provider::UniswapProvider;
use crate::domain::traits::price_provider::PriceProvider;
use anyhow::{anyhow, Error};
pub fn provider_factory(provider_name: &str) -> Result<Box<dyn PriceProvider>, anyhow::Error> {
    match provider_name {
        "uniswap" => Ok(Box::new(UniswapProvider {})),
        _ => Err(anyhow!("unknouwn provider name {}", provider_name)),
    }
}
