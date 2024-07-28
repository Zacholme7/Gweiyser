use alloy::primitives::U256;

// some commonly used eth values
pub const ONE_ETH: U256 = U256::from_limbs([1_000_000_000_000_000_000u64, 0, 0, 0]);
pub const HALF_ETH: U256 = U256::from_limbs([500_000_000_000_000_000u64, 0, 0, 0]);
pub const QUARTER_ETH: U256 = U256::from_limbs([250_000_000_000_000_000u64, 0, 0, 0]);
pub const TENTH_ETH: U256 = U256::from_limbs([100_000_000_000_000_000u64, 0, 0, 0]);


