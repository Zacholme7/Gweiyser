use alloy::signers::local::PrivateKeySigner;
use alloy::providers::ProviderBuilder;
use alloy::network::EthereumWallet;
use alloy::node_bindings::Anvil;
use anyhow::Result;
use std::sync::Arc;

use gweiyser::addresses::tokens::ethereum_tokens::WETH;
use gweiyser::util::ONE_ETH;
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

    // Construct new tokens and query general information
    // The info will be automatically populated via the provider upon construciton
    let weth = gweiyser.token(WETH).await;
    let weth_decimals = weth.decimals();
    let weth_symbol = weth.symbol();
    println!("{}: {} decimals", weth_symbol, weth_decimals);

    // get the total supply
    let total_supply = weth.total_supply().await;
    println!("total supply: {}", total_supply);

    // do some state changing transactions
    // approve an account and then get the balance
    let main_account = anvil.addresses()[0];
    let other_account = anvil.addresses()[1];
    weth.approve(other_account, ONE_ETH).await;
    let allowance = weth.allowance(main_account, other_account).await;
    println!("allowance: {}", allowance);

    // deposit some weth and then query balance
    weth.deposit(ONE_ETH).await;
    let balance = weth.balance_of(main_account).await;
    println!("balance: {}", balance);




    Ok(())
}

