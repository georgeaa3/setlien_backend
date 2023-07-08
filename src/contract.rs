use crate::admin::{
    has_administrator, pause_rent, read_administrator, read_payment_token, resume_rent,
    write_administrator, write_payment_token,
};
use crate::event;
use crate::lease::{load_lease, write_lease};
use crate::storage_types::{DataKey, LeaseState, Leasing, LeasingRenting, Renting};
use crate::token_utils::{
    balance, make_admin, set_authorized, set_unauthorized, transfer, transfer_from,
};
use crate::utils::read_count;

use soroban_sdk::{contractimpl, log, Address, BytesN, Env};
pub struct SetLien;

#[contractimpl]
impl SetLien {
    pub fn initialize(env: Env, _admin: Address, _payment_token: Address) {
        if has_administrator(&env) {
            panic!("already initialized")
        }
        write_administrator(&env, &_admin);
        write_payment_token(&env, &_payment_token);
    }

    pub fn pause(env: Env) {
        let admin = read_administrator(&env);
        admin.require_auth();

        pause_rent(&env);
        event::paused(&env, admin);
    }

    pub fn resume(env: Env) {
        let admin = read_administrator(&env);
        admin.require_auth();

        resume_rent(&env);
        event::resumed(&env, admin);
    }

    pub fn lease(env: Env, leaser: Address, token: Address, _price: u128, _duration: u128) {
        leaser.require_auth();

        let current = &env.current_contract_address();
        // make contract admin of the nft
        make_admin(&env, &token, current);
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

        // Emit event
    }

    pub fn rent(env: Env, renter: Address, token: Address, _duration: u128) {
        // Transfer token to renter
        // Set authorized to false so that user cannot transfer token
        // Transfer payment to leaser
        // Set all fields

        renter.require_auth();

        let current = &env.current_contract_address();

        // Load lease
        let mut leaser_renter = load_lease(&env, &token);
        let leaser = &leaser_renter.leasing.leaser;
        let price = leaser_renter.leasing.price;
        let payment_token = read_payment_token(&env);

        log!(
            &env,
            "Balance: {}, PaymentToken: {}",
            balance(&env, &payment_token, &renter),
            payment_token
        );

        // Transfer payment token to the leaser
        transfer_from(
            &env,
            &payment_token,
            &renter,
            leaser,
            price.try_into().unwrap(),
        );

        log!(
            &env,
            "Balance: {}, PaymentToken: {}",
            balance(&env, &payment_token, &renter),
            payment_token
        );

        // Authorize leaser to transfer nft to renter
        set_authorized(&env, &token, &leaser);

        // Transfer nft to the renter
        transfer_from(&env, &token, leaser, &renter, 1);
        // Set authorized to false so that user cannot transfer token unless delisted
        set_unauthorized(&env, &token, &renter);
        // Set all fields
        let renting: Renting = Renting {
            renter: renter.clone(),
            rent_duration: _duration,
            rented_at: 0,
        };

        leaser_renter.renting = renting;
        leaser_renter.state = LeaseState::Rented;

        write_lease(&env, &token, &leaser_renter);
    }

    pub fn endLease(env: Env, leaser: Address, token: Address) {
        // Check lease status
        // Set authorized to true
        // Change admin back to leaser

        leaser.require_auth();

        let current = &env.current_contract_address();

        // Load lease
        let mut leaser_renter = load_lease(&env, &token);

        if leaser_renter.state != LeaseState::Listed {
            panic!("cannot end lease for a non-listed token");
        }

        // Set authorized to true so that user can transfer token now
        // set_authorized(&env, &token, &leaser);
        // Set all fields
        let renting: Renting = Renting {
            renter: leaser.clone(),
            rent_duration: 0,
            rented_at: 0,
        };

        let lease = Leasing {
            leaser: leaser.clone(),
            max_duration: 0,
            price: 0,
        };

        leaser_renter.renting = renting;
        leaser_renter.leasing = lease;
        leaser_renter.state = LeaseState::Available;

        write_lease(&env, &token, &leaser_renter);
    }

    pub fn endRent(env: Env, renter: Address, token: Address) {
        // Check lease status
        // Transfer token from renter to leaser
        // Set authorized to true for both

        renter.require_auth();

        let current = &env.current_contract_address();

        // Load lease
        let mut leaser_renter = load_lease(&env, &token);

        if leaser_renter.state != LeaseState::Rented {
            panic!("cannot end rent for a non-rented token");
        }

        // Authorize renter to transfer nft to leaser
        set_authorized(&env, &token, &leaser_renter.renting.renter);

        // Transfer nft to the renter
        transfer_from(
            &env,
            &token,
            &leaser_renter.renting.renter,
            &leaser_renter.leasing.leaser,
            1,
        );
        // Set authorized to false so that user cannot transfer token unless delisted
        set_unauthorized(&env, &token, &leaser_renter.leasing.leaser);

        leaser_renter.renting.rent_duration = 0;
        leaser_renter.state = LeaseState::Listed;

        write_lease(&env, &token, &leaser_renter);
    }

    pub fn default(env: Env, leaser: Address, token: Address, relist: bool) {
        leaser.require_auth();

        let current = &env.current_contract_address();

        // Load lease
        let mut leaser_renter = load_lease(&env, &token);

        if leaser_renter.state != LeaseState::Rented {
            panic!("cannot default for a non-rented token");
        }

        // Check if rent
        if leaser_renter.renting.rent_duration == 0 {}

        // Authorize renter to transfer nft to leaser
        set_authorized(&env, &token, &leaser_renter.renting.renter);

        // Transfer nft to the renter
        transfer_from(
            &env,
            &token,
            &leaser_renter.renting.renter,
            &leaser_renter.leasing.leaser,
            1,
        );

        if relist {
            // Set authorized to false so that user cannot transfer token unless delisted
            set_unauthorized(&env, &token, &leaser_renter.leasing.leaser);
            leaser_renter.state = LeaseState::Listed;
        } else {
            leaser_renter.state = LeaseState::Available;
        }

        leaser_renter.renting.rent_duration = 0;

        write_lease(&env, &token, &leaser_renter);
    }

    pub fn get_lease(env: Env, token: Address) -> LeasingRenting {
        load_lease(&env, &token)
    }

    pub fn get_admin(env: Env) -> Address {
        read_administrator(&env)
    }

    pub fn get_payment_token(env: Env) -> Address {
        read_payment_token(&env)
    }
}
