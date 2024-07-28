use alloy::network::Network;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::transports::Transport;
use std::sync::Arc;

use crate::addresses::amm_addrs::uniswap_v3::FACTORY;

use super::gen::IUniswapV3Factory::{self, IUniswapV3FactoryInstance};

pub struct UniswapV3Factory<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    factory_contract: IUniswapV3FactoryInstance<T, Arc<P>, N>,
}

impl<P, T, N> UniswapV3Factory<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    pub fn new(provider: Arc<P>) -> Self {
        let factory_contract = IUniswapV3Factory::new(FACTORY, provider.clone());
        Self { factory_contract }
    }

    // get the addrs of a pool if it exists, else the zero address
    pub async fn get_pool(&self, token0: &Address, token1: &Address, fee: u32) -> Address {
        let IUniswapV3Factory::getPoolReturn { _0 } = self.factory_contract.getPool(*token0, *token1, fee).call().await.unwrap();
        _0 
    }

}
