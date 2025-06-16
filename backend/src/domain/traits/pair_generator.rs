use super::super::token_pair::TokenPair;
pub trait PairGenerator {
    fn generate_pairs(&self, tokens: &[&str]) -> Vec<TokenPair>;
}
