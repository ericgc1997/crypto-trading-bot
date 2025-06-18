mod domain;
mod infraestructure;
use crate::{
    domain::pair_generator::PairGenerator, infraestructure::provider_factory::ProviderFactory,
};
// use domain::token_pair::TokenPair;
fn main() {
    let tokens_id_list = vec!["btc", "eth", "sol"];
    let exanges_list = vec!["uniswap"];
    let mut price_provider_list = vec![];
    let token_pairs = PairGenerator::generate_pairs(&tokens_id_list);
    for exange in exanges_list {
        match ProviderFactory::get(exange, &token_pairs) {
            Ok(provider) => {
                provider.build(tokens_id_list);
                price_provider_list.push(provider);
            }
            Err(e) => println!("{}", e),
        };
    }
}
