
use soroban_sdk::{Address, Env};
use crate::storage_types::{LeasingRenting, DataKey};

pub fn write_lease(env: &Env, token: &Address, lease: &LeasingRenting) {
    env.storage().set(&DataKey::Lease(token.clone()), lease);
}

pub fn load_lease(env: &Env, token: &Address) -> LeasingRenting {
    env.storage().get_unchecked(&DataKey::Lease(token.clone())).unwrap()
}