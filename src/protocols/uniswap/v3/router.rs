use alloy::network::Network;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::transports::Transport;
use std::sync::Arc;

use crate::addresses::amm_addrs::uniswap_v3::ROUTER;

use super::gen::IUniswapV3Router::{self, IUniswapV3RouterInstance};

pub struct UniswapV3Router<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    router_contract: IUniswapV3RouterInstance<T, Arc<P>, N>,
}

impl<P, T, N> UniswapV3Router<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    pub fn new(provider: Arc<P>) -> Self {
        let router_contract = IUniswapV3Router::new(ROUTER, provider.clone());
        Self { router_contract }
    }


    
}
