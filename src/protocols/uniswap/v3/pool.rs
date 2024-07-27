use alloy::primitives::{Address, I128, U128, U256};
use alloy::transports::Transport;
use alloy::providers::Provider;
use std::collections::HashMap;
use alloy::network::Network;
use std::sync::Arc;

use super::gen::{IUniswapV3Pool, IUniswapV3Pool::IUniswapV3PoolInstance};



pub struct UniswapV3Pool<P, T, N> 
where 
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    pool_contract: IUniswapV3PoolInstance<T, Arc<P>, N>,
    address: Address,
    token0: Address,
    token1: Address,
    token0_decimals: u8,
    token1_decimals: u8,
    liquidity: U128,
    sqrt_price: U256,
    tick: i32,
    fee: u32,
    tick_spacing: u32,
    tick_bitmap: HashMap<i16, U256>,
    ticks: HashMap<i32, TickInfo>
}

pub struct TickInfo {
    liquidity_gross: U128,
    liquidity_net: I128,
    initialized: bool,
}


/* 
impl<P, T, N> UniswapV3Pool<P, T, N> 
where 
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{


    pub fn new(address: Address, provider: Arc<P>) -> Self {
        let pool_contract = IUniswapV3Pool::new(address, provider.clone());
        
        Self {
            pool_contract,
            address,
        }
    }



    pub fn get_tick_spacing(&self) {
        todo!()
    }




}
    */