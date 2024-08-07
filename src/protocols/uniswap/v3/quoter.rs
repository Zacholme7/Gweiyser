use alloy::network::Network;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::transports::Transport;
use std::sync::Arc;

use super::gen::{IQuoter, IQuoter::IQuoterInstance};
use crate::addresses::amm_addrs::uniswap_v3::{base, ethereum};
use crate::Chain;

pub struct UniswapV3Quoter<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    quoter_contract: IQuoterInstance<T, Arc<P>, N>,
}

impl<P, T, N> UniswapV3Quoter<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    pub fn new(provider: Arc<P>, chain: Chain) -> Self {
        let addr = if chain == Chain::Ethereum {
            ethereum::QUOTER
        } else {
            base::QUOTER
        };

        let quoter_contract = IQuoter::new(addr, provider.clone());
        Self { quoter_contract }
    }

    pub async fn quote_exact_input_single(
        &self,
        amount_in: U256,
        token_in: Address,
        token_out: Address,
        fee: u32,
    ) -> U256 {
        let params = IQuoter::QuoteExactInputSingleParams {
            tokenIn: token_in,
            tokenOut: token_out,
            fee: fee,
            amountIn: amount_in,
            sqrtPriceLimitX96: U256::ZERO,
        };
        let IQuoter::quoteExactInputSingleReturn {
            amountOut,
            sqrtPriceX96After,
            initializedTicksCrossed,
            gasEstimate,
        } = self
            .quoter_contract
            .quoteExactInputSingle(params)
            .call()
            .await
            .unwrap();
        amountOut
    }
}
