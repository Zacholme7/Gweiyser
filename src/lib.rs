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
use std::marker::PhantomData;
use alloy::providers::Provider;
use alloy::transports::Transport;
use alloy::network::Network;
use std::sync::Arc;
use std::collections::{HashMap, HashSet};

use crate::amms::uniswap::v2::factory_v2::UniswapV2Factory;
use crate::amms::uniswap::v2::pool_v2::UniswapV2Pool;
use crate::util::{ArcHttpProvider, ArcWsProvider};


pub struct Gweisyer<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    http: Arc<P>,
    _phantom: PhantomData<(T, N)>,
}

impl<P, T, N> Gweisyer<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    /// Construct a new Gweisyer
    pub fn new(http_provider: Arc<P>) -> Self {
        Self {
            http: http_provider,
            _phantom: PhantomData,
        }
    }

    // Construct an new token
    pub async fn token(&self, address: Address) -> Token<P, T, N> {
        // Just delegate the call to the token object
        let token = Token::new(address, self.http.clone()).await;
        token
    }

    /* 
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
    */

}



