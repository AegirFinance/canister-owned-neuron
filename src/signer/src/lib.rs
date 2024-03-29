use ic_cdk::{
    api,
    export::{
        candid::CandidType,
        serde::{Deserialize, Serialize},
        Principal,
    },
    init, post_upgrade, pre_upgrade, storage, update,
};
use ic_ledger_types::{AccountIdentifier, Subaccount, DEFAULT_SUBACCOUNT};
use std::{cell::RefCell, str::FromStr};

mod controller;

thread_local! {
    static KEY_ID: RefCell<EcdsaKeyId> = RefCell::new(EcdsaKeyIds::TestKeyLocalDevelopment.to_key_id());
}

#[derive(CandidType, Deserialize)]
struct StableState {
    key_id: EcdsaKeyId,
}

#[pre_upgrade]
fn pre_upgrade() {
    let state = StableState {
        key_id: KEY_ID.with(|k| k.borrow().clone()),
    };
    storage::stable_save((state,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let (s,): (StableState,) = storage::stable_restore().unwrap();
    KEY_ID.with(|k| {
        *k.borrow_mut() = s.key_id;
    });
}

#[derive(CandidType, Serialize, Debug)]
struct PrincipalReply {
    pub p: Principal,
}

#[derive(CandidType, Serialize, Debug)]
struct PublicKeyReply {
    pub public_key: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug)]
struct SignatureReply {
    pub signature: Vec<u8>,
}

type CanisterId = Principal;

#[derive(CandidType, Serialize, Debug)]
struct ECDSAPublicKey {
    pub canister_id: Option<CanisterId>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
struct ECDSAPublicKeyReply {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Serialize, Debug)]
struct SignWithECDSA {
    pub message_hash: Vec<u8>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Deserialize, Debug)]
struct SignWithECDSAReply {
    pub signature: Vec<u8>,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
struct EcdsaKeyId {
    pub curve: EcdsaCurve,
    pub name: String,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub enum EcdsaCurve {
    #[serde(rename = "secp256k1")]
    Secp256k1,
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
struct InitArgs {
    pub key_id: String,
}

#[init]
async fn init(args: InitArgs) {
    let parsed_key_id: EcdsaKeyId = EcdsaKeyIds::try_from(args.key_id).unwrap().to_key_id();
    KEY_ID.with(|key| {
        let mut k = key.borrow_mut();
        *k = parsed_key_id;
    });
}

#[update]
async fn get_principal() -> Result<PrincipalReply, String> {
    let public_key = get_public_key().await?;
    Ok(PrincipalReply {
        p: Principal::self_authenticating(public_key),
    })
}

#[update]
async fn address(subaccount: Option<Vec<u8>>) -> String {
    let public_key = Principal::self_authenticating(get_public_key().await.unwrap());
    let sub: [u8; 32] = subaccount
        .unwrap_or(DEFAULT_SUBACCOUNT.0.to_vec())
        .try_into()
        .unwrap();
    AccountIdentifier::new(&public_key, &Subaccount(sub)).to_string()
}

#[update]
async fn public_key() -> Result<PublicKeyReply, String> {
    let public_key = get_public_key().await?;
    Ok(PublicKeyReply { public_key })
}

#[update]
async fn sign(message: Vec<u8>) -> Result<SignatureReply, String> {
    controller::require(&api::caller()).await;
    assert!(message.len() == 32);

    let request = SignWithECDSA {
        message_hash: message.clone(),
        derivation_path: derivation_path(),
        key_id: key_id(),
    };
    let (res,): (SignWithECDSAReply,) = api::call::call_with_payment(
        mgmt_canister_id(),
        "sign_with_ecdsa",
        (request,),
        25_000_000_000,
    )
    .await
    .map_err(|e| format!("Failed to call sign_with_ecdsa {}", e.1))?;

    Ok(SignatureReply {
        signature: res.signature,
    })
}

fn key_id() -> EcdsaKeyId {
    KEY_ID.with(|k| k.borrow().clone())
}

fn derivation_path() -> Vec<Vec<u8>> {
    vec![]
}

fn mgmt_canister_id() -> CanisterId {
    CanisterId::from_str("aaaaa-aa").unwrap()
}

enum EcdsaKeyIds {
    #[allow(unused)]
    TestKeyLocalDevelopment,
    #[allow(unused)]
    TestKey1,
    #[allow(unused)]
    ProductionKey1,
}

impl EcdsaKeyIds {
    fn to_key_id(&self) -> EcdsaKeyId {
        EcdsaKeyId {
            curve: EcdsaCurve::Secp256k1,
            name: match self {
                Self::TestKeyLocalDevelopment => "dfx_test_key",
                Self::TestKey1 => "test_key_1",
                Self::ProductionKey1 => "key_1",
            }
            .to_string(),
        }
    }
}

#[derive(CandidType, Deserialize, Debug)]
enum ParseEcdsaKeyIdError {
    UnknownKeyId,
}

impl TryFrom<String> for EcdsaKeyIds {
    type Error = ParseEcdsaKeyIdError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.as_str() {
            "dfx_test_key" => Ok(Self::TestKeyLocalDevelopment),
            "test_key_1" => Ok(Self::TestKey1),
            "key_1" => Ok(Self::ProductionKey1),
            _ => Err(ParseEcdsaKeyIdError::UnknownKeyId),
        }
    }
}

async fn get_public_key() -> Result<Vec<u8>, String> {
    let request = ECDSAPublicKey {
        canister_id: None,
        derivation_path: derivation_path(),
        key_id: key_id(),
    };
    let (res,): (ECDSAPublicKeyReply,) =
        ic_cdk::call(mgmt_canister_id(), "ecdsa_public_key", (request,))
            .await
            .map_err(|e| format!("Failed to call ecdsa_public_key {}", e.1))?;

    Ok(res.public_key)
}
