use alloy::primitives::{Address, U256};
use alloy::transports::Transport;
use alloy::providers::Provider;
use alloy::network::Network;
use std::sync::Arc;

use super::gen::IUniswapV2Router::{self, IUniswapV2RouterInstance};
use crate::addresses::amm_addrs::uniswap_v2::ROUTER;

pub struct UniswapV2Router<P, T, N> 
where 
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    router_contract: IUniswapV2RouterInstance<T, Arc<P>, N>,
}


impl<P, T, N> UniswapV2Router<P, T, N> 
where 
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    /// Creates an instance of a new UniswapV2 router
    pub fn new(provider: Arc<P>) -> Self {
        let router_contract = IUniswapV2Router::new(ROUTER, provider.clone());
        Self {
            router_contract,
        }
    }

    /// Get amounts out
    pub async fn get_amounts_out(&self, amount_in: U256, path: &Vec<Address>) -> Vec<U256> {
        let IUniswapV2Router::getAmountsOutReturn { amounts } = self.router_contract.getAmountsOut(amount_in, path.clone()).call().await.unwrap();
        amounts
    }
}
