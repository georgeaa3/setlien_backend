
use soroban_sdk::{Address, Env, log};

use crate::{token, storage_types::INSTANCE_BUMP_AMOUNT};

pub fn balance(e: &Env, token: &Address, id: &Address) -> i128 {
    token::Client::new(e, token).balance(id)
}

pub fn transfer(e: &Env, token: &Address, from: &Address, to: &Address, amount: i128) {
    token::Client::new(e, token).transfer(from, to, &amount);
}

pub fn transfer_from(e: &Env, token: &Address, from: &Address, to: &Address, amount: i128) {
    let token_client = token::Client::new(e, token);
    let contract_address = e.current_contract_address();

    log!(e, "{}, {}", balance(e, token, from), amount);
    token_client.transfer_from(&contract_address, from, to, &amount);
}

pub fn increase_allowance(e: &Env, token: &Address, from: &Address, to: &Address, amount: i128, expiration: u32) {
    let token_client = token::Client::new(e, token);
    token_client.approve(from, &to, &amount, &expiration);
    // log!(e, "Allowance: {}, {}", token_client.allowance(from, to), amount);
}

pub fn make_admin(e: &Env, token: &Address, to: &Address) {
    token::Client::new(e, token).set_admin(&to);
}

pub fn set_authorized(e: &Env, token: &Address, to: &Address) {
    token::Client::new(e, token).set_authorized(to, &true);
}

pub fn set_unauthorized(e: &Env, token: &Address, to: &Address) {
    token::Client::new(e, token).set_authorized(to, &false);
}

pub fn is_authorized(e: &Env, token: &Address, to: &Address) -> bool {
    token::Client::new(e, token).authorized(to)
}