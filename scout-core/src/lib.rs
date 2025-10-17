use serde::{Deserialize, Serialize};

/// A1: Own It All - Everything is owned, cloned, moved
pub mod own_it_all {
    use super::*;
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WalletData {
        pub sol_balance: f64,
        pub token_accounts: Vec<TokenAccount>,
    }
    
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TokenAccount {
        pub mint: String,
        pub amount: String,
        pub decimals: u8,
    }
    
    pub fn fetch_wallet_data(_rpc_url: String, _wallet_address: String) -> Result<WalletData, String> {
        // Simulate RPC call - in real implementation, use reqwest
        Ok(WalletData {
            sol_balance: 1.5,
            token_accounts: vec![
                TokenAccount {
                    mint: "So11111111111111111111111111111111111111112".to_string(),
                    amount: "1000000000".to_string(),
                    decimals: 9,
                },
                TokenAccount {
                    mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
                    amount: "250000000".to_string(),
                    decimals: 6,
                },
            ],
        })
    }
}

/// A2: Borrow Everything - Everything is borrowed with lifetimes
pub mod borrow_everything {
    use super::*;
    
    #[derive(Debug, Serialize)]
    #[serde(bound = "")]
    pub struct WalletData<'a> {
        pub sol_balance: f64,
        pub token_accounts: Vec<TokenAccount<'a>>,
    }
    
    #[derive(Debug, Serialize)]
    #[serde(bound = "")]
    pub struct TokenAccount<'a> {
        pub mint: &'a str,
        pub amount: &'a str,
        pub decimals: u8,
    }
    
    pub fn fetch_wallet_data<'a>(
        _rpc_url: &'a str, 
        _wallet_address: &'a str
    ) -> Result<WalletData<'a>, &'static str> {
        // Simulate RPC call with borrowed data
        Ok(WalletData {
            sol_balance: 1.5,
            token_accounts: vec![
                TokenAccount {
                    mint: "So11111111111111111111111111111111111111112",
                    amount: "1000000000",
                    decimals: 9,
                },
                TokenAccount {
                    mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
                    amount: "250000000",
                    decimals: 6,
                },
            ],
        })
    }
}

/// A3: Zero-Copy Decode - Parse from raw bytes without copying
pub mod zero_copy {
    
    pub struct WalletData<'a> {
        pub raw_data: &'a [u8],
        pub sol_balance: f64,
        pub token_count: usize,
    }
    
    pub struct TokenAccount<'a> {
        pub raw_data: &'a [u8],
        pub mint_offset: usize,
        pub amount_offset: usize,
        pub decimals_offset: usize,
    }
    
    impl<'a> TokenAccount<'a> {
        pub fn mint(&self) -> &[u8] {
            &self.raw_data[self.mint_offset..self.mint_offset + 32]
        }
        
        pub fn amount(&self) -> &[u8] {
            &self.raw_data[self.amount_offset..self.amount_offset + 8]
        }
        
        pub fn decimals(&self) -> u8 {
            self.raw_data[self.decimals_offset]
        }
    }
    
    pub fn fetch_wallet_data<'a>(_rpc_url: &str, _wallet_address: &str) -> Result<WalletData<'a>, &'static str> {
        // Simulate raw binary data from RPC
        let raw_data = b"SOL_BALANCE:1.5|TOKENS:2|MINT1:So11111111111111111111111111111111111111112|AMOUNT1:1000000000|DEC1:9|MINT2:EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v|AMOUNT2:250000000|DEC2:6";
        
        Ok(WalletData {
            raw_data,
            sol_balance: 1.5,
            token_count: 2,
        })
    }
    
    pub fn parse_token_account<'a>(data: &'a [u8], index: usize) -> Option<TokenAccount<'a>> {
        if index >= 2 { return None; } // Simulate 2 tokens
        
        Some(TokenAccount {
            raw_data: data,
            mint_offset: 20 + (index * 80), // Simulated offsets
            amount_offset: 52 + (index * 80),
            decimals_offset: 60 + (index * 80),
        })
    }
}

