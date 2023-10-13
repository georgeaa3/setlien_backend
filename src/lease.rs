
use soroban_sdk::{Address, Env, Vec, vec};
use crate::storage_types::{LeasingRenting, DataKey, LEASEE_LIFETIME_THRESHOLD, LEASE_BUMP_AMOUNT};

pub fn write_lease(env: &Env, token: &Address, lease: &LeasingRenting) {
    env.storage().instance().bump(LEASEE_LIFETIME_THRESHOLD, LEASE_BUMP_AMOUNT);
    env.storage().persistent().set(&DataKey::Lease(token.clone()), lease);
}

pub fn load_lease(env: &Env, token: &Address) -> LeasingRenting {
    env.storage().instance().bump(LEASEE_LIFETIME_THRESHOLD, LEASE_BUMP_AMOUNT);
    env.storage().persistent().get(&DataKey::Lease(token.clone())).unwrap()
}

pub fn has_lease(env: &Env, token: &Address) -> bool {
    env.storage().instance().bump(LEASEE_LIFETIME_THRESHOLD, LEASE_BUMP_AMOUNT);
    env.storage().persistent().has(&DataKey::Lease(token.clone()))
}

pub fn remove_lease(env: &Env, token: &Address) {
    env.storage().instance().bump(LEASEE_LIFETIME_THRESHOLD, LEASE_BUMP_AMOUNT);
    env.storage().persistent().remove(&DataKey::Lease(token.clone()));
}

pub fn add_all_listed(env: &Env, token: &Address) -> bool {
    let mut all_leases = get_all_listed(env);
    if all_leases.contains(token) {
        return false
    }
    
    all_leases.push_back(token.clone());
    env.storage().persistent().set(&DataKey::AllListed, &all_leases);
    return true
}

pub fn remove_all_listed(env: &Env, token: &Address) -> bool {
    let mut all_leases = get_all_listed(env);
    if !all_leases.contains(token) {
        return false
    }
    
    if let Some(o) = all_leases.first_index_of(token.clone()) {
        all_leases.remove_unchecked(o);
        env.storage().persistent().set(&DataKey::AllListed, &all_leases);
        return true
    }
    
    return false
}

pub fn get_all_listed(env: &Env) -> Vec<Address> {
    env.storage().instance().bump(LEASEE_LIFETIME_THRESHOLD, LEASE_BUMP_AMOUNT);
    env.storage().persistent().get(&DataKey::AllListed).unwrap_or(vec![env])
}

pub fn add_leased_by_user(env: &Env, leaser: &Address, token: &Address) -> bool {
    let mut leased_by_user = get_leased_by_user(env, leaser);
    if leased_by_user.contains(token) {
        return false
    }
    
    leased_by_user.push_back(token.clone());
    env.storage().persistent().set(&DataKey::LeasedByUser(leaser.clone()), &leased_by_user);
    return true
}

pub fn remove_leased_by_user(env: &Env, leaser: &Address, token: &Address) -> bool {
    let mut leased_by_user = get_leased_by_user(env, leaser);
    if !leased_by_user.contains(token) {
        return false
    }
    
    if let Some(o) = leased_by_user.first_index_of(token.clone()) {
        leased_by_user.remove_unchecked(o);
        env.storage().persistent().set(&DataKey::LeasedByUser(leaser.clone()), &leased_by_user);
        return true
    }
    
    return false
}

pub fn get_leased_by_user(env: &Env, leaser: &Address) -> Vec<Address> {
    env.storage().instance().bump(LEASEE_LIFETIME_THRESHOLD, LEASE_BUMP_AMOUNT);
    env.storage().persistent().get(&DataKey::LeasedByUser(leaser.clone())).unwrap_or(vec![env])
}

pub fn add_rented_by_user(env: &Env, renter: &Address, token: &Address) -> bool {
    let mut rented_by_user = get_rented_by_user(env, renter);
    if rented_by_user.contains(token) {
        return false
    }
    
    rented_by_user.push_back(token.clone());
    env.storage().persistent().set(&DataKey::RentedByUser(renter.clone()), &rented_by_user);
    return true
}

pub fn remove_rented_by_user(env: &Env, renter: &Address, token: &Address) -> bool {
    let mut rented_by_user = get_rented_by_user(env, renter);
    if !rented_by_user.contains(token) {
        return false
    }
    
    if let Some(o) = rented_by_user.first_index_of(token.clone()) {
        rented_by_user.remove_unchecked(o);
        env.storage().persistent().set(&DataKey::RentedByUser(renter.clone()), &rented_by_user);
        return true
    }
    
    return false
}

pub fn get_rented_by_user(env: &Env, renter: &Address) -> Vec<Address> {
    env.storage().instance().bump(LEASEE_LIFETIME_THRESHOLD, LEASE_BUMP_AMOUNT);
    env.storage().persistent().get(&DataKey::RentedByUser(renter.clone())).unwrap_or(vec![env])
}