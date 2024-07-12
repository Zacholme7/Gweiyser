use alloy::primitives::{Address, U256};
use std::sync::Arc;
use alloy::providers::RootProvider;
use alloy::transports::http::{Http, Client};

use crate::token::abi::ERC20Token;
use crate::util::{ArcHttpProvider, HttpTransport};


/// Representation of a token
pub struct Token{
        token_contract: ERC20Token::ERC20TokenInstance<HttpTransport, ArcHttpProvider>,
        symbol: String,
        decimals: u8,
}


impl Token {
        /// Given the token address, fetches the corresponding information and returns
        /// a populated instance of a new token
        pub async fn new(address: Address, provider: ArcHttpProvider) -> Self {
                let token_contract = ERC20Token::new(address, provider);
                let ERC20Token::symbolReturn { symbol } = token_contract.symbol().call().await.unwrap();
                let ERC20Token::decimalsReturn { decimals } = token_contract.decimals().call().await.unwrap();
                Self {
                        token_contract,
                        symbol,
                        decimals
                }
        }

        // Given an address, returns the amount of tokens that address has in the raw value
        pub async fn balance_of(&self, address: Address) -> U256 {
                let ERC20Token::balanceOfReturn { balance} = self.token_contract.balanceOf(address).call().await.unwrap();
                balance
        }

        // Given an address, returns the amount of tokens that address has in the normalized  value
        pub async fn balance_of_normalized(&self, address: Address) -> U256 {
                let ERC20Token::balanceOfReturn { balance} = self.token_contract.balanceOf(address).call().await.unwrap();
                balance / U256::from(10u128.pow(self.decimals as u32))
        }

        /// Returns the precision of the token
        pub fn decimals(&self) -> u8 {
                self.decimals
        }

        /// Returns the symbol of the token
        pub fn symbol(&self) -> &str {
                self.symbol.as_str()
        }
}