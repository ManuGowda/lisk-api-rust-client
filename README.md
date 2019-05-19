# Lisk Rust - API Client
> Rust API Client for accessing Lisk Blockchain

[![CircleCI](https://circleci.com/gh/ManuGowda/lisk-api-rust-client/tree/master.svg?style=svg&circle-token=2e6f0f19e6e3b0b3eda9dde759d4251eeb8961bd)](https://circleci.com/gh/ManuGowda/lisk-api-rust-client/tree/master)

## Prerequisites

Follow the official RUST docs and install [RUST](https://www.rust-lang.org/tools/install)

## Contribution Installation
```
git clone https://github.com/ManuGowda/lisk-api-rust-client.git
cd lisk-api-rust-client
cargo build
cargo test -- --nocapture
```

## Using the dependencies
Edit `Cargo.toml` file under `dependencies` section
```
lisk_api_rust_client = "0.1.0"
```

## Library usage
```
use lisk_api_rust_client::{Connection, Manager};

const HOST: &str = "https://node.lisk.io/api/";
let connection = Connection::new(HOST);
```
