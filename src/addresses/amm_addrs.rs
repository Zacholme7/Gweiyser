use alloy::primitives::{address, Address};

pub mod uniswap_v2 {
    use super::{address, Address};
    pub const FACTORY: Address = address!("5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");
    pub const ROUTER: Address = address!("7a250d5630B4cF539739dF2C5dAcb4c659F2488D");
}

pub mod uniswap_v3 {
    use super::{address, Address};
    pub const FACTORY: Address = address!("1F98431c8aD98523631AE4a59f267346ea31F984");
    pub const ROUTER: Address = address!("68b3465833fb72A70ecDF485E0e4C7bD8665Fc45");
    pub const QUOTER: Address = address!("61fFE014bA17989E743c5F6cB21bF9697530B21e");
    pub const TICK_LEN: Address = address!("bfd8137f7d1516D3ea5cA83523914859ec47F573");
}

