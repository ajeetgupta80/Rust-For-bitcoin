#![allow(unused)]
extern crate bitcoincore_rpc;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use serde::Deserialize;
use serde_json::json;

const RPC_URL: &str = "http://localhost:18443";
const RPC_USER: &str = "alice";
const RPC_PASS: &str = "password";
const WALLET: &str = "reg_wallet";

fn init_rpc_client() -> Client {
    Client::new(
        RPC_URL,
        Auth::UserPass(RPC_USER.to_owned(), RPC_PASS.to_owned()),
    )
    .expect("Failed to create RPC client")
}
fn list_wallet_dir(client: &Client) -> bitcoincore_rpc::Result<Vec<String>> {
    #[derive(Deserialize)]
    struct Name {
        name: String,
    }
    #[derive(Deserialize)]
    struct CallResult {
        wallets: Vec<Name>,
    }

    let result: CallResult = client.call("listwalletdir", &[])?;
    Ok(result.wallets.into_iter().map(|n| n.name).collect())
}

fn main() -> bitcoincore_rpc::Result<()> {
    let rpc_client = init_rpc_client();
    let wallet = list_wallet_dir(&rpc_client);
    rpc_client.load_wallet(WALLET);
    let info = rpc_client.get_blockchain_info().unwrap();
    let bal = rpc_client.get_balance(None, Some(false)).unwrap();
    let block_cnt = rpc_client.get_block_count().unwrap();

    println!("{:?}", info);
    println!();
    println!("balance: {}", bal);
    println!("block-count: {}", block_cnt);

    for w in wallet {
        print!("{:?}", w);
    }
    Ok(())
}
