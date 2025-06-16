use super::super::price::Price;
use super::super::token_pair::TokenPair;
use anyhow::Error;

pub trait PriceProvider {
    fn build(&self, pairs_list: &[TokenPair]);
    fn get_price(&self, pair: &TokenPair) -> Result<Price, anyhow::Error>;
}
