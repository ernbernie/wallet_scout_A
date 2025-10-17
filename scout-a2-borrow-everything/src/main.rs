use clap::Parser;
use scout_core::borrow_everything::{fetch_wallet_data, WalletData, TokenAccount};

#[derive(Parser)]
#[command(name = "wallet-scout-a2")]
#[command(about = "WalletScout A2 - Borrow Everything: Everything is borrowed with lifetimes")]
struct Args {
    #[arg(long, help = "RPC endpoint URL")]
    rpc: String,
    
    #[arg(long, help = "Wallet address to inspect")]
    wallet: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("🔷 A2: BORROW EVERYTHING - Memory Strategy Demo");
    println!("===============================================");
    
    // A2 Strategy: Borrow everything, never clone
    analyze_wallet_borrowed(&args.rpc, &args.wallet)?;
    
    println!("✅ A2 Strategy: All data borrowed with proper lifetimes!");
    
    Ok(())
}

// A2 Strategy: Everything is borrowed, lifetimes everywhere
fn analyze_wallet_borrowed<'a>(
    rpc_url: &'a str, 
    wallet_address: &'a str
) -> Result<(), &'static str> {
    println!("RPC: {}", rpc_url);
    println!("Wallet: {}", wallet_address);
    println!();
    
    // Borrow the data, never own it
    let wallet_data: WalletData<'a> = fetch_wallet_data(rpc_url, wallet_address)?;
    
    // Borrow everything, no cloning
    let borrowed_tokens = &wallet_data.token_accounts;
    
    println!("💰 SOL Balance: {:.6} SOL", wallet_data.sol_balance);
    println!("🪙 Token Accounts: {}", borrowed_tokens.len());
    println!();
    
    // Process each token account with borrowed data
    for (i, token) in borrowed_tokens.iter().enumerate() {
        // Borrow the token data, never clone
        let borrowed_mint = &token.mint;
        let borrowed_amount = &token.amount;
        
        println!("Token {}: {}", i + 1, borrowed_mint);
        println!("  Amount: {} (decimals: {})", borrowed_amount, token.decimals);
        
        // Demonstrate borrowing by passing references
        let processed_info = process_token_borrowed(token);
        println!("  Processed: {}\n", processed_info);
    }
    
    // Demonstrate lifetime constraints
    demonstrate_lifetime_borrowing(&wallet_data);
    
    Ok(())
}

// A2 Helper: Takes references, returns borrowed data
fn process_token_borrowed<'a>(token: &'a TokenAccount<'a>) -> String {
    // Everything is borrowed, no cloning
    let mint = token.mint; // &str
    let amount = token.amount; // &str
    
    format!("Mint: {}, Amount: {}, Decimals: {}", mint, amount, token.decimals)
}

// A2 Advanced: Demonstrate lifetime relationships
fn demonstrate_lifetime_borrowing<'a>(wallet_data: &'a WalletData<'a>) {
    println!("🔗 Lifetime Demonstration:");
    
    // The lifetime 'a connects the wallet_data to its contents
    let tokens = &wallet_data.token_accounts;
    
    for token in tokens.iter() {
        // All these references live for the same lifetime 'a
        let mint_ref: &'a str = token.mint;
        let amount_ref: &'a str = token.amount;
        
        println!("  Borrowed mint: {}", mint_ref);
        println!("  Borrowed amount: {}", amount_ref);
        
        // This would fail if we tried to clone or own the data
        // let owned_mint = mint_ref.to_string(); // A2 avoids this!
    }
}
