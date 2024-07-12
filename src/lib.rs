use addresses::amm_addrs::uniswap_v2;
// re-exports
pub use token::Token;

// modules defines
pub mod amms;
pub mod addresses;
pub mod token;
pub mod util;

use alloy::providers::ProviderBuilder;
use alloy::transports::http::reqwest::Url;
use alloy::primitives::Address;
use std::sync::Arc;

use crate::util::ArcHttpProvider;
use crate::amms::uniswap::v2::pool::UniswapV2Pool;
use crate::amms::uniswap::v2::factory::UniswapV2Factory;




pub struct Gweisyer {
        provider: ArcHttpProvider,
        uniswapv2_factory: UniswapV2Factory
}


impl Gweisyer {
        /// Construct a new Gweisyer
        /// will instantiate a new provider with the provided rpc url
        pub fn new(url: impl Into<String> ) -> Self {
                let url: Url = url.into().parse().unwrap();
                let provider = Arc::new(ProviderBuilder::new()
                        .on_http(url));
                let uniswapv2_factory = UniswapV2Factory::new(provider.clone());



                Self {
                        provider,
                        uniswapv2_factory
                }
        }

        // Construct an new token
        pub async fn token(&self, address: Address) -> Token {
                // Just delegate the call to the token object
                let token = Token::new(address, self.provider.clone()).await;
                token
        }

        /// Build a new UniswapV2AmmPool
        /// This will lazy load the factory address, only want the factory is we are looking for a pool
        pub async fn uniswapV2(&self, base: &Token, quote: &Token) -> UniswapV2Pool {
                self.uniswapv2_factory.query_for_pool(base, quote).await
        }
        

}
