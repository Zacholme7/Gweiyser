use alloy::network::Network;
use alloy::primitives::{Address, I128, I16, I256, I32, U128, U16, U256};
use alloy::providers::Provider;
use alloy::transports::Transport;
use anyhow::Result;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;
use uniswap_v3_math::tick_math::{MAX_SQRT_RATIO, MAX_TICK, MIN_SQRT_RATIO, MIN_TICK};

use super::gen::{IUniswapV3Pool, IUniswapV3Pool::IUniswapV3PoolInstance};
use crate::Token;
pub const U256_1: U256 = U256::from_limbs([1, 0, 0, 0]);

#[derive(Debug)]
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
    token0_symbol: String,
    token1_symbol: String,
    liquidity: u128,
    sqrt_price_x96: U256,
    tick: i32,
    fee: u32,
    tick_spacing: i32,
    max_liquidity_per_tick: U128,
    tick_bitmap: HashMap<i16, U256>,
    ticks: HashMap<i32, TickInfo>,
}

#[derive(Debug)]
pub struct TickInfo {
    liquidity_gross: u128,
    liquidity_net: i128,
    initialized: bool,
}

pub struct CurrentState {
    amount_specified_remaining: I256,
    amount_calculated: I256,
    sqrt_price_x_96: U256,
    tick: i32,
    liquidity: u128,
}

#[derive(Default)]
pub struct StepComputations {
    pub sqrt_price_start_x_96: U256,
    pub tick_next: i32,
    pub initialized: bool,
    pub sqrt_price_next_x96: U256,
    pub amount_in: U256,
    pub amount_out: U256,
    pub fee_amount: U256,
}

