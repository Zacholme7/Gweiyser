use alloy::transports::Transport;
use alloy::primitives::Address;
use alloy::providers::Provider;
use std::marker::PhantomData;
use alloy::network::Network;
use std::sync::Arc;

use crate::token::Token;
use crate::protocols::uniswap::v2::pool_v2::UniswapV2Pool;

// modules defines
pub mod protocols;
mod token;
mod sync_pools;
pub mod util;
pub mod addresses;

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
        let token = Token::new(address, self.http.clone()).await;
        token
    }

    /// Construct a new uniswapv2 pool
    pub async fn uniswap_v2_pool(&self, address: Address) -> UniswapV2Pool {
        let pool = UniswapV2Pool::new(address, self.http.clone()).await;
        pool
    }

    /* 
    /// Construct a new uniswapv2 router
    pub async fn uniswap_v2_router(&self) -> UniswapV2Router<P, T, N> {
        let router = UniswapV2Router::new(self.http.clone()).await;
        router
    }

    /// Construct a new uniswapv2 factory
    pub async fn uniswap_v2_factory(&self) -> UniswapV2Factory<P, T, N> {
        let factory = UniswapV2Factory::new(self.http.clone()).await;
        factory
    }

    /// Construct a new uniswapv3 pool
    pub async fn uniswap_v3_pool(&self, address: Address) -> UniswapV3Pool<P, T, N> {
        let pool = UniswapV3Pool::new(address, self.http.clone()).await;
        pool
    }


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

    /// Construct a new uniswapv3 factory
    pub async fn uniswap_v3_factory(&self) -> UniswapV3Factory<P, T, N> {
        let factory = UniswapV3Factory::new(self.http.clone()).await;
        factory
    }
    */
 }



