use alloy::transports::Transport;
use alloy::primitives::Address;
use alloy::providers::Provider;
use std::marker::PhantomData;
use alloy::network::Network;
use std::sync::Arc;

use crate::token::Token;

// modules defines
mod protocols;
mod token;
mod sync_pools;
pub mod util;
pub mod addresses;



pub struct Gweisyer<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    http: Arc<P>,
    _phantom: PhantomData<(T, N)>,
}

impl<P, T, N> Gweisyer<P, T, N>
where
    P: Provider<T, N>,
    T: Transport + Clone,
    N: Network,
{
    /// Construct a new Gweisyer
    pub fn new(http_provider: Arc<P>) -> Self {
        Self {
            http: http_provider,
            _phantom: PhantomData,
        }
    }

    // Construct an new token
    pub async fn token(&self, address: Address) -> Token<P, T, N> {
        // Just delegate the call to the token object
        let token = Token::new(address, self.http.clone()).await;
        token
    }

    // Construct a router



 }



