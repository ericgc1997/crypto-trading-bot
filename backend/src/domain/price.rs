struct Price {
    exchange: ExchangeId, // : "binance", "uniswap"
    pair: TokenPair,
    sell_price: f64,
    buy_price: f64,
}
