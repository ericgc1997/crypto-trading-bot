use super::token_pair::TokenPair;

pub struct Price {
    exchange: String, // : "binance", "uniswap"
    pair: TokenPair,
    sell_price: f64,
    buy_price: f64,
}
