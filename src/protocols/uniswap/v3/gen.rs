use alloy::sol;


sol!(
    #[sol(rpc)]
    IUniswapV3Factory,
    "src/protocols/uniswap/v3/abis/UniswapV3Factory.json"
);

sol!(
    #[sol(rpc)]
    IUniswapV3Pool,
    "src/protocols/uniswap/v3/abis/UniswapV3Pool.json"
);

sol!( 
    #[sol(rpc)]
    ITickLens,
    "src/protocols/uniswap/v3/abis/UniswapV3TickLens.json"
);
