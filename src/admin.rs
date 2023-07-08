use crate::storage_types::DataKey;
use soroban_sdk::{Address, Env};

pub fn write_payment_token(e: &Env, id: &Address) {
    let key = DataKey::PaymentToken;
    e.storage().set(&key, id);
}

pub fn read_payment_token(e: &Env) -> Address {
    let key = DataKey::PaymentToken;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().has(&key)
}

pub fn read_administrator(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().get_unchecked(&key).unwrap()
}

pub fn write_administrator(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().set(&key, id);
}

pub fn pause_rent(e: &Env) {
    let key = DataKey::Paused;
    e.storage().set(&key, &true);
}

pub fn resume_rent(e: &Env) {
    let key = DataKey::Paused;
    e.storage().set(&key, &false);
}