// Legacy function for backward compatibility
pub fn fetch_wallet_data(rpc_url: String, wallet_address: String) -> String {
    format!("RPC: {}, Wallet: {}", rpc_url, wallet_address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a1_own_it_all_basic_functionality() {
        let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
        let wallet_address = "test_wallet_123".to_string();
        
        let result = own_it_all::fetch_wallet_data(rpc_url.clone(), wallet_address.clone());
        assert!(result.is_ok());
        
        let wallet_data = result.unwrap();
        assert_eq!(wallet_data.sol_balance, 1.5);
        assert_eq!(wallet_data.token_accounts.len(), 2);
    }

    #[test]
    fn test_a1_ownership_transfer() {
        let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
        let wallet_address = "test_wallet_456".to_string();
        
        // Test that we can move data around (A1 philosophy)
        let wallet_data = own_it_all::fetch_wallet_data(rpc_url, wallet_address).unwrap();
        
        // Clone for processing (A1 strategy)
        let owned_tokens = wallet_data.token_accounts.clone();
        let first_token = owned_tokens[0].clone();
        
        // Move ownership around
        let processed_token = process_token_owned_test(first_token);
        assert!(processed_token.contains("So11111111111111111111111111111111111111112"));
    }

    #[test]
    fn test_a1_multiple_ownership() {
        let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
        let wallet_address = "test_wallet_789".to_string();
        
        let wallet_data = own_it_all::fetch_wallet_data(rpc_url, wallet_address).unwrap();
        
        // Create multiple owned copies (A1 allows this)
        let copy1 = wallet_data.clone();
        let copy2 = wallet_data.clone();
        
        assert_eq!(copy1.sol_balance, copy2.sol_balance);
        assert_eq!(copy1.token_accounts.len(), copy2.token_accounts.len());
        
        // Both copies are independent and owned
        let token1_copy1 = copy1.token_accounts[0].clone();
        let token1_copy2 = copy2.token_accounts[0].clone();
        
        assert_eq!(token1_copy1.mint, token1_copy2.mint);
    }

    // Helper function for A1 tests
    fn process_token_owned_test(token: own_it_all::TokenAccount) -> String {
        let mint = token.mint.clone(); // Clone to own
        let amount = token.amount.clone(); // Clone to own
        
        format!("Mint: {}, Amount: {}, Decimals: {}", mint, amount, token.decimals)
    }

    #[test]
    fn test_a2_borrow_everything_basic_functionality() {
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let wallet_address = "test_wallet_123";
        
        let result = borrow_everything::fetch_wallet_data(rpc_url, wallet_address);
        assert!(result.is_ok());
        
        let wallet_data = result.unwrap();
        assert_eq!(wallet_data.sol_balance, 1.5);
        assert_eq!(wallet_data.token_accounts.len(), 2);
    }

    #[test]
    fn test_a2_lifetime_relationships() {
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let wallet_address = "test_wallet_456";
        
        // Test that lifetimes work correctly
        let wallet_data = borrow_everything::fetch_wallet_data(rpc_url, wallet_address).unwrap();
        let tokens = &wallet_data.token_accounts;
        
        // All references should have the same lifetime
        for token in tokens {
            let mint_ref: &str = token.mint;
            let amount_ref: &str = token.amount;
            
            assert!(!mint_ref.is_empty());
            assert!(!amount_ref.is_empty());
        }
    }

    #[test]
    fn test_a2_no_cloning() {
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let wallet_address = "test_wallet_789";
        
        let wallet_data = borrow_everything::fetch_wallet_data(rpc_url, wallet_address).unwrap();
        let tokens = &wallet_data.token_accounts;
        
        // A2 strategy: borrow everything, never clone
        let borrowed_tokens = tokens; // Just borrowing
        let first_token = &borrowed_tokens[0];
        
        // We can access data without cloning
        let mint_borrowed = first_token.mint;
        let amount_borrowed = first_token.amount;
        
        assert_eq!(mint_borrowed, "So11111111111111111111111111111111111111112");
        assert_eq!(amount_borrowed, "1000000000");
    }

    #[test]
    fn test_a3_zero_copy_basic_functionality() {
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let wallet_address = "test_wallet_123";
        
        let result = zero_copy::fetch_wallet_data(rpc_url, wallet_address);
        assert!(result.is_ok());
        
        let wallet_data = result.unwrap();
        assert_eq!(wallet_data.sol_balance, 1.5);
        assert_eq!(wallet_data.token_count, 2);
        assert!(!wallet_data.raw_data.is_empty());
    }

    #[test]
    fn test_a3_slice_parsing() {
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let wallet_address = "test_wallet_456";
        
        let wallet_data = zero_copy::fetch_wallet_data(rpc_url, wallet_address).unwrap();
        let raw_data = wallet_data.raw_data;
        
        // Test zero-copy parsing
        for i in 0..wallet_data.token_count {
            if let Some(token_account) = zero_copy::parse_token_account(raw_data, i) {
                let mint_slice = token_account.mint();
                let amount_slice = token_account.amount();
                
                // Verify we can access slices without copying
                assert_eq!(mint_slice.len(), 32);
                assert_eq!(amount_slice.len(), 8);
                
                // The slices should point to the original data
                assert!(std::ptr::eq(mint_slice.as_ptr(), &raw_data[token_account.mint_offset]));
            }
        }
    }

    #[test]
    fn test_a3_no_data_copying() {
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let wallet_address = "test_wallet_789";
        
        let wallet_data = zero_copy::fetch_wallet_data(rpc_url, wallet_address).unwrap();
        let raw_data = wallet_data.raw_data;
        
        // Test that we can parse multiple tokens from the same raw data
        let token1 = zero_copy::parse_token_account(raw_data, 0).unwrap();
        let token2 = zero_copy::parse_token_account(raw_data, 1).unwrap();
        
        // Both tokens should reference the same underlying data
        assert!(std::ptr::eq(token1.raw_data.as_ptr(), raw_data.as_ptr()));
        assert!(std::ptr::eq(token2.raw_data.as_ptr(), raw_data.as_ptr()));
        
        // But have different offsets
        assert_ne!(token1.mint_offset, token2.mint_offset);
    }

    #[test]
    fn test_ownership_strategy_comparison() {
        let rpc_url = "https://api.mainnet-beta.solana.com";
        let wallet_address = "comparison_test";
        
        // Test all three strategies with the same data
        let a1_result = own_it_all::fetch_wallet_data(rpc_url.to_string(), wallet_address.to_string());
        let a2_result = borrow_everything::fetch_wallet_data(rpc_url, wallet_address);
        let a3_result = zero_copy::fetch_wallet_data(rpc_url, wallet_address);
        
        assert!(a1_result.is_ok());
        assert!(a2_result.is_ok());
        assert!(a3_result.is_ok());
        
        let a1_data = a1_result.unwrap();
        let a2_data = a2_result.unwrap();
        let a3_data = a3_result.unwrap();
        
        // All should have the same balance
        assert_eq!(a1_data.sol_balance, a2_data.sol_balance);
        assert_eq!(a1_data.sol_balance, a3_data.sol_balance);
        
        // All should have the same number of tokens
        assert_eq!(a1_data.token_accounts.len(), a2_data.token_accounts.len());
        assert_eq!(a1_data.token_accounts.len(), a3_data.token_count);
    }
}
