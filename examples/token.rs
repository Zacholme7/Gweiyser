use alloy::network::EthereumWallet;
use alloy::signers::local::PrivateKeySigner;
use gweiyser::addresses::tokens::ethereum_tokens::{USDC, WETH};
use gweiyser::Gweisyer;
use gweiyser::Token;
use alloy::providers::ProviderBuilder;
use anyhow::Result;
use std::sync::Arc;
use alloy::node_bindings::Anvil;

#[tokio::main]
async fn main() -> Result<()> {

    // construct the provider
    let url = "https://eth.merkle.io";
    let anvil = Anvil::new()
        .fork(url)

        .try_spawn()?;
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::new(signer);
    let provider = Arc::new(ProviderBuilder::new().wallet(wallet).on_http(anvil.endpoint_url()));

    // instantiate gweiyser with provider
    let gweiyser = Gweisyer::new(provider.clone());
    // Construct new tokens and query general information
    // The info will be automatically populated via the provider upon construciton
    let weth = gweiyser.token(WETH).await;
    let weth_decimals = weth.decimals();
    let weth_symbol = weth.symbol();
    println!("{}: {} decimals", weth_symbol, weth_decimals);

    let usdc = gweiyser.token(USDC).await;
    let usdc_decimals = usdc.decimals();
    let usdc_symbol = usdc.symbol();
    println!("{}: {} decimals", usdc_symbol, usdc_decimals);
    Ok(())

    /* 
    // Attempt to construct a uniswapV2 pool wtih the two tokens
    // construciton order does not matter, these are the same pool
    let weth_usdc_v2 = gweiyser.uniswapV2(&weth, &usdc).await;
    let usdc_weth_v2 = gweiyser.uniswapV2(&usdc, &weth).await;
    println!("{:?}", usdc_weth_v2);
    assert!(weth_usdc_v2.address() == usdc_weth_v2.address());

    // simulate a swap
    // reserve querying and unit conversion will happen beind the scenes
    // this means that you can input a swap amount in human readable amount
    // (amount in of base, base token, quote token)
    let usdc_out = weth_usdc_v2.get_amount_out(2, &weth, &usdc); // simulates a swap from 2 weth to usdc
    let weth_out = weth_usdc_v2.get_amount_out(10_000, &usdc, &weth); // simualtes a swap from 10000 usdc to weth
    println!("{:?}", usdc_out);
    loop {}
    */

    /*
    // Sync all the pools
    gweiyser.sync_pools(UniswapV2, UniswapV3, Sushiswap)
    let all_weth_usdc_pools = gweisyer.get_all_pools(weth, usdc);
    let all_usdc_pools = gweiyser.get_all_poos(usdc);






    */
}

