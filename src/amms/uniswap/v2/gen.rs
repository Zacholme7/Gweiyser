use alloy::sol;

sol!(
    #[sol(rpc)]
    IUniswapV2Factory,
    "src/amms/uniswap/v2/abis/UniswapV2Factory.json"
);

sol!(
    #[sol(rpc)]
    IUniswapV2Pool,
    "src/amms/uniswap/v2/abis/UniswapV2Pool.json"
);

sol!(
    #[sol(rpc)]
    IUniswapV2Router,
    "src/amms/uniswap/v2/abis/UniswapV2Router.json"
);
