trait PriceProvider {
    fn get_price(&self, pair: &TokenPair) -> Result<Price, DomainError>;
}
