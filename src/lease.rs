
use soroban_sdk::{Address, Env};
use crate::storage_types::{LeasingRenting, DataKey, LEASE_BUMP_AMOUNT};

pub fn write_lease(env: &Env, token: &Address, lease: &LeasingRenting) {
    env.storage().instance().bump(LEASE_BUMP_AMOUNT);
    env.storage().persistent().set(&DataKey::Lease(token.clone()), lease);
}

pub fn load_lease(env: &Env, token: &Address) -> LeasingRenting {
    env.storage().instance().bump(LEASE_BUMP_AMOUNT);
    env.storage().persistent().get(&DataKey::Lease(token.clone())).unwrap()
}

pub fn has_lease(env: &Env, token: &Address) -> bool {
    env.storage().instance().bump(LEASE_BUMP_AMOUNT);
    env.storage().persistent().has(&DataKey::Lease(token.clone()))
}

pub fn remove_lease(env: &Env, token: &Address) {
    env.storage().instance().bump(LEASE_BUMP_AMOUNT);
    env.storage().persistent().remove(&DataKey::Lease(token.clone()))
}