
use core::ops::Add;

use soroban_sdk::{Address, Env, log};

use crate::{token, storage_types::INSTANCE_BUMP_AMOUNT};

pub fn balance(e: &Env, token: &Address, id: &Address) -> i128 {
    token::Client::new(e, token).balance(id)
}

pub fn transfer_from(e: &Env, token: &Address, from: &Address, to: &Address, amount: i128) {
    let token_client = token::Client::new(e, token);
    let contract_address = e.current_contract_address();

    log!(e, "{}, {}", balance(e, token, from), amount);
    token_client.transfer_from(&contract_address, from, to, &amount);
}

pub fn make_admin(e: &Env, token: &Address, to: &Address) {
    soroban_sdk::token::StellarAssetClient::new(e, token).set_admin(&to);
}

pub fn set_authorized(e: &Env, token: &Address, to: &Address) {
    soroban_sdk::token::StellarAssetClient::new(e, token).set_authorized(to, &true);
}

pub fn set_unauthorized(e: &Env, token: &Address, to: &Address) {
    soroban_sdk::token::StellarAssetClient::new(e, token).set_authorized(to, &false);
}

pub fn is_authorized(e: &Env, token: &Address, to: &Address) -> bool {
    soroban_sdk::token::StellarAssetClient::new(e, token).authorized(to)
}

pub fn get_allowance(e: &Env, token: &Address, from: &Address, spender: &Address) -> i128 {
    soroban_sdk::token::TokenClient::new(e, token).allowance(from, spender)
}

pub fn clawback(e: &Env, token: &Address, from: &Address, amount: &i128)  {
    soroban_sdk::token::StellarAssetClient::new(e, token).clawback(from, amount)
}

pub fn mint(e: &Env, token: &Address, to: &Address, amount: &i128)  {
    soroban_sdk::token::StellarAssetClient::new(e, token).mint(to, amount)
}