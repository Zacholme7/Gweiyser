use alloy::signers::local::PrivateKeySigner;
use alloy::providers::ProviderBuilder;
use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use anyhow::Result;
use std::sync::Arc;

use gweiyser::addresses::tokens::ethereum_tokens::{WETH, USDC};

use gweiyser::Gweisyer;


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
    let gweiyser = Gweisyer::new(provider.clone());
    Ok(())
}
