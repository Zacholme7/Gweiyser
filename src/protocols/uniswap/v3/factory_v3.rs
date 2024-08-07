use alloy::network::Network;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::transports::Transport;
use std::sync::Arc;

use super::gen::IUniswapV3Factory::{self, IUniswapV3FactoryInstance};
use crate::addresses::amm_addrs::uniswap_v3::{base, ethereum};
use crate::Chain;

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
    pub fn new(provider: Arc<P>, chain: Chain) -> Self {
        let addr = if chain == Chain::Ethereum {
            ethereum::FACTORY
        } else {
            base::FACTORY
        };

        let factory_contract = IUniswapV3Factory::new(addr, provider.clone());
        Self { factory_contract }
    }

    // get the addrs of a pool if it exists, else the zero address
    pub async fn get_pool(&self, token0: &Address, token1: &Address, fee: u32) -> Address {
        let IUniswapV3Factory::getPoolReturn { _0 } = self
            .factory_contract
            .getPool(*token0, *token1, fee)
            .call()
            .await
            .unwrap();
        _0
    }
}