impl<P, T, N> UniswapV3Pool<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    /// Construct a new pool
    pub async fn new(address: Address, provider: Arc<P>) -> Self {
        let pool_contract = IUniswapV3Pool::new(address, provider.clone());

        // get the token addresses
        let IUniswapV3Pool::token0Return { _0: token0_address } =
            pool_contract.token0().call().await.unwrap();
        let IUniswapV3Pool::token1Return { _0: token1_address } =
            pool_contract.token1().call().await.unwrap();

        // fetch the token info
        let token0 = Token::new(token0_address, provider.clone()).await;
        let token0_symbol = token0.symbol();
        let token0_decimals = token0.decimals();
        let token1 = Token::new(token1_address, provider.clone()).await;
        let token1_symbol = token1.symbol();
        let token1_decimals = token1.decimals() as u8;

        // fetch rest of pool info
        let IUniswapV3Pool::liquidityReturn { _0: liquidity } =
            pool_contract.liquidity().call().await.unwrap();
        let IUniswapV3Pool::slot0Return {
            sqrtPriceX96, tick, ..
        } = pool_contract.slot0().call().await.unwrap();
        let IUniswapV3Pool::feeReturn { _0: fee } = pool_contract.fee().call().await.unwrap();
        let IUniswapV3Pool::tickSpacingReturn { _0: tick_spacing } =
            pool_contract.tickSpacing().call().await.unwrap();
        let IUniswapV3Pool::maxLiquidityPerTickReturn {
            _0: max_liquidity_per_tick,
        } = pool_contract.maxLiquidityPerTick().call().await.unwrap();

        let mut pool = Self {
            pool_contract,
            address,
            token0: token0_address,
            token1: token1_address,
            token0_decimals,
            token1_decimals,
            token0_symbol,
            token1_symbol,
            liquidity,
            sqrt_price_x96: sqrtPriceX96,
            tick,
            fee,
            tick_spacing,
            max_liquidity_per_tick: U128::from(max_liquidity_per_tick),
            tick_bitmap: HashMap::new(),
            ticks: HashMap::new(),
        };

        pool.update_tick_bitmap().await;
        pool.update_ticks().await;
        pool
    }

    pub async fn update_tick_bitmap(&mut self) {
        let word_pos = self.tick >> 8;
        for i in (word_pos - 3)..=(word_pos + 3) {
            let IUniswapV3Pool::tickBitmapReturn { _0: bitmap } = self
                .pool_contract
                .tickBitmap(i.try_into().unwrap())
                .call()
                .await
                .unwrap();
            self.tick_bitmap.insert(i.try_into().unwrap(), bitmap);
        }
    }

    pub async fn update_ticks(&mut self) {
        let lower_tick = (self.tick / self.tick_spacing - 32) * self.tick_spacing;
        let upper_tick = (self.tick / self.tick_spacing + 32) * self.tick_spacing;

        for i in (lower_tick..=upper_tick).step_by(self.tick_spacing as usize) {
            let IUniswapV3Pool::ticksReturn {
                liquidityGross,
                liquidityNet,
                initialized,
                ..
            } = self.pool_contract.ticks(i).call().await.unwrap();

            self.ticks.insert(
                i,
                TickInfo {
                    liquidity_gross: liquidityGross,
                    liquidity_net: liquidityNet,
                    initialized,
                },
            );
        }
    }

    pub fn address(&self) -> Address {
        self.address
    }

    pub fn token0(&self) -> Address {
        self.token0
    }

    pub fn token1(&self) -> Address {
        self.token1
    }

    pub fn fee(&self) -> u32 {
        self.fee
    }

    pub fn tick_spacing(&self) -> i32 {
        self.tick_spacing
    }

    pub fn max_liquidity_per_tick(&self) -> U128 {
        self.max_liquidity_per_tick
    }

    pub fn get_amount_out(&self, token_in: Address, amount_in: U256) -> Result<U256> {
        if amount_in.is_zero() {
            return Ok(U256::ZERO);
        }

        let zero_for_one = token_in == self.token0;

        // Set sqrt_price_limit_x_96 to the max or min sqrt price in the pool depending on zero_for_one
        let sqrt_price_limit_x_96 = if zero_for_one {
            MIN_SQRT_RATIO + U256_1
        } else {
            MAX_SQRT_RATIO - U256_1
        };

        // Initialize a mutable state state struct to hold the dynamic simulated state of the pool
        let mut current_state = CurrentState {
            sqrt_price_x_96: self.sqrt_price_x96, //Active price on the pool
            amount_calculated: I256::ZERO,        //Amount of token_out that has been calculated
            amount_specified_remaining: I256::from_raw(amount_in), //Amount of token_in that has not been swapped
            tick: self.tick,                                       //Current i24 tick of the pool
            liquidity: self.liquidity, //Current available liquidity in the tick range
        };

        while current_state.amount_specified_remaining != I256::ZERO
            && current_state.sqrt_price_x_96 != sqrt_price_limit_x_96
        {
            // Initialize a new step struct to hold the dynamic state of the pool at each step
            let mut step = StepComputations {
                // Set the sqrt_price_start_x_96 to the current sqrt_price_x_96
                sqrt_price_start_x_96: current_state.sqrt_price_x_96,
                ..Default::default()
            };

            // Get the next tick from the current tick
            (step.tick_next, step.initialized) =
                uniswap_v3_math::tick_bitmap::next_initialized_tick_within_one_word(
                    &self.tick_bitmap,
                    current_state.tick,
                    self.tick_spacing,
                    zero_for_one,
                )?;

            // ensure that we do not overshoot the min/max tick, as the tick bitmap is not aware of these bounds
            // Note: this could be removed as we are clamping in the batch contract
            step.tick_next = step.tick_next.clamp(MIN_TICK, MAX_TICK);

            // Get the next sqrt price from the input amount
            step.sqrt_price_next_x96 =
                uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick(step.tick_next)?;

            // Target spot price
            let swap_target_sqrt_ratio = if zero_for_one {
                if step.sqrt_price_next_x96 < sqrt_price_limit_x_96 {
                    sqrt_price_limit_x_96
                } else {
                    step.sqrt_price_next_x96
                }
            } else if step.sqrt_price_next_x96 > sqrt_price_limit_x_96 {
                sqrt_price_limit_x_96
            } else {
                step.sqrt_price_next_x96
            };

            // Compute swap step and update the current state
            (
                current_state.sqrt_price_x_96,
                step.amount_in,
                step.amount_out,
                step.fee_amount,
            ) = uniswap_v3_math::swap_math::compute_swap_step(
                current_state.sqrt_price_x_96,
                swap_target_sqrt_ratio,
                current_state.liquidity,
                current_state.amount_specified_remaining,
                self.fee,
            )?;

            // Decrement the amount remaining to be swapped and amount received from the step
            current_state.amount_specified_remaining = current_state
                .amount_specified_remaining
                .overflowing_sub(I256::from_raw(
                    step.amount_in.overflowing_add(step.fee_amount).0,
                ))
                .0;

            current_state.amount_calculated -= I256::from_raw(step.amount_out);

            // If the price moved all the way to the next price, recompute the liquidity change for the next iteration
            if current_state.sqrt_price_x_96 == step.sqrt_price_next_x96 {
                if step.initialized {
                    let mut liquidity_net = if let Some(info) = self.ticks.get(&step.tick_next) {
                        info.liquidity_net
                    } else {
                        0
                    };

                    // we are on a tick boundary, and the next tick is initialized, so we must charge a protocol fee
                    if zero_for_one {
                        liquidity_net = -liquidity_net;
                    }

                    current_state.liquidity = if liquidity_net < 0 {
                        if current_state.liquidity < (-liquidity_net as u128) {
                            return Ok(U256::ZERO); // this orignally returned an error
                        } else {
                            current_state.liquidity - (-liquidity_net as u128)
                        }
                    } else {
                        current_state.liquidity + (liquidity_net as u128)
                    };
                }
                // Increment the current tick
                current_state.tick = if zero_for_one {
                    step.tick_next.wrapping_sub(1)
                } else {
                    step.tick_next
                }
                // If the current_state sqrt price is not equal to the step sqrt price, then we are not on the same tick.
                // Update the current_state.tick to the tick at the current_state.sqrt_price_x_96
            } else if current_state.sqrt_price_x_96 != step.sqrt_price_start_x_96 {
                current_state.tick = uniswap_v3_math::tick_math::get_tick_at_sqrt_ratio(
                    current_state.sqrt_price_x_96,
                )?;
            }
        }

        let amount_out = (-current_state.amount_calculated).into_raw();

        Ok(amount_out)
    }

    pub fn get_price(&self, base_token: Address) -> Result<f64> {
        let tick = uniswap_v3_math::tick_math::get_tick_at_sqrt_ratio(self.sqrt_price_x96)?;
        let shift = self.token0_decimals as i8 - self.token1_decimals as i8;

        let price = match shift.cmp(&0) {
            Ordering::Less => 1.0001_f64.powi(tick) / 10_f64.powi(-shift as i32),
            Ordering::Greater => 1.0001_f64.powi(tick) * 10_f64.powi(shift as i32),
            Ordering::Equal => 1.0001_f64.powi(tick),
        };

        if base_token == self.token0 {
            Ok(price)
        } else {
            Ok(1.0 / price)
        }
    }
}
