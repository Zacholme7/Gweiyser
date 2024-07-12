use alloy::sol;

// Abi Generation an ERC20 token
sol!(
        #[sol(rpc)]
        contract Token {
                function totalSupply() external view returns (uint256);
                function balanceOf(address account) external view returns (uint256);
                function name() external view returns (string memory);
                function symbol() external view returns (string memory);
                function decimals() external view returns (uint8);
        }
);