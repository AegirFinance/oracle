use anyhow::anyhow;
use candid::{CandidType, Decode, Encode};
use crossbeam::channel;
use ic_agent::{export::Principal, Agent, Identity, Signature};
use k256::{
    ecdsa::{self, signature::Signer, SigningKey, VerifyingKey},
    pkcs8::{Document, EncodePublicKey},
    sha2::{Digest, Sha256},
    PublicKey,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::convert::TryInto;
use std::sync::Arc;
use tokio::runtime::Handle;

#[derive(CandidType, Deserialize, Debug)]
struct PublicKeyArgument {}

#[derive(CandidType, Deserialize, Debug)]
struct PublicKeyReply {
    pub public_key: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug)]
struct SignatureReply {
    pub signature: Vec<u8>,
}

pub struct CanisterIdentity {
    pub canister: Principal,
    pub identity: Arc<dyn Identity>,
    pub ic_url: String,
    pub fetch_root_key: bool,
    pub handle: Handle,
}

impl CanisterIdentity {
    pub fn new(
        canister: Principal,
        identity: Arc<dyn Identity>,
        ic_url: String,
        fetch_root_key: bool,
        handle: Handle,
    ) -> Self {
        Self {
            canister,
            identity,
            ic_url,
            fetch_root_key,
            handle,
        }
    }

    pub fn canister_update<A, R>(&self, method_name: &str, arg: &A) -> Result<R, String>
    where
        A: CandidType,
        R: CandidType + DeserializeOwned,
    {
        let (tx, rx) = channel::bounded(1);
        let identity = self.identity.clone();
        let canister = self.canister.clone();
        let ic_url = self.ic_url.clone();
        let fetch_root_key = self.fetch_root_key.clone();
        let arg_bytes = Encode!(&arg).map_err(|e| format!("{e}"))?;
        let method = method_name.to_string();
        self.handle.spawn(async move {
            let agent = get_agent_async(identity, &ic_url, fetch_root_key).await;
            let _ = tx.send(match agent {
                Err(e) => Err(e),
                Ok(agent) => agent
                    .update(&canister, method)
                    .with_arg(&arg_bytes)
                    .call_and_wait()
                    .await
                    .map_err(|err| anyhow!(err)),
            });
        });
        let r = rx.recv();
        let bytes = r.map_err(|e| format!("{e}"))?.map_err(|e| format!("{e}"))?;
        let result = Decode!(&bytes, Result<R, String>).map_err(|e| format!("{e}"))?;
        return Ok(result?);
    }

    fn public_key(&self) -> Result<Vec<u8>, String> {
        let result: PublicKeyReply = self.canister_update("public_key", &PublicKeyArgument {})?;
        assert!(
            result.public_key.len() == 33,
            "malformed public_key, len: {}, expected 33",
            result.public_key.len()
        );
        let verifying_key =
            VerifyingKey::from_sec1_bytes(&result.public_key).map_err(|e| format!("{e}"))?;
        let public_key: PublicKey = verifying_key.into();
        let key_der = public_key.to_public_key_der().map_err(|e| format!("{e}"))?;
        Ok(key_der.as_bytes().to_vec())
    }
}

impl Identity for CanisterIdentity {
    fn sender(&self) -> Result<Principal, String> {
        Ok(Principal::self_authenticating(self.public_key()?))
    }

    fn sign(&self, blob: &[u8]) -> Result<Signature, String> {
        let mut hasher = Sha256::new();
        hasher.update(blob);
        let message: [u8; 32] = hasher.finalize().as_slice().try_into().unwrap();
        let result: SignatureReply = self.canister_update("sign", &message)?;
        Ok(Signature {
            public_key: Some(self.public_key()?),
            signature: Some(result.signature),
        })
    }
}

fn get_agent(identity: Arc<dyn Identity>, ic_url: &str) -> anyhow::Result<Agent> {
    let timeout = std::time::Duration::from_secs(60 * 5);
    Agent::builder()
        .with_transport(
            ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport::create({
                ic_url
            })?,
        )
        .with_ingress_expiry(Some(timeout))
        .with_arc_identity(identity)
        .build()
        .map_err(|err| anyhow!(err))
}

async fn get_agent_async(
    identity: Arc<dyn Identity>,
    ic_url: &str,
    fetch_root_key: bool,
) -> anyhow::Result<Agent> {
    let agent = get_agent(identity, ic_url)?;
    if fetch_root_key {
        agent.fetch_root_key().await?;
    }
    Ok(agent)
}
