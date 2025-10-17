use scout_core::own_it_all;
use scout_core::borrow_everything;
use scout_core::zero_copy;

/// Integration tests that compare all three ownership strategies
/// These tests demonstrate the different memory management approaches
/// and verify they all produce equivalent results.

#[test]
fn test_all_strategies_produce_same_results() {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let wallet_address = "integration_test_wallet";
    
    // Test all three strategies
    let a1_result = own_it_all::fetch_wallet_data(rpc_url.to_string(), wallet_address.to_string());
    let a2_result = borrow_everything::fetch_wallet_data(rpc_url, wallet_address);
    let a3_result = zero_copy::fetch_wallet_data(rpc_url, wallet_address);
    
    assert!(a1_result.is_ok(), "A1 strategy should succeed");
    assert!(a2_result.is_ok(), "A2 strategy should succeed");
    assert!(a3_result.is_ok(), "A3 strategy should succeed");
    
    let a1_data = a1_result.unwrap();
    let a2_data = a2_result.unwrap();
    let a3_data = a3_result.unwrap();
    
    // All strategies should produce the same balance
    assert_eq!(a1_data.sol_balance, a2_data.sol_balance);
    assert_eq!(a1_data.sol_balance, a3_data.sol_balance);
    assert_eq!(a1_data.sol_balance, 1.5);
    
    // All strategies should have the same number of tokens
    assert_eq!(a1_data.token_accounts.len(), a2_data.token_accounts.len());
    assert_eq!(a1_data.token_accounts.len(), a3_data.token_count);
    assert_eq!(a1_data.token_accounts.len(), 2);
}

#[test]
fn test_ownership_strategy_memory_characteristics() {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let wallet_address = "memory_test_wallet";
    
    // Test A1: Own It All - should allow multiple independent copies
    let a1_data = own_it_all::fetch_wallet_data(rpc_url.to_string(), wallet_address.to_string()).unwrap();
    let a1_copy1 = a1_data.clone();
    let a1_copy2 = a1_data.clone();
    
    // A1 allows multiple owned copies
    assert_eq!(a1_copy1.sol_balance, a1_copy2.sol_balance);
    assert_eq!(a1_copy1.token_accounts[0].mint, a1_copy2.token_accounts[0].mint);
    
    // Test A2: Borrow Everything - should work with borrowed references
    let a2_data = borrow_everything::fetch_wallet_data(rpc_url, wallet_address).unwrap();
    let a2_tokens = &a2_data.token_accounts;
    let a2_first_token = &a2_tokens[0];
    
    // A2 works with borrowed references
    assert_eq!(a2_first_token.mint, "So11111111111111111111111111111111111111112");
    assert_eq!(a2_first_token.amount, "1000000000");
    
    // Test A3: Zero-Copy - should work with raw byte slices
    let a3_data = zero_copy::fetch_wallet_data(rpc_url, wallet_address).unwrap();
    let a3_raw_data = a3_data.raw_data;
    
    // A3 works with raw bytes
    assert!(!a3_raw_data.is_empty());
    assert_eq!(a3_data.token_count, 2);
    
    // Parse tokens from raw bytes
    for i in 0..a3_data.token_count {
        if let Some(token) = zero_copy::parse_token_account(a3_raw_data, i) {
            let mint_slice = token.mint();
            let amount_slice = token.amount();
            
            assert_eq!(mint_slice.len(), 32);
            assert_eq!(amount_slice.len(), 8);
        }
    }
}

#[test]
fn test_ownership_transfer_patterns() {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let wallet_address = "transfer_test_wallet";
    
    // Test A1: Ownership transfer with cloning
    let a1_data = own_it_all::fetch_wallet_data(rpc_url.to_string(), wallet_address.to_string()).unwrap();
    let a1_tokens = a1_data.token_accounts.clone(); // Clone to own
    let first_token = a1_tokens[0].clone(); // Clone to own
    
    // Move ownership around (A1 philosophy)
    let processed_token = process_a1_token(first_token);
    assert!(processed_token.contains("So11111111111111111111111111111111111111112"));
    
    // Test A2: Borrowing without cloning
    let a2_data = borrow_everything::fetch_wallet_data(rpc_url, wallet_address).unwrap();
    let a2_tokens = &a2_data.token_accounts; // Just borrowing
    let first_token_ref = &a2_tokens[0]; // Just borrowing
    
    // Process without cloning (A2 philosophy)
    let processed_info = process_a2_token(first_token_ref);
    assert!(processed_info.contains("So11111111111111111111111111111111111111112"));
    
    // Test A3: Zero-copy processing
    let a3_data = zero_copy::fetch_wallet_data(rpc_url, wallet_address).unwrap();
    let raw_data = a3_data.raw_data;
    
    if let Some(token_account) = zero_copy::parse_token_account(raw_data, 0) {
        let mint_slice = token_account.mint();
        let amount_slice = token_account.amount();
        
        // Process without copying (A3 philosophy)
        let processed_bytes = process_a3_token(&token_account);
        assert!(processed_bytes.contains("32"));
    }
}

