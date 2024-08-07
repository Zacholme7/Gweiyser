use alloy::primitives::{address, Address};

pub mod uniswap_v2 {
    use super::{address, Address};

    pub mod ethereum {
        use super::{address, Address};
        pub const FACTORY: Address = address!("5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");
        pub const ROUTER: Address = address!("7a250d5630B4cF539739dF2C5dAcb4c659F2488D");
    }

    pub mod base {
        use super::{address, Address};
        pub const FACTORY: Address = address!("8909Dc15e40173Ff4699343b6eB8132c65e18eC6");
        pub const ROUTER: Address = address!("4752ba5dbc23f44d87826276bf6fd6b1c372ad24");
    }
}

pub mod uniswap_v3 {
    use super::{address, Address};

    pub mod ethereum {
        use super::{address, Address};
        pub const FACTORY: Address = address!("1F98431c8aD98523631AE4a59f267346ea31F984");
        pub const ROUTER: Address = address!("68b3465833fb72A70ecDF485E0e4C7bD8665Fc45");
        pub const QUOTER: Address = address!("61fFE014bA17989E743c5F6cB21bF9697530B21e");
        pub const TICK_LEN: Address = address!("bfd8137f7d1516D3ea5cA83523914859ec47F573");
    }

    pub mod base {
        use super::{address, Address};
        pub const FACTORY: Address = address!("33128a8fC17869897dcE68Ed026d694621f6FDfD");
        pub const ROUTER: Address = address!("2626664c2603336E57B271c5C0b26F421741e481");
        pub const QUOTER: Address = address!("3d4e44Eb1374240CE5F1B871ab261CD16335B76a");
        pub const TICK_LEN: Address = address!("0CdeE061c75D43c82520eD998C23ac2991c9ac6d");
    }
}
