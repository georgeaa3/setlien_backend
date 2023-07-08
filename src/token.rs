#![allow(unused)]
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env};

soroban_sdk::contractimport!(
    file = "/Users/jawadoc/Work/onchain/soroban-examples/token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
);