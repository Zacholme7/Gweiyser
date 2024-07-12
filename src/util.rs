use alloy::network::Ethereum;
use alloy::providers::RootProvider;
use alloy::pubsub::PubSubFrontend;
use alloy::transports::http::{Client, Http};
use std::sync::Arc;

// type definitions to make life a little easier
pub type HttpTransport = Http<Client>;
pub type HttpProvider = RootProvider<HttpTransport, Ethereum>;
pub type ArcHttpProvider = Arc<HttpProvider>;
pub type ArcWsProvider = Arc<RootProvider<PubSubFrontend>>;
