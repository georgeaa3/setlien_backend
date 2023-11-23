use crate::admin::{
    has_administrator, pause_rent, read_administrator, read_payment_token, resume_rent,
    write_administrator, write_payment_token,
};
use crate::event::{self};
use crate::lease::{has_lease, load_lease, remove_lease, write_lease, 
    get_all_listed, add_all_listed, remove_all_listed, 
    add_leased_by_user, remove_leased_by_user, get_leased_by_user, 
    add_rented_by_user, remove_rented_by_user, get_rented_by_user};
use crate::storage_types::{LeaseState, Leasing, LeasingRenting, Renting, INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT};
use crate::token_utils::{make_admin, set_authorized, set_unauthorized, transfer_from, increase_allowance};

use soroban_sdk::{contractimpl, contract, Address, Env, BytesN, log, IntoVal, Vec};

#[contract]
pub struct SetLien;

const NFT_BALANCE: i128 = 1;
const SECONDS_IN_DAYS: u128 = 86400;


pub trait LienTrait {
    fn initialize(env: Env, _admin: Address, _payment_token: Address);

    fn pause(env: Env);

    fn resume(env: Env);

    fn change_nft_admin(env: Env, token: Address, new_admin: Address);

    fn change_payment_token(env: Env, payment_token: Address);

    fn lease(env: Env, leaser: Address, token: Address, _price: u128, _duration: u128);

    fn rent(env: Env, renter: Address, token: Address, duration: u128);

    fn end_lease(env: Env, leaser: Address, token: Address);

    fn end_rent(env: Env, renter: Address, token: Address);

    fn claim_token(env: Env, leaser: Address, token: Address, relist: bool);

    fn has_lease(env: Env, token: Address) -> bool;

    fn get_lease(env: Env, token: Address) -> Option<LeasingRenting>;

    fn get_admin(env: Env) -> Address;

    fn get_payment_token(env: Env) -> Address;

    fn get_all_listed(env: Env) -> Vec<Address>;

    fn get_leased_by_user(env: Env, user: Address) -> Vec<Address>;

    fn get_rented_by_user(env: Env, user: Address) -> Vec<Address>;

    fn upgrade(env: Env, new_wasm_hash: BytesN<32>);

}

#[contractimpl]
impl LienTrait for SetLien {
    fn initialize(env: Env, _admin: Address, _payment_token: Address) {
        if has_administrator(&env) {
            panic!("already initialized")
        }
        write_administrator(&env, &_admin);
        write_payment_token(&env, &_payment_token);

        event::initialized(&env, &_admin, &_payment_token);
    }

    fn pause(env: Env) {
        let admin = read_administrator(&env);
        admin.require_auth();

        pause_rent(&env);
        event::paused(&env, admin);
    }

    fn resume(env: Env) {
        let admin = read_administrator(&env);
        admin.require_auth();

        resume_rent(&env);
        event::resumed(&env, admin);
    }

    fn change_nft_admin(env: Env, token: Address, new_admin: Address) {
        let admin = read_administrator(&env);
        admin.require_auth();

        make_admin(&env, &token, &new_admin);
    }

    fn change_payment_token(env: Env, payment_token: Address) {
        let admin = read_administrator(&env);
        admin.require_auth();

        write_payment_token(&env, &payment_token);
    }

    fn lease(env: Env, leaser: Address, token: Address, _price: u128, _duration: u128) {
        leaser.require_auth();
        
        env.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);

        let current = &env.current_contract_address();
        
        // Already has lease
        if has_lease(&env, &token) {
            panic!("token already has lease");
        }
        
        if !is_leaseable(&env, &leaser, &token, _price, _duration) {
            panic!("cannot lease token");
        }
        
        let expiration = env.ledger().sequence() + INSTANCE_BUMP_AMOUNT;
        // leaser.require_auth_for_args((current,NFT_BALANCE * 2,expiration).into_val(&env));
        // Set allowance to transfer
        // increase_allowance(&env, &token, &leaser, current, NFT_BALANCE * 2, expiration);
        // make contract admin of the nft
        // make_admin(&env, &token, current);
        // Set authorized to false so that user cannot transfer token unless delisted
        set_unauthorized(&env, &token, &leaser);
        // Set all fields
        let lease = Leasing {
            leaser: leaser.clone(),
            max_duration: _duration,
            price: _price,
        };
        let renting: Renting = Renting {
            renter: leaser.clone(),
            rent_duration: 0,
            rented_at: 0,
        };

