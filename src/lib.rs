use addresses::amm_addrs::uniswap_v2;
// re-exports
pub use token::Token;
use pool_sync::*;

// modules defines
pub mod addresses;
pub mod amms;
pub mod token;
pub mod util;
pub mod sync_pools;

use alloy::primitives::Address;
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::transports::http::reqwest::Url;
use std::sync::Arc;
use std::collections::{HashMap, HashSet};

use crate::amms::uniswap::v2::factory_v2::UniswapV2Factory;
use crate::amms::uniswap::v2::pool_v2::UniswapV2Pool;
use crate::util::{ArcHttpProvider, ArcWsProvider};



pub struct Gweisyer {
    http: ArcHttpProvider,
    ws: ArcWsProvider,
    uniswapv2_factory: UniswapV2Factory,
    symbol_to_pools: HashMap<String, HashSet<usize>>,
    pair_to_pool: HashMap<(String, String), usize>,
    token_to_address: HashMap<String, Address>,
}

impl Gweisyer {
    /// Construct a new Gweisyer
    /// will instantiate a new provider with the provided rpc url
    pub async fn new(http_url: impl Into<String>, ws_url: impl Into<String>) -> Self {
        // todo! make this just take a provider, ex may want an anvil provider for state updates
        let http_url: Url = http_url.into().parse().unwrap();
        let http = Arc::new(ProviderBuilder::new().on_http(http_url));
        let ws = Arc::new(
            ProviderBuilder::new()
                .on_ws(WsConnect::new(ws_url))
                .await
                .unwrap(),
        );

        let pool_sync = PoolSync::builder()
            .add_pool(PoolType::UniswapV2)
            .chain(Chain::Ethereum)
            .rate_limit(25)
            .build().unwrap();

        let pools = pool_sync.sync_pools(http.clone()).await.unwrap();
        let mut symbol_to_pools = HashMap::new();
        let mut pair_to_pool = HashMap::new();

        for (index, pool) in pools.iter().enumerate() {
            let token0_symbol = pool.token0();
        }




        let uniswapv2_factory = UniswapV2Factory::new(http.clone(), ws.clone());
        Self {
            http,
            ws,
            uniswapv2_factory,
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



    pub fn get_pools_with_token(base: &Token) -> Vec<Pool>{
        let pools = Vec::new();
        pools

    }


    pub fn get_pool(base: &Token, quote: &Token) -> Pool {
        todo!()

    }

}



