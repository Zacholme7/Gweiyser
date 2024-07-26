use super::gen::{IUniswapV2Factory, IUniswapV2Factory::IUniswapV2FactoryInstance};
use super::pool_v2::UniswapV2Pool;
use crate::amms::uniswap::v2::factory_v2;
use crate::util::{ArcHttpProvider, ArcWsProvider, HttpTransport};
use crate::Token;
use alloy::primitives::Address;
use std::cmp::Ordering;

pub struct UniswapV2Factory {
    factory_contract: IUniswapV2FactoryInstance<HttpTransport, ArcHttpProvider>,
    http: ArcHttpProvider,
    ws: ArcWsProvider,
}

impl UniswapV2Factory {
    /* 
    /// Creates an instance of a new UniswapV2 factory
    pub fn new(http: ArcHttpProvider, ws: ArcWsProvider) -> Self {
        let factory_address = factory_v2::address();
        let factory_contract = IUniswapV2Factory::new(factory_address, http.clone());
        Self {
            factory_contract,
            http,
            ws,
        }
    }

    /// Sees if the a pool for the pair exists, if so create and return one
    pub async fn query_for_pool(&self, base: &Token, quote: &Token) -> UniswapV2Pool {
        let (token0, token1) = match base.address().cmp(&quote.address()) {
            Ordering::Less | Ordering::Equal => (base, quote),
            Ordering::Greater => (quote, base),
        };

        let IUniswapV2Factory::getPairReturn { _0 } = self
            .factory_contract
            .getPair(token0.address(), token1.address())
            .call()
            .await
            .unwrap();

        if _0 == Address::ZERO {
            // the pool does not exist
            todo!()
        } else {
            let mut pool = UniswapV2Pool::new(_0, token0, token1, self.http.clone());
            pool.sync_reserves(self.ws.clone());
            pool
        }
    }
    */
}

