#![no_main]

use alloy_primitives::keccak256;
use alloy_rlp::encode;
use alloy_rpc_types_eth::{Account, EIP1186AccountProofResponse, Header};
use alloy_trie::{proof::verify_proof, Nibbles};
use sp1_zkvm::io::{commit_slice, read};

sp1_zkvm::entrypoint!(main);

pub fn main() {
    // https://stackoverflow.com/questions/71648494/including-a-none-in-a-bincode-deserialization-will-throw-an-error-despite-being
    let block_headers_str = read::<String>();
    let proof_str = read::<String>();
    let block_headers: Vec<Header> = serde_json::from_str(&block_headers_str).unwrap();
    let proof: EIP1186AccountProofResponse = serde_json::from_str(&proof_str).unwrap();

    // commit reference_block_number
    commit_slice(&block_headers[0].number.to_be_bytes());

    for i in 0..block_headers.len() - 1 {
        let block_header = block_headers[i].clone();
        let previous_block_header = block_headers[i + 1].clone();
        if block_header.parent_hash != previous_block_header.hash {
            panic!("invalid chain of blocks");
        }
    }

    let proof_block_header = block_headers[block_headers.len() - 1].clone();
    commit_slice(&proof_block_header.number.to_be_bytes());

    let account = Account {
        nonce: proof.nonce,
        balance: proof.balance,
        storage_root: proof.storage_hash,
        code_hash: proof.code_hash,
    };

    verify_proof(
        proof_block_header.state_root,
        Nibbles::unpack(keccak256(proof.address)),
        Some(encode(&account)),
        &proof.account_proof,
    )
    .expect("failed to verify the account proof");

    for storage_proof in proof.storage_proof {
        verify_proof(
            proof.storage_hash,
            Nibbles::unpack(keccak256(storage_proof.key.0)),
            Some(encode(storage_proof.value).to_vec()),
            &storage_proof.proof,
        )
        .expect("failed to verify the storage proof");

        commit_slice(&storage_proof.value.to_be_bytes_vec().as_slice());
    }
}
