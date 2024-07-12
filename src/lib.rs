// reexports
pub use token::Token;

// modules defines
pub mod amms;
pub mod addresses;
pub mod token;
pub mod util;

use alloy::providers::{ProviderBuilder, RootProvider};
use alloy::transports::http::{Http, Client};
use alloy::transports::http::reqwest::Url;
use alloy::primitives::Address;
use std::sync::Arc;

use crate::util::ArcHttpProvider;

pub struct Gweisyer {
        provider: ArcHttpProvider
}


impl Gweisyer {
        /// Construct a new Gweisyer
        /// will instantiate a new provider with the provided rpc url
        pub fn new(url: impl Into<String> ) -> Self {
                let url: Url = url.into().parse().unwrap();
                let provider = Arc::new(ProviderBuilder::new()
                        .on_http(url));

                Self {
                        provider
                }
        }

        // Construct an new token
        pub async fn token(&self, address: Address) -> Token {
                // Just delegate the call to the token object
                let token = Token::new(address, self.provider.clone()).await;
                token
        }

                /* 
        /// Build a new UniswapV2AmmPool
        /// This will lazy load the factory address, only want the factory is we are looking for a pool
        pub fn uniswapV2(&mut self, base: Address, quote: Address) -> UniswapV2Pool {
                if self.uniswapv2_factory.is_none() {
                        // this is our first time constructing a pool, we want to initialize the factory for future use
                        let factory = IUniswapV2Factory::new(self.provider.clone());
                        self.uniswapv2_factory  = factory;
                }





                let pool = UniswapV2::new(base, quote, self.provider().clone).await;
                pool
        }
                */
        

}
