pub use addresses::tokens::Token;
/// All definitions and functions pertaining to AMM protocols
pub mod amms;
/// Declarations of most used addresses
pub mod addresses;




use alloy::providers::{ProviderBuilder, Provider};



pub struct Gweisyer {
        url: String
}


impl Gweisyer {
        /// Construct a new Gweisyer
        /// will instantiate a new provider with the provided rpc url
        pub fn new(url: impl Into<String> ) -> Self {
                Self {
                        url: url.into()
                }
        }

}
