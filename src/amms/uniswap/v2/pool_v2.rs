use super::gen::IUniswapV2Pool;
use crate::util::{ArcHttpProvider, ArcWsProvider};
use crate::Token;
use alloy::primitives::{Address, U256};
use alloy::rpc::types::Filter;
use alloy_sol_types::{SolCall, SolEvent, SolEventInterface};
use futures::StreamExt;

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

/* 
// todo!() start a task with a provider that is constantly syncing reserves
impl UniswapV2Pool {
    /// Construct a new UniswapV2 pool from the pool address and token information
    pub fn new(pool: Address, token0: &Token, token1: &Token, provider: ArcHttpProvider) -> Self {
        // initial pool reserve

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

    pub fn sync_reserves(&mut self, provider: ArcWsProvider) {
        todo!()
        /*
        let filter = Filter::new()
                .event_signature(IUniswapV2Pool::Sync::SIGNATURE_HASH);

        let poller = provider.subscribe_logs(&filter).await.unwrap();
        let mut stream = poller.into_stream();
        while let Some(log) = stream.next().await {
                let res = IUniswapV2Pool::Sync::decode_log(log, false).unwrap();
                println!("new log: {res:#?}");
        }
        */
    }

    /// Return the address of the pool
    pub fn address(&self) -> Address {
        self.pool_address
    }
    /// Given a amount in, calculate the amount out
    pub fn get_amount_out(&self, amount_in: u128, base: &Token, quote: &Token) -> U256 {
        // dy = (y0 * dx * (1- fee)) / (x0 + dx * (1 - fee))
        let amount_in_256 = U256::from(amount_in);
        let (base_reserves, quote_reserves) = if base.address() == self.token0_address {
            (
                U256::from(self.token0_reserves),
                U256::from(self.token1_reserves),
            )
        } else {
            (
                U256::from(self.token1_reserves),
                U256::from(self.token0_reserves),
            )
        };

        // calculation constants
        let fee = U256::from(997);
        let scale = U256::from(1000);
        (quote_reserves * amount_in_256 * fee) / (base_reserves * scale + amount_in_256)
    }
}*/

