use super::super::price::Price;
use super::super::token_pair::TokenPair;
use anyhow::Error;

pub trait PriceProvider {
    fn get_price(&self, pair: &TokenPair) -> Result<Price, Error>;
}
