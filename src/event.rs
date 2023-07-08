use soroban_sdk::{Address, Env, Symbol};

pub(crate) fn paused(e: &Env, admin: Address) {
    let topics = (Symbol::new(e, "paused"), admin);
    e.events().publish(topics, true);
}

pub(crate) fn resumed(e: &Env, admin: Address) {
    let topics = (Symbol::new(e, "resumed"), admin);
    e.events().publish(topics, false);
}

pub(crate) fn initialized(e: &Env, admin: &Address, payment_token: &Address) {
    let topics = (Symbol::new(e, "initialized"), admin, payment_token);
    e.events().publish(topics, 0);
}

pub(crate) fn leased(e: &Env, leaser: &Address, token: &Address, price: u128, duration: u128) {
    let topics = (Symbol::new(e, "leased"), leaser, token);
    e.events().publish(topics, price);
}

pub(crate) fn rented(e: &Env, renter: &Address, token: &Address, duration: u128) {
    let topics = (Symbol::new(e, "rented"), renter, token);
    e.events().publish(topics, duration);
}

pub(crate) fn returned(e: &Env, renter: &Address, token: &Address, amount: i128) {
    let topics = (Symbol::new(e, "returned"), renter, token);
    e.events().publish(topics, amount);
}

pub(crate) fn end_lease(e: &Env, leaser: &Address, token: &Address, amount: i128) {
    let topics = (Symbol::new(e, "end_lease"), leaser, token);
    e.events().publish(topics, amount);
}

pub(crate) fn claimed(e: &Env, leaser: &Address, token: &Address, relist: bool) {
    let topics = (Symbol::new(e, "claimed"), leaser, token);
    e.events().publish(topics, relist);
}