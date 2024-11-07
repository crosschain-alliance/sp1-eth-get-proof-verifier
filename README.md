# sp1-eth-get-proof-verifier

This repository contains an SP1 program that verifies account and storage proofs retrieved using the eth_getProof RPC call. It also enables the verification of a chain of blocks, allowing you to verify a storage proof against an ancestor block when you have a more recent block.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
  - [Building the Program](#building-the-program)
  - [Running the Script](#running-the-script)
- [Example](#example)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgements](#acknowledgements)

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust Programming Language**: Install Rust by following the instructions [here](https://www.rust-lang.org/tools/install).
- **SP1**: Install SP1 by following the [official installation guide](https://docs.succinct.xyz/getting-started/install.html).
- **Ethereum Node RPC URL**: Obtain an RPC URL from providers like [Infura](https://infura.io/) or [Alchemy](https://www.alchemy.com/).

## Installation

Clone the repository and navigate into it:

```bash
git clone https://github.com/crosschain-alliance/sp1-eth-get-proof-verifier.git
cd sp1-eth-get-proof-verifier
```

## Usage

### Building the Program

Navigate to the `program` directory and build the SP1 proof:

```bash
cd program
cargo prove build
```

### Running the Script

Navigate to the `script` directory and execute the script:

```bash
cd ../script
RUST_LOG=info cargo run --release -- --execute \
    --rpc-url <rpc_url> \
    --block-number <block_number_hex_encoded> \
    --proof-block-number <ancestral_block_number_hex_encoded> \
    --account <account> \
    --storage-key <storage_key>
```

If you want to generate the proof, replace `execute` with `prove`

**Parameters:**

- `<rpc_url>`: The RPC URL of your node.
- `<reference_block_number>`(optional):  The block number in hexadecimal format (e.g., `0xAABBCC`).
- `<proof_block_number>`: The block number for the proof, specified in hexadecimal format (e.g., 0xAABBCC). If `<proof_block_number>` is set to a value less than `<reference_block_number>`, the program will validate the entire chain of blocks from `<reference_block_number>` down to `<proof_block_number>`.
- `<account>`: The account address to verify (e.g., `0x1234567890abcdef1234567890abcdef12345678`).
- `<storage_key>`: The storage key you wish to verify (e.g., `0x000000000000000000000000000000000000000000000000000000000000000b`).

## Example

Here's an example command:

```bash
RUST_LOG=info cargo run --release -- --execute \
    --rpc-url https://mainnet.infura.io/v3/YOUR-PROJECT-ID \
    --reference-block-number 0x13D9A60 \
    --proof-block-number 0x13D9A5F \
    --account 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 \
    --storage-key 0x000000000000000000000000000000000000000000000000000000000000000b
```

## Contributing

Contributions are welcome! If you have suggestions or find issues, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgements

- **[Succinct Labs](https://succinct.xyz)** for developing SP1.
- **[Alloy](https://github.com/alloy-rs)** for providing valuable tools and resources that assisted in this project's development.



