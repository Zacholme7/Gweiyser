use alloy_sol_types::sol;

// Abi Generation an ERC20 token
sol!(
    #[sol(rpc)]
    contract ERC20Token {
        function totalSupply() external view returns (uint256 totalSupply);
        function balanceOf(address account) external view returns (uint256 balance);
        function symbol() external view returns (string memory symbol);
        function approve(address spender, uint256 amount) external returns (bool success);
        function allowance(address owner, address spender) public view returns (uint256 allowance);
        function decimals() public view returns (uint8 decimals);
        function deposit() external payable;
    }
);
