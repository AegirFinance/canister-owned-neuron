//! All the common functionality.

use anyhow::{anyhow, bail, Context};
use candid::{
    parser::typing::{check_prog, TypeEnv},
    types::Function,
    IDLProg, Principal,
};
use ic_agent::{
    identity::AnonymousIdentity,
    Agent, Identity,
};
use serde_cbor::Value;
use std::sync::Arc;

pub const IC_URL: &str = "https://ic0.app";

// The OID of secp256k1 curve is `1.3.132.0.10`.
// Encoding in DER results in following bytes.
const EC_PARAMETERS: [u8; 7] = [6, 5, 43, 129, 4, 0, 10];

#[derive(Debug)]
pub enum AuthInfo {
    NoAuth, // No authentication details were provided;
}

/// Returns an agent with an identity derived from a private key if it was
/// provided.
pub fn get_agent(auth: &AuthInfo, ic_url: &str) -> anyhow::Result<Agent> {
    let timeout = std::time::Duration::from_secs(60 * 5);
    let builder = Agent::builder()
        .with_transport(
            // TODO: Replace with outbount HTTP Calls
            ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport::create({
                ic_url
            })?,
        )
        .with_ingress_expiry(Some(timeout));

    let identity = get_identity(auth)?;
    builder
        .with_boxed_identity(identity)
        .build()
        .map_err(|err| anyhow!(err))
}

/// Returns an identity derived from the private key.
pub fn get_identity(auth: &AuthInfo) -> anyhow::Result<Box<dyn Identity>> {
    match auth {
        AuthInfo::NoAuth => Ok(Box::new(AnonymousIdentity) as _),
    }
}
