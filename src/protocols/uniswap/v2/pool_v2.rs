use alloy::primitives::{Address, U256};
use alloy::transports::Transport;
use alloy::providers::Provider;
use alloy::network::Network;
use std::sync::Arc;

use super::gen::IUniswapV2Pool;
use crate::token::ERC20Token;

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

impl UniswapV2Pool {
    /// Create a new pool and populate it
    /// todo!() make it opetial between two tokens and address
    pub async fn new<P, T, N>(address: Address, provider: Arc<P>) -> Self 
    where 
        P: Provider<T, N>,
        T: Transport + Clone,
        N: Network,
    {
        let pool_contract = IUniswapV2Pool::new(address, provider.clone());
        let IUniswapV2Pool::token0Return { _0: token0 } = pool_contract.token0().call().await.unwrap();
        let IUniswapV2Pool::token1Return { _0: token1 } = pool_contract.token1().call().await.unwrap();

        // make the erc20 contracts to get the token names
        let token0_contract = ERC20Token::new(token0, provider.clone());
        let ERC20Token::symbolReturn { symbol: token0_symbol } = token0_contract.symbol().call().await.unwrap();
        let ERC20Token::decimalsReturn { decimals: token0_decimals } = token0_contract.decimals().call().await.unwrap();
        let token1_contract = ERC20Token::new(token1, provider.clone());
        let ERC20Token::symbolReturn { symbol: token1_symbol } = token1_contract.symbol().call().await.unwrap();
        let ERC20Token::decimalsReturn { decimals: token1_decimals } = token1_contract.decimals().call().await.unwrap();

        let IUniswapV2Pool::getReservesReturn { _reserve0, _reserve1, _blockTimestampLast } = pool_contract.getReserves().call().await.unwrap();

        Self {
            pool_address: address,
            token0_address: token0, 
            token1_address: token1,
            token0_symbol,
            token1_symbol,
            token0_decimals,
            token1_decimals,
            token0_reserves: _reserve0,
            token1_reserves: _reserve1,
        }
    }

    /// Return the address of the pool
    pub fn address(&self) -> Address {
        self.pool_address
    }

    pub fn token0_address(&self) -> Address {
        self.token0_address
    }

    pub fn token1_address(&self) -> Address {
        self.token1_address
    }

    pub fn token0_symbol(&self) -> &str {
        self.token0_symbol.as_str()
    }

    pub fn token1_symbol(&self) -> &str {
        self.token1_symbol.as_str()
    }

    pub fn token0_decimals(&self) -> u8 {
        self.token0_decimals
    }

    pub fn token1_decimals(&self) -> u8 {
        self.token1_decimals
    }

    pub fn token0_reserves(&self) -> U256 {
        U256::from(self.token0_reserves)
    }

    pub fn token1_reserves(&self) -> U256 {
        U256::from(self.token1_reserves)
    }
}