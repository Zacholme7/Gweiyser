use addresses::amm_addrs::uniswap_v2;
// re-exports
pub use token::Token;

// modules defines
pub mod amms;
pub mod addresses;
pub mod token;
pub mod util;

use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::transports::http::reqwest::Url;
use alloy::primitives::Address;
use std::sync::Arc;

use crate::util::{ArcHttpProvider, ArcWsProvider};
use crate::amms::uniswap::v2::pool::UniswapV2Pool;
use crate::amms::uniswap::v2::factory::UniswapV2Factory;




pub struct Gweisyer {
        http: ArcHttpProvider,
        ws: ArcWsProvider,
        uniswapv2_factory: UniswapV2Factory
}


impl Gweisyer {
        /// Construct a new Gweisyer
        /// will instantiate a new provider with the provided rpc url
        pub async fn new(http_url: impl Into<String>, ws_url: impl Into<String>) -> Self {
                let http_url: Url = http_url.into().parse().unwrap();
                let http = Arc::new(ProviderBuilder::new()
                        .on_http(http_url));
                let ws = Arc::new(ProviderBuilder::new()
                        .on_ws(WsConnect::new(ws_url)).await.unwrap());

                let uniswapv2_factory = UniswapV2Factory::new(http.clone(), ws.clone());
                Self {
                        http,
                        ws,
                        uniswapv2_factory
                }
        }

        // Construct an new token
        pub async fn token(&self, address: Address) -> Token {
                // Just delegate the call to the token object
                let token = Token::new(address, self.http.clone()).await;
                token
        }

        /// Build a new UniswapV2AmmPool
        /// This will lazy load the factory address, only want the factory is we are looking for a pool
        pub async fn uniswapV2(&self, base: &Token, quote: &Token) -> UniswapV2Pool {
                self.uniswapv2_factory.query_for_pool(base, quote).await
        }
        

}
