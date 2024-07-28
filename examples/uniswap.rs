use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use alloy::primitives::address;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use anyhow::Result;
use gweiyser::util::ONE_ETH;
use std::sync::Arc;

use gweiyser::addresses::tokens::ethereum_tokens::{DAI, USDC, WETH};
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
            .on_http(anvil.endpoint_url()),
    );

    // instantiate gweiyser with provider
    let gweiyser = Gweiyser::new(provider.clone());

    // V2
    let factory_v2 = gweiyser.uniswap_v2_factory();
    let weth_usdc_v2_address = factory_v2.get_pair(&WETH, &USDC).await;
    println!("Uniswap v2 weth usdc pool: {}", weth_usdc_v2_address);

    let weth_usdc_v2_pool = gweiyser.uniswap_v2_pool(weth_usdc_v2_address).await;
    println!("Uniswap v2 weth usdc pool {:#?}", weth_usdc_v2_pool);

    // V3
    let factory = gweiyser.uniswap_v3_factory();
    let weth_usdc_v3_address = factory.get_pool(&WETH, &USDC, 3000).await;
    println!(
        "Uniswap v3 weth usdc pool address {:?}",
        weth_usdc_v3_address
    );

    let weth_usdc_v3_pool = gweiyser.uniswap_v3_pool(weth_usdc_v3_address).await;
    println!("Uniswap v3 weth usdc pool {:#?}", weth_usdc_v3_pool);

    // simulate swap out
    let dai_out = weth_usdc_v3_pool.get_amount_out(WETH, ONE_ETH)?;
    println!("UniswapV3 swap 1 weth for {} dai", dai_out);

    // get the current price
    let current_price = weth_usdc_v3_pool.get_price(WETH)?;
    println!("Current price {}", current_price);

    Ok(())
}
