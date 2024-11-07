use alloy_primitives::{Address, B256};
use alloy_rpc_client::ClientBuilder;
use alloy_rpc_types_eth::{Block, EIP1186AccountProofResponse, Header};
use alloy_transport_http::reqwest::Url;
use clap::Parser;
use core::panic;
use sp1_sdk::{ProverClient, SP1Stdin};
use std::str::FromStr;
use tokio;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The Ethereum account address to verify (e.g., `0x1234567890abcdef1234567890abcdef12345678`)
    #[arg(short, long)]
    account: String,

    /// The target block number to start verification from, in hexadecimal format (e.g., `0xAABBCC`). If left empty, it defaults to the latest block.
    #[arg(short, long, default_value = "")]
    reference_block_number: String,

    /// The block number for proof verification, specified in hexadecimal format (e.g., `0xAABBCC`).
    /// If `proof_block_number` is less than `reference_block_number`, the program will validate
    /// the entire chain of blocks from `reference_block_number` down to `proof_block_number`.
    #[arg(short, long)]
    proof_block_number: String,

    /// The RPC URL of your Ethereum node (e.g., `https://mainnet.infura.io/v3/YOUR-PROJECT-ID`)
    #[arg(short, long)]
    rpc_url: String,

    /// The specific storage key to verify within the account's storage, in hexadecimal format
    /// (e.g., `0x000000000000000000000000000000000000000000000000000000000000000b`)
    #[arg(short, long)]
    storage_key: String,

    /// Flag to execute the verification. Needed by cargo run --release
    #[arg(short, long)]
    execute: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = ClientBuilder::default().http(Url::from_str(&args.rpc_url).unwrap());
    let reference_block_number = if args.reference_block_number.is_empty() {
        args.proof_block_number.clone()
    } else {
        args.reference_block_number
    };
    let storage_key = args.storage_key;
    let proof_block_number = args.proof_block_number;

    if proof_block_number > reference_block_number {
        panic!("Invalid ancestral block number. It must be less than block number")
    }

    let mut blocks: Vec<Block> = Vec::new();
    for block_number in (u64::from_str_radix(&proof_block_number[2..], 16).unwrap()
        ..(u64::from_str_radix(&reference_block_number[2..], 16).unwrap() + 1))
        .rev()
    {
        println!("block_number {:?}", block_number);
        blocks.push(
            client
                .request(
                    "eth_getBlockByNumber",
                    vec![
                        serde_json::json!(format!("0x{:x}", block_number)),
                        serde_json::json!(false),
                    ],
                )
                .await
                .unwrap(),
        );
    }

    let account: Address = args.account.clone().parse().unwrap();
    let storage_key = B256::from_slice(&hex::decode(&storage_key.clone()[2..]).unwrap().as_slice());
    let proof: EIP1186AccountProofResponse = client
        .request(
            "eth_getProof",
            vec![
                serde_json::json!(account),
                serde_json::json!(vec![storage_key]),
                serde_json::json!(proof_block_number),
            ],
        )
        .await
        .unwrap();

    // https://stackoverflow.com/questions/71648494/including-a-none-in-a-bincode-deserialization-will-throw-an-error-despite-being
    let mut stdin = SP1Stdin::new();
    stdin.write::<String>(
        &serde_json::to_string(
            &blocks
                .iter()
                .map(|block: &Block| block.header.clone())
                .collect::<Vec<Header>>(),
        )
        .unwrap(),
    );
    stdin.write::<String>(&serde_json::to_string(&proof).unwrap());

    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    let proof = client.prove(&pk, stdin).run().unwrap();
    client.verify(&proof, &vk).expect("verification failed");

    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("succesfully generated and verified proof for the program!");

    Ok(())
}
