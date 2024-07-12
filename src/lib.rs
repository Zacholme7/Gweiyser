// reexports
pub use token::Token;

// modules defines
pub mod amms;
pub mod addresses;
pub mod token;

use alloy::providers::{ProviderBuilder, RootProvider};
use alloy::transports::http::{Http, Client};
use alloy::transports::http::reqwest::Url;
use alloy::primitives::Address;
use std::sync::Arc;

pub struct Gweisyer {
        provider: Arc<RootProvider<Http<Client>>>
}


impl Gweisyer {
        /// Construct a new Gweisyer
        /// will instantiate a new provider with the provided rpc url
        pub fn new(url: impl Into<String> ) -> Self {
                let url: Url = url.into().parse().unwrap();
                let provider = Arc::new(ProviderBuilder::new()
                        .on_http(url));

                Self {
                        provider
                }
        }

        // Construct an new toke
        pub async fn token(&self, address: Address) -> Token {
                // Just delegate the call to the token object
                let token = Token::new(address, self.provider.clone()).await;
                token
        }

}
