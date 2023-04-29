use candid::Principal;
use ic_cdk::api::management_canister::main::{canister_status, CanisterIdRecord};

// Used in methods to ensure that the caller is a controller of this canister. Don't forget to
// .await this!
pub async fn require(user: &Principal) {
    let response = canister_status(CanisterIdRecord {
        canister_id: ic_cdk::api::id(),
    })
    .await
    .map_err(|e| format!("canister_status failed {}", e.1))
    .unwrap();

    let controllers = response.0.settings.controllers;
    assert!(
        controllers.contains(user),
        "Caller does not control this canister"
    );
}
