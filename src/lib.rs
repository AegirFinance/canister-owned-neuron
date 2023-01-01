/**
* Copyright  : 2023 Aegir Finance
* License    : GPL 3.0
* Maintainer : Aegir <0xAegir@protonmail.com>
* Stability  : Experimental
*/
use ic_cdk::export::{
    candid::CandidType,
    serde::{Deserialize, Serialize},
    Principal,
};
use serde::{Deserialize, Serialize};
use serde_cbor::Serializer;
use std::cell::RefCell;

thread_local! {
    // TODO: Use the canister controllers to authenticate messages
    static OWNER: RefCell<Principal> = RefCell::new(Principal::anonymous());
    static NEURON_ID: RefCell<u64> = RefCell::new(0);
}

#[init]
fn init(
    owner: Principal,
) {
    OWNER.with(|o| *o.borrow_mut() = owner);
}

#[ic_cdk_macros::query]
fn owner() -> Principal {
    OWNER.with(|o| (*o.borrow()).clone())
}

#[derive(CandidType, Serialize, Debug)]
struct Message {
    pub contents: String,
}

#[ic_cdk_macros::update]
async fn claim() -> Vec<u8> {
    // serialize a message
    let message = Message {
        contents: "Hello world!".to_string(),
    };

    let serialized_message = serialize(message);
    serialized_message

    // sign the message

    // return the signature

    // TODO: send the message via http outcalls
}

fn serialize(m: Message) -> Vec<u8> {
    let mut data = vec![];
    let mut serializer = Serializer::new(&mut data);
    serializer.self_describe().unwrap();
    m.serialize(&mut serializer).unwrap();
    data
}

#[pre_upgrade]
fn pre_upgrade() {
    let owner = OWNER.with(|x| x.borrow().clone());
    let neuron_id = NEURON_ID.with(|x| x.borrow().clone());
    ic::stable_store((owner, neuron_id)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (owner, neuron_id): (Principal, u64) = ic::stable_restore().unwrap();
    OWNER.with(|o| *o.borrow_mut() = owner);
    NEURON_ID.with(|n| *n.borrow_mut() = neuron_id);
}
