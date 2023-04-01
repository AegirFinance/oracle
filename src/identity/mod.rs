use anyhow::Context;
use candid::Principal;
use ic_agent::{
    identity::{AnonymousIdentity, BasicIdentity, Secp256k1Identity},
    Identity,
};
use std::sync::Arc;

mod canister_identity;

#[derive(Debug)]
pub struct CanisterInfo {
    pub signer: Principal,
    pub local: Arc<dyn Identity>,
    pub fetch_root_key: bool,
    pub handle: tokio::runtime::Handle,
}

#[derive(Debug)]
pub enum AuthInfo {
    NoAuth, // No authentication details were provided;
    // only unsigned queries are allowed.
    PemFile(String),        // --private-pem file specified
    Canister(CanisterInfo), // --canister-signer principal specified
}

/// Returns an identity derived from the private key.
pub fn get_identity(auth: &AuthInfo) -> anyhow::Result<Box<dyn Identity>> {
    match auth {
        AuthInfo::NoAuth => Ok(Box::new(AnonymousIdentity) as _),
        AuthInfo::PemFile(pem) => match Secp256k1Identity::from_pem(pem.as_bytes()) {
            Ok(id) => Ok(Box::new(id) as _),
            Err(_) => match BasicIdentity::from_pem(pem.as_bytes()) {
                Ok(id) => Ok(Box::new(id) as _),
                Err(e) => Err(e).context("couldn't load identity from PEM file"),
            },
        },
        AuthInfo::Canister(info) => Ok(Box::new(canister_identity::CanisterIdentity::new(
            info.signer,
            info.local.clone(),
            info.fetch_root_key,
            info.handle.clone(),
        ))),
    }
}
