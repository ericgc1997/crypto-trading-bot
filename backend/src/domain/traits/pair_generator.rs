trait PairGenerator {
    fn generate_pairs(&self, tokens: &[TokenIdentity]) -> Vec<TokenPair>;
}
