use alloy::network::Network;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::transports::Transport;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::protocols::uniswap::v2::{UniswapV2Factory, UniswapV2Pool, UniswapV2Router};
use crate::protocols::uniswap::v3::{UniswapV3Factory, UniswapV3Pool};
use crate::token::Token;

// modules defines
pub mod addresses;
pub mod protocols;
mod sync_pools;
mod token;
pub mod util;

pub struct Gweiyser<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    http: Arc<P>,
    _phantom: PhantomData<(T, N)>,
}

impl<P, T, N> Gweiyser<P, T, N>
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
        Token::new(address, self.http.clone()).await
    }

    /// Construct a new uniswapv2 pool
    pub async fn uniswap_v2_pool(&self, address: Address) -> UniswapV2Pool {
        UniswapV2Pool::new(address, self.http.clone()).await
    }

    /// Construct a new uniswapv2 router
    pub fn uniswap_v2_router(&self) -> UniswapV2Router<P, T, N> {
        UniswapV2Router::new(self.http.clone())
    }

    /// Construct a new uniswapv2 factory
    pub fn uniswap_v2_factory(&self) -> UniswapV2Factory<P, T, N> {
        UniswapV2Factory::new(self.http.clone())
    }

    /// Construct a new uniswapv3 pool
    pub async fn uniswap_v3_pool(&self, address: Address) -> UniswapV3Pool<P, T, N> {
        UniswapV3Pool::new(address, self.http.clone()).await
    }

    /// Construct a new uniswapv3 factory
    pub fn uniswap_v3_factory(&self) -> UniswapV3Factory<P, T, N> {
        UniswapV3Factory::new(self.http.clone())
    }



    /*

    /// Construct a new uniswapv3 router
    pub async fn uniswap_v3_router(&self) -> UniswapV3Router<P, T, N> {
        let router = UniswapV3Router::new(self.http.clone()).await;
        router
    }

    /// Construct a new uniswapv3 quoter
    pub async fn uniswap_v3_quoter(&self) -> UniswapV3Quoter<P, T, N> {
        let quoter = UniswapV3Quoter::new(self.http.clone()).await;
        quoter
    }

    */
}