        let leaserent = LeasingRenting {
            leasing: lease,
            renting,
            state: crate::storage_types::LeaseState::Listed,
        };

        // write lease
        write_lease(&env, &token, &leaserent);

        add_all_listed(&env, &token);

        add_leased_by_user(&env, &leaser, &token);

        event::leased(&env, &leaser, &token, _price, _duration);
        // Emit event
    }

    fn rent(env: Env, renter: Address, token: Address, duration: u128) {
        // Transfer token to renter
        // Set authorized to false so that user cannot transfer token
        // Transfer payment to leaser
        // Set all fields

        renter.require_auth();
        let current = &env.current_contract_address();

        env.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);

        if !has_lease(&env, &token) {
            panic!("token does not have lease");
        }

        // Load lease
        let mut leaser_renter = load_lease(&env, &token);
        let leaser = &leaser_renter.leasing.leaser;
        let price = calculate_total_price(duration, leaser_renter.leasing.price);
        let payment_token = read_payment_token(&env);

        if !is_rentable(
            &env,
            &renter,
            &leaser,
            duration,
            leaser_renter.leasing.max_duration,
        ) {
            panic!("cannot rent token");
        }

        // Set allowance to transfer payment token
        let expiration = env.ledger().sequence() + INSTANCE_BUMP_AMOUNT;
        // increase_allowance(&env, &payment_token, &renter, current, (price * 2).try_into().unwrap(), expiration);

        // Set allowance to transfer nft token
        // increase_allowance(&env, &token, &renter, current, NFT_BALANCE * 2, expiration);
        // Transfer payment token to the leaser
        transfer_from(
            &env,
            &payment_token,
            &renter,
            leaser,
            price.try_into().unwrap(),
        );

        // Authorize leaser to transfer nft to renter
        set_authorized(&env, &token, &leaser);

        // Transfer nft to the renter
        transfer_from(&env, &token, leaser, &renter, NFT_BALANCE);
        // Set authorized to false so that user cannot transfer token unless delisted
        set_unauthorized(&env, &token, &renter);
        // Set all fields
        let renting: Renting = Renting {
            renter: renter.clone(),
            rent_duration: duration,
            rented_at: env.ledger().timestamp() as u128,
        };

        leaser_renter.renting = renting;
        leaser_renter.state = LeaseState::Rented;

        write_lease(&env, &token, &leaser_renter);

        add_rented_by_user(&env, &renter, &token);

        event::rented(&env, &renter, &token,  duration);
    }

    fn end_lease(env: Env, leaser: Address, token: Address) {
        // Check lease status
        // Set authorized to true
        // Change admin back to leaser

        leaser.require_auth();

        env.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);

        if !has_lease(&env, &token) {
            panic!("token does not have lease");
        }
        // Load lease
        let leaser_renter = load_lease(&env, &token);

        if leaser_renter.state != LeaseState::Listed {
            panic!("cannot end lease for a non-listed token");
        }

        // make leaser admin of the nft token
        // make_admin(&env, &token, &leaser);
        // Set authorized to true
        set_authorized(&env, &token, &leaser);

        remove_lease(&env, &token);

        remove_all_listed(&env, &token);

        remove_leased_by_user(&env, &leaser, &token);

        event::end_lease(&env, &leaser, &token, 0);
    }

    fn end_rent(env: Env, renter: Address, token: Address) {
        // Check lease status
        // Transfer token from renter to leaser
        // Set authorized to true for both

        renter.require_auth();

        env.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);

        if !has_lease(&env, &token) {
            panic!("token does not have lease");
        }
        // Load lease
        let leaser_renter = load_lease(&env, &token);

        if leaser_renter.state != LeaseState::Rented {
            panic!("cannot end rent for a non-rented token");
        }

        // Authorize renter to transfer nft to leaser
        set_authorized(&env, &token, &renter);

        // Transfer nft to the renter
        transfer_from(
            &env,
            &token,
            &renter,
            &leaser_renter.leasing.leaser,
            NFT_BALANCE,
        );
        // Set authorized to false so that user cannot transfer token unless delisted
        set_authorized(&env, &token, &leaser_renter.leasing.leaser);
        // make leaser admin of the nft token
        // make_admin(&env, &token, &leaser_renter.leasing.leaser);
        
        remove_lease(&env, &token);

        remove_all_listed(&env, &token);

        remove_rented_by_user(&env, &renter, &token);

        remove_leased_by_user(&env, &leaser_renter.leasing.leaser, &token);

        event::returned(&env, &renter, &token, 0);
    }

    fn claim_token(env: Env, leaser: Address, token: Address, relist: bool) {
        leaser.require_auth();

        env.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);

        // Load lease
        let mut leaser_renter: LeasingRenting = load_lease(&env, &token);

        if leaser_renter.state != LeaseState::Rented {
            panic!("cannot default for a non-rented token");
        }

        let (duration, rented_at, max_duration) = (
            leaser_renter.leasing.max_duration,
            leaser_renter.renting.rented_at,
            leaser_renter.renting.rent_duration,
        );

        // Check if rent is overdue
        if !is_claimable(&env, rented_at, duration, max_duration) {
            panic!("cannot claim token");
        }

        // Authorize renter to transfer nft to leaser
        set_authorized(&env, &token, &leaser_renter.renting.renter);

        // TODO: Check if there is no approval from renter, then clawback and mint
        // Transfer nft to the renter
        transfer_from(
            &env,
            &token,
            &leaser_renter.renting.renter,
            &leaser_renter.leasing.leaser,
            NFT_BALANCE,
        );

        if relist {
            // Set authorized to false so that user cannot transfer token unless delisted
            set_unauthorized(&env, &token, &leaser_renter.leasing.leaser);
            leaser_renter.state = LeaseState::Listed;
            leaser_renter.renting.rent_duration = 0;
            write_lease(&env, &token, &leaser_renter);
        } else {
            set_authorized(&env, &token, &leaser_renter.leasing.leaser);
            // make leaser admin of the nft token
            // make_admin(&env, &token, &leaser_renter.leasing.leaser);
            
            remove_lease(&env, &token);

            remove_all_listed(&env, &token);

            remove_leased_by_user(&env, &leaser, &token);
        }

        remove_rented_by_user(&env, &leaser_renter.renting.renter, &token);

        event::claimed(&env, &leaser, &token, relist);
    }
    
    fn upgrade(env: Env, new_wasm_hash: BytesN<32>) {
        let admin = read_administrator(&env);
        admin.require_auth();

        env.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);

        env.deployer().update_current_contract_wasm(new_wasm_hash);
    }

    fn get_lease(env: Env, token: Address) -> Option<LeasingRenting> {
        env.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);
        if has_lease(&env, &token) {
            Some(load_lease(&env, &token))
        } else {
            None
        }
    }

    fn get_all_listed(env: Env) -> Vec<Address> {
        env.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);
        get_all_listed(&env)
    }

    fn get_leased_by_user(env: Env, user: Address) -> Vec<Address> {
        env.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);
        get_leased_by_user(&env, &user)
    }

    fn get_rented_by_user(env: Env, user: Address) -> Vec<Address> {
        env.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);
        get_rented_by_user(&env, &user)
    }

    fn has_lease(env: Env, token: Address) -> bool {
        env.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);
        has_lease(&env, &token)
    }

    fn get_admin(env: Env) -> Address {
        env.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);
        read_administrator(&env)
    }

    fn get_payment_token(env: Env) -> Address {
        env.storage().instance().bump(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT * 2);
        read_payment_token(&env)
    }
}

