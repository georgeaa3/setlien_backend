use crate::storage_types::{DataKey, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{Address, Env};

pub fn write_payment_token(e: &Env, id: &Address) {
    let key = DataKey::PaymentToken;
    e.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);
    e.storage().instance().set(&key, id);
}

pub fn read_payment_token(e: &Env) -> Address {
    let key = DataKey::PaymentToken;
    e.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);
    e.storage().instance().get(&key).unwrap()
}

pub fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);
    e.storage().instance().has(&key)
}

pub fn read_administrator(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);
    e.storage().instance().get(&key).unwrap()
}

pub fn write_administrator(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);
    e.storage().instance().set(&key, id);
}

pub fn pause_rent(e: &Env) {
    let key = DataKey::Paused;
    e.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);
    e.storage().instance().set(&key, &true);
}

pub fn resume_rent(e: &Env) {
    let key = DataKey::Paused;
    e.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);
    e.storage().instance().set(&key, &false);
}