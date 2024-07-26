use alloy::transports::Transport;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::network::Network;
use std::sync::Arc;

use super::gen::{IUniswapV2Factory, IUniswapV2Factory::IUniswapV2FactoryInstance};
use crate::addresses::amm_addrs::uniswap_v2::FACTORY;
pub struct UniswapV2Factory<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    factory_contract: IUniswapV2FactoryInstance<T, Arc<P>, N>,
}

impl<P, T, N> UniswapV2Factory<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{   
    /// Creates an instance of a new UniswapV2 factory
    pub fn new(provider: Arc<P>) -> Self {
        let factory_contract = IUniswapV2Factory::new(FACTORY, provider.clone());
        Self {
            factory_contract,
        }
    }

    /// Get the addrss of a pair if it exists, else the zero address
    pub async fn get_pair(&self, token0: &Address, token1: &Address) -> Address {
        let IUniswapV2Factory::getPairReturn { _0 } = self.factory_contract.getPair(*token0, *token1).call().await.unwrap();
        _0 
    }

    // tood!() implement the other functions that are not super useful, allPairs(), allPairsLength(), ...

}