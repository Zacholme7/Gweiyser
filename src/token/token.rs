use super::abi::{ERC20Token, ERC20Token::ERC20TokenInstance};
use alloy::network::Network;
use alloy::primitives::{Address, U256};
use alloy::providers::Provider;
use alloy::transports::Transport;
use std::sync::Arc;

use crate::addresses::token_addrs::ethereum_tokens::WETH;

pub struct Token<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    token_contract: ERC20TokenInstance<T, Arc<P>, N>,
    address: Address,
    symbol: String,
    decimals: u8,
}

impl<P, T, N> Token<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    /// Given the token address, fetches the corresponding information and returns
    /// a populated instance of a new token
    pub async fn new(address: Address, provider: Arc<P>) -> Self {
        let token_contract = ERC20Token::new(address, provider);
        let ERC20Token::symbolReturn { symbol } = token_contract.symbol().call().await.unwrap();
        let ERC20Token::decimalsReturn { decimals } =
            token_contract.decimals().call().await.unwrap();
        Self {
            token_contract,
            address,
            symbol,
            decimals,
        }
    }

    /// Returns the address of the token
    pub fn address(&self) -> Address {
        self.address
    }

    /// Returns the symbol of the token
    pub fn symbol(&self) -> String {
        self.symbol.clone()
    }

    // often want to do maths with decimals, so 256 ret is a convenience thing :)
    /// Returns the precision of the token
    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    // Given an address, returns the amount of tokens that address has in the raw value
    pub async fn balance_of(&self, address: Address) -> U256 {
        let ERC20Token::balanceOfReturn { balance } =
            self.token_contract.balanceOf(address).call().await.unwrap();
        balance
    }

    // Given an address, returns the amount of tokens that address has in the normalized  value
    pub async fn balance_of_normalized(&self, address: Address) -> U256 {
        let ERC20Token::balanceOfReturn { balance } =
            self.token_contract.balanceOf(address).call().await.unwrap();
        balance / U256::from(10u128.pow(self.decimals as u32))
    }

    /// Get the total supply of the token
    pub async fn total_supply(&self) -> U256 {
        let ERC20Token::totalSupplyReturn { totalSupply } =
            self.token_contract.totalSupply().call().await.unwrap();
        totalSupply
    }

    // Approve an address to spend a certain amount of tokens
    pub async fn approve(&self, spender: Address, amount: U256) {
        let _ = self
            .token_contract
            .approve(spender, amount)
            .send()
            .await
            .unwrap();
    }

    /// Get the allowance of an address
    pub async fn allowance(&self, owner: Address, spender: Address) -> U256 {
        let ERC20Token::allowanceReturn { allowance } = self
            .token_contract
            .allowance(owner, spender)
            .call()
            .await
            .unwrap();
        allowance
    }

    // swap from eth into token
    pub async fn deposit(&self, amount: U256) {
        let _ = self
            .token_contract
            .deposit()
            .value(amount)
            .send()
            .await
            .unwrap();
    }

    pub async fn transfer_from(&self, from: Address, to: Address, amount: U256) {
        let _ = self
            .token_contract
            .transferFrom(from, to, amount)
            .send()
            .await
            .unwrap();
    }
}
