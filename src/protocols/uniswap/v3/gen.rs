use alloy::sol;

sol!(
    #[derive(Debug)]
    #[sol(rpc)]
    IUniswapV3Factory,
    "src/protocols/uniswap/v3/abis/UniswapV3Factory.json"
);

sol!(
    #[derive(Debug)]
    #[sol(rpc)]
    IUniswapV3Pool,
    "src/protocols/uniswap/v3/abis/UniswapV3Pool.json"
);

sol!(
    #[derive(Debug)]
    #[sol(rpc)]
    ITickLens,
    "src/protocols/uniswap/v3/abis/UniswapV3TickLens.json"
);

sol!(
    #[derive(Debug)]
    #[sol(rpc)]
    IQuoter,
    "src/protocols/uniswap/v3/abis/UniswapV3Quoter.json"
);

sol!(
    #[derive(Debug)]
    #[sol(rpc)]
    IUniswapV3Router,
    "src/protocols/uniswap/v3/abis/UniswapV3Router.json"
);

