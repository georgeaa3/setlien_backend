#![no_std]

use crate::storage_types::DataKey;
use soroban_sdk::{Env};

pub fn read_count(e: &Env) -> u128 {
    let key = DataKey::Count;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_count(e: &Env, count: &u128) {
    let key = DataKey::Count;
    e.storage().set(&key, count);
}