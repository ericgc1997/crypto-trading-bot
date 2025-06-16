use std::any;

use crate::domain::price::Price;
use crate::domain::token_pair::TokenPair;
use crate::domain::traits::price_provider::PriceProvider;
use anyhow::anyhow;
pub struct UniswapProvider {}

impl PriceProvider for UniswapProvider {
    fn build(&self, pairs_list: &[TokenPair]) {}
    fn get_price(&self, pair_list: &TokenPair) -> Result<Price, anyhow::Error> {
        return Err(anyhow!("not implementef"));
        // Llamada a contrato getReserves()
    }
}
