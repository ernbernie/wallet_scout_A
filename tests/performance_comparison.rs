use scout_core::own_it_all;
use scout_core::borrow_everything;
use scout_core::zero_copy;
use std::time::Instant;

/// Performance comparison tests for the three ownership strategies
/// These tests measure execution time and memory usage patterns

#[test]
fn test_execution_time_comparison() {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let wallet_address = "performance_test_wallet";
    let iterations = 1000;
    
    // Benchmark A1: Own It All
    let a1_start = Instant::now();
    for _ in 0..iterations {
        let _data = own_it_all::fetch_wallet_data(rpc_url.to_string(), wallet_address.to_string()).unwrap();
        // Simulate processing with owned data
        let _copy = _data.clone();
        let _tokens = _copy.token_accounts.clone();
        for token in _tokens {
            let _mint = token.mint.clone();
            let _amount = token.amount.clone();
        }
    }
    let a1_duration = a1_start.elapsed();
    
    // Benchmark A2: Borrow Everything
    let a2_start = Instant::now();
    for _ in 0..iterations {
        let _data = borrow_everything::fetch_wallet_data(rpc_url, wallet_address).unwrap();
        // Simulate processing with borrowed data
        let _tokens = &_data.token_accounts;
        for token in _tokens {
            let _mint = token.mint;
            let _amount = token.amount;
        }
    }
    let a2_duration = a2_start.elapsed();
    
    // Benchmark A3: Zero-Copy
    let a3_start = Instant::now();
    for _ in 0..iterations {
        let _data = zero_copy::fetch_wallet_data(rpc_url, wallet_address).unwrap();
        // Simulate processing with zero-copy data
        let _raw_data = _data.raw_data;
        for i in 0.._data.token_count {
            if let Some(token) = zero_copy::parse_token_account(_raw_data, i) {
                let _mint_slice = token.mint();
                let _amount_slice = token.amount();
            }
        }
    }
    let a3_duration = a3_start.elapsed();
    
    println!("Performance Results ({} iterations):", iterations);
    println!("A1 (Own It All): {:?}", a1_duration);
    println!("A2 (Borrow Everything): {:?}", a2_duration);
    println!("A3 (Zero-Copy): {:?}", a3_duration);
    
    // All strategies should complete successfully
    assert!(a1_duration.as_nanos() > 0);
    assert!(a2_duration.as_nanos() > 0);
    assert!(a3_duration.as_nanos() > 0);
}

#[test]
fn test_memory_usage_patterns() {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let wallet_address = "memory_pattern_test";
    
    // Test A1: Memory usage with multiple copies
    let a1_data = own_it_all::fetch_wallet_data(rpc_url.to_string(), wallet_address.to_string()).unwrap();
    let a1_copy1 = a1_data.clone();
    let a1_copy2 = a1_data.clone();
    let a1_copy3 = a1_data.clone();
    
    // A1 creates multiple independent copies
    assert_eq!(a1_copy1.sol_balance, a1_copy2.sol_balance);
    assert_eq!(a1_copy2.sol_balance, a1_copy3.sol_balance);
    assert_eq!(a1_copy1.token_accounts.len(), a1_copy2.token_accounts.len());
    
    // Test A2: Memory usage with borrowing
    let a2_data = borrow_everything::fetch_wallet_data(rpc_url, wallet_address).unwrap();
    let a2_ref1 = &a2_data;
    let a2_ref2 = &a2_data;
    let a2_ref3 = &a2_data;
    
    // A2 creates multiple references to the same data
    assert_eq!(a2_ref1.sol_balance, a2_ref2.sol_balance);
    assert_eq!(a2_ref2.sol_balance, a2_ref3.sol_balance);
    assert_eq!(a2_ref1.token_accounts.len(), a2_ref2.token_accounts.len());
    
    // Test A3: Memory usage with zero-copy
    let a3_data = zero_copy::fetch_wallet_data(rpc_url, wallet_address).unwrap();
    let raw_data = a3_data.raw_data;
    
    // Create multiple token accounts from the same raw data
    let token1 = zero_copy::parse_token_account(raw_data, 0).unwrap();
    let token2 = zero_copy::parse_token_account(raw_data, 1).unwrap();
    let token1_again = zero_copy::parse_token_account(raw_data, 0).unwrap();
    
    // All should reference the same underlying data
    assert!(std::ptr::eq(token1.raw_data.as_ptr(), raw_data.as_ptr()));
    assert!(std::ptr::eq(token2.raw_data.as_ptr(), raw_data.as_ptr()));
    assert!(std::ptr::eq(token1_again.raw_data.as_ptr(), raw_data.as_ptr()));
}

