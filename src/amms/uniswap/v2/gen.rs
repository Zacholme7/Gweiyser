use alloy_sol_types::{sol, SolCall};


sol!(
        #[sol(rpc)]
        IUniswapV2Factory,
        "src/amms/uniswap/v2/abis/UniswapV2Factory.json"
);
