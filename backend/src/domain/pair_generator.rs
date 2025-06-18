use super::token_pair::TokenPair;
pub struct PairGenerator {}
impl PairGenerator {
    pub fn generate_pairs(tokens: &[&str]) -> Vec<TokenPair> {
        let mut pairs: Vec<TokenPair> = vec![];
        for i in 0..tokens.len() {
            for j in (i + 1)..tokens.len() {
                let new_pair = TokenPair {
                    first: tokens[i].to_string(),
                    second: tokens[j].to_string(),
                };
                pairs.push(new_pair);
            }
        }
        pairs
    }
}