fn is_nft(env: &Env, leaser: &Address, token: &Address) -> bool {
    true
}

fn is_leaseable(
    env: &Env,
    leaser: &Address,
    token: &Address,
    _price: u128,
    _duration: u128,
) -> bool {
    if _price <= 0 || _duration <= 0 {
        return false;
    }

    if !is_nft(env, leaser, token) {
        return false;
    }

    true
}

fn is_rentable(
    env: &Env,
    renter: &Address,
    leaser: &Address,
    _duration: u128,
    max_duration: u128,
) -> bool {
    if renter.eq(leaser) {
        return false;
    }

    if _duration <= 0 || _duration % SECONDS_IN_DAYS != 0 {
        return false;
    }

    if _duration > max_duration {
        return false;
    }
    true
}

fn is_claimable(env: &Env, rented_at: u128, duration: u128, max_duration: u128) -> bool {
    // now: 100000, rented_at: 90000, duration: 1000, max_duration: 2000
    let now = env.ledger().timestamp() as u128; // 10000

    log!(env, "{}, {}, {}", now, rented_at, duration);
     // rent time has not started yet (should never happen)
    if rented_at > now {
        return false;
    }

    // 100000 - 90000 = 10000 <  1000 = false
    if (now - rented_at) < duration {
        return false;
    }
    true
}

fn calculate_total_price(_duration: u128, _price: u128) -> u128 {
    let num_days = _duration / (SECONDS_IN_DAYS);
    num_days * _price
}
