use gweiyser::Token;
use gweiyser::Gweisyer;
use gweiyser::addresses::tokens::etherum_tokens::{WETH, USDC};

fn main() {
        // instantiate gweiyser with an rpc url
        // will construct a provider to fetch relevant info
        let gweiyser = Gweisyer::new("https://eth.merkle.io");

        // Construct new tokens and query general information
        // The info will be automatically populated via the provider upon construciton
        let weth = gweiyser.token(WETH);
        let weth_decimals = weth.decimals();
        let weth_symbol = weth.symbol();

        let usdc = gweiyser.token(USDC);
        let usdc_decimals = usdc.decimals();
        let usdc_symbol = usdc.symbol();

        // Attempt to construct a uniswapV2 pool wtih the two tokens
        // construciton order does not matter, these are the same pool
        let weth_usdc_v2 = gweisyer.uniswapV2(weth, usdc)?;
        let usdc_weth_v2 = gweisyer.uniswapV2(usdc, weth)?;
        assert!(weth_usdc_v2.address() == usdc_weth_v2.address());

        // simulate a swap
        // reserve querying and unit conversion will happen beind the scenes
        // this means that you can input a swap amount in human readable amount
        // (amount in of base, base token, quote token)
        let usdc_out = weth_usdc_v2.get_amount_out(2, weth, usdc); // simulates a swap from 2 weth to usdc
        let weth_out = weth_usdc_v2.get_amount_out(10_000, usdc, weth); // simualtes a swap from 10000 usdc to weth


        println!("blah blah");
}