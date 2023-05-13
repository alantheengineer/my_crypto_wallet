use anyhow::Result;
use text_colorizer::*;
mod eth_wallet;
mod utils;
use std::env;
use std::str::FromStr;
use web3::types::Address;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let wallet_file_path = "crypto_wallet.json";
    let loaded_wallet: Wallet = match eth_wallet::Wallet::from_file(wallet_file_path) {
        Ok(v: Wallet) => v,
        Err(_) => {
            println!("{}: failed to find wallet JSON file, generating a new one...",
                "Warning".red().bold());
            let (secret_key, pub_key) = eth_wallet::generate_keypair();
            println!("{}: {}","Secret key".red().bold(), &secret_key.to_string());
            println!("{}: {}", "Public key".green().bold(), &pub_key.to_string());
            let pub_address = eth_wallet::public_key_address(&pub_key);
            println!("{}: {:?}", "public address".bold(), pub_address);

            let _wallet = eth_wallet::Wallet::new(&secret_key, &pub_key);
            _wallet.save_to_file(wallet_file_path)?;
            _wallet
        }
    };

    
    println!("loaded_wallet: {:?}", loaded_wallet);

    let endpoint = env::var("INFURA_SEPOLIA_WS")?;
    let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;

    let block_number = web3_con.eth().block_number().await?;
    println!("block number: {}", &block_number);

    let balance = loaded_wallet.get_balance_in_eth(&web3_con).await?;
    println!("wallet balance: {}", &balance);

    //let transaction = eth_wallet::create_eth_transaction(
    //    Address::from_str("0x.....")?,
    //    0.0001,
    //);
    //let transact_hash =
    //    eth_wallet::sign_and_send(&web3_con, transaction, &loaded_wallet.get_secret_key()?).await?;
    //println!("transaction hash: {:?}", transact_hash);

    Ok(())
}   
