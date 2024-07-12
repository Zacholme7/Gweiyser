use alloy_sol_types::{sol, SolCall};

// Abi Generation an ERC20 token
sol!(
        #[sol(rpc)]
        contract ERC20Token {
                function totalSupply() external view returns (uint256 totalSupply);
                function balanceOf(address account) external view returns (uint256 balance);
                function symbol() external view returns (string memory symbol);
                function decimals() external view returns (uint8 decimals);
        }
);