#[test]
fn test_lifetime_constraints() {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let wallet_address = "lifetime_test_wallet";
    
    // Test A2: Lifetime relationships
    let a2_data = borrow_everything::fetch_wallet_data(rpc_url, wallet_address).unwrap();
    let tokens = &a2_data.token_accounts;
    
    // All references should have the same lifetime
    for token in tokens {
        let mint_ref: &str = token.mint;
        let amount_ref: &str = token.amount;
        
        // These references are valid as long as a2_data is alive
        assert!(!mint_ref.is_empty());
        assert!(!amount_ref.is_empty());
    }
    
    // Test A3: Lifetime with raw data
    let a3_data = zero_copy::fetch_wallet_data(rpc_url, wallet_address).unwrap();
    let raw_data = a3_data.raw_data;
    
    // Create multiple token accounts from the same raw data
    let token1 = zero_copy::parse_token_account(raw_data, 0).unwrap();
    let token2 = zero_copy::parse_token_account(raw_data, 1).unwrap();
    
    // Both should reference the same underlying data
    assert!(std::ptr::eq(token1.raw_data.as_ptr(), raw_data.as_ptr()));
    assert!(std::ptr::eq(token2.raw_data.as_ptr(), raw_data.as_ptr()));
}

#[test]
fn test_memory_efficiency_comparison() {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let wallet_address = "efficiency_test_wallet";
    
    // Test A1: Memory usage with owned data
    let a1_data = own_it_all::fetch_wallet_data(rpc_url.to_string(), wallet_address.to_string()).unwrap();
    let a1_copy = a1_data.clone(); // This creates a full copy
    
    // A1 uses more memory due to cloning
    assert_eq!(a1_data.sol_balance, a1_copy.sol_balance);
    assert_eq!(a1_data.token_accounts.len(), a1_copy.token_accounts.len());
    
    // Test A2: Memory usage with borrowed data
    let a2_data = borrow_everything::fetch_wallet_data(rpc_url, wallet_address).unwrap();
    let a2_tokens = &a2_data.token_accounts; // No copying, just borrowing
    
    // A2 uses less memory as it doesn't clone
    assert_eq!(a2_data.token_accounts.len(), a2_tokens.len());
    
    // Test A3: Memory usage with zero-copy
    let a3_data = zero_copy::fetch_wallet_data(rpc_url, wallet_address).unwrap();
    let raw_data = a3_data.raw_data;
    
    // A3 uses the least memory as it works directly with raw bytes
    assert!(!raw_data.is_empty());
    
    // Create multiple token accounts without copying data
    let token1 = zero_copy::parse_token_account(raw_data, 0).unwrap();
    let token2 = zero_copy::parse_token_account(raw_data, 1).unwrap();
    
    // Both reference the same raw data (no copying)
    assert!(std::ptr::eq(token1.raw_data.as_ptr(), token2.raw_data.as_ptr()));
}

// Helper functions to demonstrate different processing patterns

fn process_a1_token(token: own_it_all::TokenAccount) -> String {
    // A1: Clone to own, then process
    let mint = token.mint.clone();
    let amount = token.amount.clone();
    
    format!("A1 Processed: Mint={}, Amount={}, Decimals={}", mint, amount, token.decimals)
}

fn process_a2_token(token: &borrow_everything::TokenAccount) -> String {
    // A2: Borrow everything, never clone
    let mint = token.mint; // &str
    let amount = token.amount; // &str
    
    format!("A2 Processed: Mint={}, Amount={}, Decimals={}", mint, amount, token.decimals)
}

fn process_a3_token(token: &zero_copy::TokenAccount) -> String {
    // A3: Work directly with byte slices
    let mint_slice = token.mint();
    let amount_slice = token.amount();
    
    format!("A3 Processed: Mint slice len={}, Amount slice len={}, Decimals={}", 
            mint_slice.len(), amount_slice.len(), token.decimals())
}
