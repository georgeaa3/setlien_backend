use soroban_sdk::{Address, Env, Symbol};

pub(crate) fn paused(e: &Env, admin: Address) {
    let topics = (Symbol::new(e, "paused"), admin);
    e.events().publish(topics, true);
}

pub(crate) fn resumed(e: &Env, admin: Address) {
    let topics = (Symbol::new(e, "resumed"), admin);
    e.events().publish(topics, false);
}

pub(crate) fn initialized(e: &Env, admin: Address, paused: bool) {
    let topics = (Symbol::new(e, "paused"), admin);
    e.events().publish(topics, paused);
}

pub(crate) fn leased(e: &Env, from: Address, to: Address, amount: i128) {
    let topics = (Symbol::new(e, "increase_allowance"), from, to);
    e.events().publish(topics, amount);
}

pub(crate) fn rented(e: &Env, from: Address, to: Address, amount: i128) {
    let topics = (Symbol::new(e, "increase_allowance"), from, to);
    e.events().publish(topics, amount);
}

pub(crate) fn returned(e: &Env, from: Address, to: Address, amount: i128) {
    let topics = (Symbol::new(e, "increase_allowance"), from, to);
    e.events().publish(topics, amount);
}

pub(crate) fn endLease(e: &Env, from: Address, to: Address, amount: i128) {
    let topics = (Symbol::new(e, "increase_allowance"), from, to);
    e.events().publish(topics, amount);
}