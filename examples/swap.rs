use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use alloy::primitives::{address, U256};
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use anyhow::Result;
use std::sync::Arc;

use gweiyser::addresses::amms::uniswap_v3::base::ROUTER;
use gweiyser::addresses::tokens::base_tokens::{BRETT, WETH};
use gweiyser::util::ONE_ETH;
use gweiyser::{Chain, Gweiyser};

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://base-rpc.publicnode.com";

    let anvil = Anvil::new().fork(url).port(8500_u16).try_spawn()?;
    let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
    println!("{:?}", anvil.endpoint());
    let wallet = EthereumWallet::new(signer);
    let provider = Arc::new(
        ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http("http://localhost:8500".parse().unwrap()),
    );

    let gweiyser = Gweiyser::new(provider.clone(), Chain::Base);

    // give the account some weth
    let weth = gweiyser.token(WETH).await;
    weth.deposit(ONE_ETH).await;
    weth.approve(ROUTER, U256::from(5e18)).await;

    let v3_router = gweiyser.uniswap_v3_router().await;

    let out = v3_router
        .exact_input_single(U256::from(1e17), WETH, BRETT, 3000, anvil.addresses()[0])
        .await;

    println!("{:?}", out);
    Ok(())
}
