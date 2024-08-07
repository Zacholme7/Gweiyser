use alloy::network::Network;
use alloy::primitives::Address;
use alloy::providers::Provider;
use alloy::transports::Transport;
use std::sync::Arc;

use super::gen::ITickLens::{self, ITickLensInstance};
use crate::addresses::amm_addrs::uniswap_v3::{base, ethereum};
use crate::Chain;

pub struct TickLens<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    tick_lens_contract: ITickLensInstance<T, Arc<P>, N>,
}

#[derive(Debug, Clone)]
pub struct PopulatedTick {
    pub tick: i32,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,
}

impl<P, T, N> TickLens<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    /// Create a new tick lens contract
    pub fn new(provider: Arc<P>, chain: Chain) -> Self {
        let addr = if chain == Chain::Ethereum {
            ethereum::TICK_LEN
        } else {
            base::TICK_LEN
        };

        let tick_lens_contract = ITickLens::new(addr, provider.clone());
        Self { tick_lens_contract }
    }

    pub async fn get_populated_ticks_in_word(
        &self,
        pool: Address,
        tick_bitmap_index: i16,
    ) -> Vec<PopulatedTick> {
        let ITickLens::getPopulatedTicksInWordReturn { populatedTicks } = self
            .tick_lens_contract
            .getPopulatedTicksInWord(pool, tick_bitmap_index)
            .call()
            .await
            .unwrap();

        populatedTicks
            .into_iter()
            .map(|tick| PopulatedTick {
                tick: tick.tick,
                liquidity_net: tick.liquidityNet,
                liquidity_gross: tick.liquidityGross,
            })
            .collect()
    }
}
