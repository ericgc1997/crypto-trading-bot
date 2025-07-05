use std::any;

use crate::domain::price::Price;
use crate::domain::token_pair::TokenPair;
use crate::domain::traits::price_provider::PriceProvider;
use anyhow::anyhow;
pub struct UniswapProvider {}

impl UniswapProvider {
    pub fn new(pairs_list: &[TokenPair]) -> Self {
        UniswapProvider {
            // pairs: pairs.iter().map(|s| s.to_string()).collect(),
        }
    }
}
impl PriceProvider for UniswapProvider {
    fn get_price(&self, pair_list: &TokenPair) -> Result<Price, anyhow::Error> {
        return Err(anyhow!("not implementef"));
        // Llamada a contrato getReserves()
    }
}
