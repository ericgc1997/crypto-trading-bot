mod application;
mod domain;
mod infraestructure;
use crate::{
    application::provider_factory::ProviderFactory, domain::pair_generator::PairGenerator,
};
// use domain::token_pair::TokenPair;
fn main() {
    let tokens_id_list = vec!["btc", "eth", "sol"];
    let exanges_list = vec!["uniswap", "sushiswap"];
    let mut price_provider_list = vec![];

    let token_pairs = PairGenerator::generate_pairs(&tokens_id_list);

    //create the variables to handle each exange
    for exange in exanges_list {
        match ProviderFactory::get(exange, &token_pairs) {
            Ok(provider) => {
                price_provider_list.push(provider);
            }
            Err(e) => println!("{}", e),
        };
    }
}
