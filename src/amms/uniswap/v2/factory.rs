use std::cmp::Ordering;
use alloy::primitives::Address;
use crate::util::{ArcHttpProvider, HttpTransport};
use crate::addresses::amm_addrs::uniswap_v2::factory;
use crate::Token;
use super::gen::{IUniswapV2Factory, IUniswapV2Factory::IUniswapV2FactoryInstance};
use super::pool::UniswapV2Pool;


pub struct UniswapV2Factory {
        factory_contract: IUniswapV2FactoryInstance<HttpTransport, ArcHttpProvider>,
}


impl UniswapV2Factory {
        /// Creates an instance of a new UniswapV2 factory
        pub fn new(provider: ArcHttpProvider) -> Self {
                let factory_contract = IUniswapV2Factory::new(factory, provider);
                Self  {
                        factory_contract
                }
        }

        /// Sees if the a pool for the pair exists, if so create and return one
        pub async fn query_for_pool(&self, base: &Token, quote: &Token) -> UniswapV2Pool {
                let (token0, token1) = match base.address().cmp(&quote.address()) {
                        Ordering::Less | Ordering::Equal => (base, quote),
                        Ordering::Greater => (quote, base)
                };

                let IUniswapV2Factory::getPairReturn { _0 } = self.factory_contract.getPair(token0.address(), token1.address()).call().await.unwrap();

                if _0 == Address::ZERO {
                        // the pool does not exist
                        todo!()
                } else {
                        UniswapV2Pool::new(_0, token0, token1)
                }
        }


}