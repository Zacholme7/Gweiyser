use alloy::network::Network;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::transports::Transport;
use protocols::uniswap::v3::quoter::UniswapV3Quoter;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::protocols::uniswap::v2::{UniswapV2Factory, UniswapV2Pool, UniswapV2Router};
use crate::protocols::uniswap::v3::{UniswapV3Factory, UniswapV3Pool, UniswapV3Router};
use crate::token::Token;

// modules defines
pub mod addresses;
pub mod protocols;
mod sync_pools;
mod token;
pub mod util;

#[derive(Clone, Copy, PartialEq)]
pub enum Chain {
    Ethereum,
    Base,
}

pub struct Gweiyser<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    http: Arc<P>,
    chain: Chain,
    _phantom: PhantomData<(T, N)>,
}

impl<P, T, N> Gweiyser<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    /// Construct a new Gweisyer
    pub fn new(http_provider: Arc<P>, chain: Chain) -> Self {
        Self {
            http: http_provider,
            chain,
            _phantom: PhantomData,
        }
    }

    // Construct an new token
    pub async fn token(&self, address: Address) -> Token<P, T, N> {
        // Just delegate the call to the token object
        Token::new(address, self.http.clone()).await
    }

    /// Construct a new uniswapv2 pool
    pub async fn uniswap_v2_pool(&self, address: Address) -> UniswapV2Pool<P, T, N> {
        UniswapV2Pool::new(address, self.http.clone()).await
    }

    /// Construct a new uniswapv2 router
    pub fn uniswap_v2_router(&self) -> UniswapV2Router<P, T, N> {
        UniswapV2Router::new(self.http.clone(), self.chain)
    }

    /// Construct a new uniswapv2 factory
    pub fn uniswap_v2_factory(&self) -> UniswapV2Factory<P, T, N> {
        UniswapV2Factory::new(self.http.clone(), self.chain)
    }

    /// Construct a new uniswapv3 pool
    pub async fn uniswap_v3_pool(&self, address: Address) -> UniswapV3Pool<P, T, N> {
        UniswapV3Pool::new(address, self.http.clone()).await
    }

    /// Construct a new uniswapv3 factory
    pub fn uniswap_v3_factory(&self) -> UniswapV3Factory<P, T, N> {
        UniswapV3Factory::new(self.http.clone(), self.chain)
    }

    /// Construct a new uniswapv3 quoter
    pub fn uniswap_v3_quoter(&self) -> UniswapV3Quoter<P, T, N> {
        UniswapV3Quoter::new(self.http.clone(), self.chain)
    }

    /// Construct a new uniswapv3 router
    pub fn uniswap_v3_router(&self) -> UniswapV3Router<P, T, N> {
        UniswapV3Router::new(self.http.clone(), self.chain)
    }


}
