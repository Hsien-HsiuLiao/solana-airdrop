/*use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::signature::{Keypair, Signer};

fn main() {
    let wallet = Keypair::new();
    let pubkey = Signer::pubkey(&wallet);
    let rpc_url = String::from("https://api.devnet.solana.com");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    match client.request_airdrop(&pubkey, LAMPORTS_PER_SOL) {
        Ok(sig) => loop {
            if let Ok(confirmed) = client.confirm_transaction(&sig) {
                if confirmed {
                    println!("Transaction: {} Status: {}", sig, confirmed);
                    break;
                }
            }
        },
        Err(_) => println!("Error requesting airdrop"),
    };
}

//https://solanacookbook.com/references/local-development.html#subscribing-to-events
*/
use solana_client::rpc_client::RpcClient;
use rust_client::{check_balance, request_air_drop, transfer_funds, create_keypair};
use solana_sdk::signer::Signer;

const URL: &str = "https://api.devnet.solana.com";
//const URL: &str = "http://127.0.0.1:8899";

// Wrapper struct to allow Debug implementation

struct RpcClientWrapper {
    url: String,
    #[allow(dead_code)] // Suppress warning since we access it via as_ref()
    client: RpcClient, 
}

impl RpcClientWrapper {
    fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            client: RpcClient::new(url),
        }
    }
    
    fn as_ref(&self) -> &RpcClient {
        &self.client
    }
}

impl std::fmt::Debug for RpcClientWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "url: {}", self.url)
    }
}

fn main() {
    let rpc_client_wrapper = RpcClientWrapper::new(URL);
    let rpc_client = rpc_client_wrapper.as_ref();

    match rpc_client.get_version() {
        Ok(version) => {
            println!("RPC is available. Version: {:?}", version); //Version: 3.0.6
        },
        Err(e) => {
            println!("RPC is not available: {:?}", e);
            return;
        }
    }

    let sender = create_keypair();
    let receiver = create_keypair();

    println!("RPC Client: {:?}", rpc_client_wrapper);
    println!("Sender: {:?}", sender.pubkey());
    println!("Receiver: {:?}", receiver.pubkey());

    if let Ok(airdrop_signature) = request_air_drop(&rpc_client, &sender.pubkey(), 1.0) {
        println!("Airdrop finished! Signature: {:?}",  airdrop_signature);

        if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
            println!("Sender balance: {:?}", balance);
        }

        let transfer_amount = 0.5;

        match transfer_funds(&rpc_client, &sender, &receiver.pubkey(), transfer_amount) {
            Ok(sig) => { 
                println!("Transfer of {:?} finished. Signature: {:?}", transfer_amount, sig);
                if let Ok(balance) = check_balance(&rpc_client, &sender.pubkey()) {
                    println!("Sender balance after transfer: {:?}", balance);
                }
                if let Ok(balance) = check_balance(&rpc_client, &receiver.pubkey()) {
                    println!("Receiver balance after transfer: {:?}", balance);
                }
            },
            Err(err) => println!("Error: {:?}", err),
        }
    } else {
        println!("Airdrop failed");
    }
}