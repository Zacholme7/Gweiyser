use alloy::signers::local::PrivateKeySigner;
use alloy::providers::ProviderBuilder;
use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use alloy::primitives::address;
use anyhow::Result;
use std::sync::Arc;

use gweiyser::addresses::tokens::ethereum_tokens::{WETH, USDC};
use gweiyser::protocols::uniswap::v2::pool_v2::UniswapV2Pool;
use gweiyser::Gweiyser;


#[tokio::main]
async fn main() -> Result<()> {
    // Construct provider and anvil instance
    let url = "https://eth.merkle.io";
    let anvil = Anvil::new().fork(url).try_spawn()?;
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    let wallet = EthereumWallet::new(signer);
    let provider = Arc::new(
        ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet)
        .on_http(anvil.endpoint_url())
    );

    // instantiate gweiyser with provider
    let gweiyser = Gweiyser::new(provider.clone());

    let weth_usdc_v2 = address!("B4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc");
    let pool = gweiyser.uniswap_v2_pool(weth_usdc_v2).await;

    println!("Address {}", pool.address());
    println!("Token0 {}", pool.token0_address());
    println!("Token1 {}", pool.token1_address());
    println!("Token0 symbol {}", pool.token0_symbol());
    println!("Token1 symbol {}", pool.token1_symbol());
    println!("Token0 decimals {}", pool.token0_decimals());
    println!("Token1 decimals {}", pool.token1_decimals());

    Ok(())
}
