use alloy::primitives::Address;
use std::sync::Arc;
use alloy::providers::RootProvider;
use alloy::transports::http::{Http, Client};

use crate::token::abi::ERC20Token;


/// Representation of a token
pub struct Token{
        pub symbol: String,
        pub decimals: u8,
}


impl Token {
        /// Given the token address, fetches the corresponding information and returns
        /// a populated instance of a new token
        pub async fn new(address: Address, provider: Arc<RootProvider<Http<Client>>>) -> Self {
                let token_contract = ERC20Token::new(address, provider);
                let ERC20Token::symbolReturn { symbol } = token_contract.symbol().call().await.unwrap();
                let ERC20Token::decimalsReturn { decimals } = token_contract.decimals().call().await.unwrap();
                Self {
                        symbol,
                        decimals
                }
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


