use super::r#gen::IUniswapV2Router::{self, IUniswapV2RouterInstance};



/* 

pub struct UniswapV2Router {
    router_contract: IUniswapV2RouterInstance<HttpTransport, ArcHttpProvider>,
    http: ArcHttpProvider,
    ws: ArcWsProvider,
}


impl UniswapV2Router {
    /// Creates an instance of a new UniswapV2 router
    pub fn new(http: ArcHttpProvider, ws: ArcWsProvider) -> Self {
        let router_contract = IUniswapV2Router::new(router, http.clone());
        Self {
            router_contract,
            http,
            ws,
        }
    }

    /// Get amounts out
    pub fn get_amounts_out(&self, amount_in: u128, path: &[Address]) -> Vec<U256> {
        let IUniswapV2Router::getAmountsOutReturn { amounts } = self.router_contract.getAmountsOut(amount_in, path).call().await.unwrap();
        amounts
    }

    pub fn swap_exact_tokens_for_tokens(&self, amount_in: u128, amount_out_min: u128, path: &[Address], to: Address, deadline: u128) -> bool {
        let IUniswapV2Router::swapExactTokensForTokensReturn { success } = self.router_contract.swapExactTokensForTokens(amount_in, amount_out_min, path, to, deadline).call().await.unwrap();
        success
    }



}
    */
