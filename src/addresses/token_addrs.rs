use alloy::primitives::{address, Address};

/// List of commonly used token addresses for Ethereum
pub mod ethereum_tokens {
    use super::*;
    pub const WETH: Address = address!("C02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2");
    pub const USDC: Address = address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
    pub const USDT: Address = address!("dAC17F958D2ee523a2206206994597C13D831ec7");
    pub const DAI: Address = address!("6B175474E89094C44Da98b954EedeAC495271d0F");
    pub const UNI: Address = address!("1f9840a85d5aF5bf1D1762F925BDADdC4201F984");
}

/// List of commonly used token addresses for Base
pub mod base_tokens {
    use super::*;
    pub const WETH: Address = address!("4200000000000000000000000000000000000006");
    pub const BRETT: Address = address!("532f27101965dd16442E59d40670FaF5eBB142E4");
}
