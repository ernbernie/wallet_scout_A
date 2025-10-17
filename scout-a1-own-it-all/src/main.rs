use clap::Parser;
use scout_core::own_it_all::{fetch_wallet_data, WalletData, TokenAccount};

#[derive(Parser)]
#[command(name = "wallet-scout-a1")]
#[command(about = "WalletScout A1 - Own It All: Everything is owned, cloned, moved")]
struct Args {
    #[arg(long, help = "RPC endpoint URL")]
    rpc: String,
    
    #[arg(long, help = "Wallet address to inspect")]
    wallet: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("🔷 A1: OWN IT ALL - Memory Strategy Demo");
    println!("==========================================");
    println!("RPC: {}", args.rpc);
    println!("Wallet: {}", args.wallet);
    println!();
    
    // A1 Strategy: Own everything, clone liberally
    let wallet_data: WalletData = fetch_wallet_data(args.rpc.clone(), args.wallet.clone())?;
    
    // Clone the data for processing (A1 philosophy)
    let owned_data = wallet_data.clone();
    let owned_tokens = owned_data.token_accounts.clone();
    
    println!("💰 SOL Balance: {:.6} SOL", owned_data.sol_balance);
    println!("🪙 Token Accounts: {}", owned_tokens.len());
    println!();
    
    // Process each token account with owned data
    for (i, token) in owned_tokens.iter().enumerate() {
        let owned_token = token.clone(); // Clone for A1 strategy
        let owned_mint = owned_token.mint.clone();
        let owned_amount = owned_token.amount.clone();
        
        println!("Token {}: {}", i + 1, owned_mint);
        println!("  Amount: {} (decimals: {})", owned_amount, owned_token.decimals);
        
        // Demonstrate ownership by moving data around
        let processed_token = process_token_owned(owned_token);
        println!("  Processed: {}\n", processed_token);
    }
    
    println!("✅ A1 Strategy: All data owned, cloned, and moved safely!");
    
    Ok(())
}

// A1 Helper: Takes ownership, returns ownership
fn process_token_owned(token: TokenAccount) -> String {
    let mint = token.mint.clone(); // Clone to own
    let amount = token.amount.clone(); // Clone to own
    
    format!("Mint: {}, Amount: {}, Decimals: {}", mint, amount, token.decimals)
}
