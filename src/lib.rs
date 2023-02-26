/**
* Copyright  : 2023 Aegir Finance
* License    : GPL 3.0
* Maintainer : Aegir <0xAegir@protonmail.com>
* Stability  : Experimental
*/
use ic_cdk::{
    api,
    export::{
        candid::{CandidType, Encode},
        Principal,
    },
    storage,
};
// use serde::{Deserialize, Serialize};
// use serde_cbor::Serializer;
use std::cell::RefCell;

mod auth;
mod signing;
mod transport;

thread_local! {
    // TODO: Use the canister controllers to authenticate messages
    static OWNER: RefCell<Principal> = RefCell::new(Principal::anonymous());
    static NEURON_ID: RefCell<u64> = RefCell::new(0);
}

#[ic_cdk_macros::init]
fn init(owner: Option<Principal>) {
    OWNER.with(|o| *o.borrow_mut() = owner.unwrap_or_else(|| api::caller()));
}

#[ic_cdk_macros::query]
fn owner() -> Principal {
    OWNER.with(|o| (*o.borrow()).clone())
}

#[derive(CandidType, Debug)]
struct Message {
    pub contents: String,
}

// Test candid-encoding inside a canister.
#[ic_cdk_macros::update]
async fn encode(to: String) -> Vec<u8> {
    Encode!(&Message {
        contents: format!("Hello {to}!"),
    })
    .unwrap()
}

#[ic_cdk_macros::pre_upgrade]
fn pre_upgrade() {
    let owner = OWNER.with(|x| x.borrow().clone());
    let neuron_id = NEURON_ID.with(|x| x.borrow().clone());
    storage::stable_save((owner, neuron_id)).unwrap();
}

#[ic_cdk_macros::post_upgrade]
fn post_upgrade() {
    let (owner, neuron_id): (Principal, u64) = storage::stable_restore().unwrap();
    OWNER.with(|o| *o.borrow_mut() = owner);
    NEURON_ID.with(|n| *n.borrow_mut() = neuron_id);
}
