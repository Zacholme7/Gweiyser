use alloy::primitives::Address;
use alloy_sol_types::abi::token;
use crate::Token;



#[derive(Default, Debug)]
pub struct UniswapV2Pool {
        pool_address: Address,
        token0_address: Address,
        token1_address: Address,
        token0_symbol: String,
        token1_symbol: String,
        token0_decimals: u8,
        token1_decimals: u8,
        token0_reserves: u128,
        token1_reserves: u128,
}



// todo!() start a task with a provider that is constantly syncing reserves
impl UniswapV2Pool {
        /// Construct a new UniswapV2 pool from the pool address and token information
        pub fn new(pool: Address, token0: &Token, token1: &Token) -> Self {
                Self {
                        pool_address: pool,
                        token0_address: token0.address(),
                        token1_address: token1.address(),
                        token0_symbol: token0.symbol().to_string(),
                        token1_symbol: token1.symbol().to_string(),
                        token0_decimals: token0.decimals(),
                        token1_decimals: token1.decimals(),
                        ..Default::default()
                }
        }

        /// Return the address of the pool
        pub fn address(&self) -> Address {
                self.pool_address
        }
}