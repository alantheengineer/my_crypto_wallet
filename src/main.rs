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
    let args: Vec<String> = env::args().collect();
    let wallet_file_path = "crypto_wallet.json";
    if args.len() < 0 {
        let (secret_key, pub_key) = eth_wallet::generate_keypair();

        println!("{}: {}","Secret key".red().bold(), &secret_key.to_string());
        println!("{}: {}", "Public key".green().bold(), &pub_key.to_string());

        let pub_address = eth_wallet::public_key_address(&pub_key);
        println!("{}: {:?}", "public address".bold(), pub_address);

        let crypto_wallet = eth_wallet::Wallet::new(&secret_key, &pub_key);
        println!("crypto_wallet: {:?}", &crypto_wallet);
        crypto_wallet.save_to_file(wallet_file_path)?;
    }

    let loaded_wallet = eth_wallet::Wallet::from_file(wallet_file_path)?;
    println!("loaded_wallet: {:?}", loaded_wallet);

    let endpoint = env::var("INFURA_GOERLI_WS")?;
    let web3_con = eth_wallet::establish_web3_connection(&endpoint).await?;

    let block_number = web3_con.eth().block_number().await?;
    println!("block number: {}", &block_number);

    let balance = loaded_wallet.get_balance_in_eth(&web3_con).await?;
    println!("wallet balance: {}", &balance);

    //let transaction = eth_wallet::create_eth_transaction(
    //    Address::from_str("0xc7B669bbafE6BE353f7bc11c0A223F8b693adA17")?,
    //    0.0001,
    //);
    //let transact_hash =
    //    eth_wallet::sign_and_send(&web3_con, transaction, &loaded_wallet.get_secret_key()?).await?;
    //println!("transaction hash: {:?}", transact_hash);

    Ok(())
}   