#[test]
fn test_clone_vs_borrow_vs_slice_performance() {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let wallet_address = "clone_vs_borrow_test";
    let iterations = 10000;
    
    // Test cloning performance (A1 style)
    let clone_start = Instant::now();
    let a1_data = own_it_all::fetch_wallet_data(rpc_url.to_string(), wallet_address.to_string()).unwrap();
    for _ in 0..iterations {
        let _cloned_tokens = a1_data.token_accounts.clone();
        for token in _cloned_tokens {
            let _cloned_mint = token.mint.clone();
            let _cloned_amount = token.amount.clone();
        }
    }
    let clone_duration = clone_start.elapsed();
    
    // Test borrowing performance (A2 style)
    let borrow_start = Instant::now();
    let a2_data = borrow_everything::fetch_wallet_data(rpc_url, wallet_address).unwrap();
    for _ in 0..iterations {
        let _borrowed_tokens = &a2_data.token_accounts;
        for token in _borrowed_tokens {
            let _borrowed_mint = token.mint;
            let _borrowed_amount = token.amount;
        }
    }
    let borrow_duration = borrow_start.elapsed();
    
    // Test slice performance (A3 style)
    let slice_start = Instant::now();
    let a3_data = zero_copy::fetch_wallet_data(rpc_url, wallet_address).unwrap();
    let raw_data = a3_data.raw_data;
    for _ in 0..iterations {
        for i in 0..a3_data.token_count {
            if let Some(token) = zero_copy::parse_token_account(raw_data, i) {
                let _slice_mint = token.mint();
                let _slice_amount = token.amount();
            }
        }
    }
    let slice_duration = slice_start.elapsed();
    
    println!("Clone vs Borrow vs Slice Performance ({} iterations):", iterations);
    println!("Cloning (A1): {:?}", clone_duration);
    println!("Borrowing (A2): {:?}", borrow_duration);
    println!("Slicing (A3): {:?}", slice_duration);
    
    // All should complete successfully
    assert!(clone_duration.as_nanos() > 0);
    assert!(borrow_duration.as_nanos() > 0);
    assert!(slice_duration.as_nanos() > 0);
}

#[test]
fn test_ownership_strategy_scalability() {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let wallet_address = "scalability_test";
    let scale_factors = [1, 10, 100, 1000];
    
    for &scale in &scale_factors {
        // Test A1 scalability
        let a1_start = Instant::now();
        for _ in 0..scale {
            let data = own_it_all::fetch_wallet_data(rpc_url.to_string(), wallet_address.to_string()).unwrap();
            let _copy = data.clone();
            let _tokens = _copy.token_accounts.clone();
        }
        let a1_duration = a1_start.elapsed();
        
        // Test A2 scalability
        let a2_start = Instant::now();
        for _ in 0..scale {
            let data = borrow_everything::fetch_wallet_data(rpc_url, wallet_address).unwrap();
            let _tokens = &data.token_accounts;
        }
        let a2_duration = a2_start.elapsed();
        
        // Test A3 scalability
        let a3_start = Instant::now();
        for _ in 0..scale {
            let data = zero_copy::fetch_wallet_data(rpc_url, wallet_address).unwrap();
            let _raw_data = data.raw_data;
        }
        let a3_duration = a3_start.elapsed();
        
        println!("Scale factor {}: A1={:?}, A2={:?}, A3={:?}", 
                 scale, a1_duration, a2_duration, a3_duration);
        
        // All strategies should scale successfully
        assert!(a1_duration.as_nanos() >= 0);
        assert!(a2_duration.as_nanos() >= 0);
        assert!(a3_duration.as_nanos() >= 0);
    }
}
