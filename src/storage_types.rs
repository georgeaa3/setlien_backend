use soroban_sdk::{contracttype, Address};

pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 34560; // 2 days
pub(crate) const LEASE_BUMP_AMOUNT: u32 = 518400; // 30 days

#[derive(Clone, PartialEq, Debug)]
#[contracttype]
pub enum LeaseState {
    Available,
    Listed,
    Rented,
}

#[derive(Clone)]
#[contracttype]
pub struct Leasing {
    pub leaser: Address,
    pub max_duration: u128,
    pub price: u128,
}

#[derive(Clone)]
#[contracttype]
pub struct Renting {
    pub renter: Address,
    pub rent_duration: u128,
    pub rented_at: u128,
}

#[derive(Clone)]
#[contracttype]
pub struct LeasingRenting {
    pub leasing: Leasing,
    pub renting: Renting,
    pub state: LeaseState,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    PaymentToken,
    Admin,
    Paused,
    Count,
    Lease(Address)
}