// keep the file name distinct but give it reasonable path name
// rather use tokens::WETH vs token_addrs::WETH. Picky.. I know
pub use amm_addrs as amms;
pub use token_addrs as tokens;

// declare our modules
pub mod amm_addrs;
pub mod token_addrs;

