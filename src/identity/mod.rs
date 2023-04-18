use anyhow::{anyhow, Context};
use candid::Principal;
use clap::Args;
use ic_agent::{
    Agent,
    identity::{AnonymousIdentity, BasicIdentity, Secp256k1Identity},
    Identity,
};
use ic_base_types::PrincipalId;
use icp_ledger::AccountIdentifier;
use std::{
    str::FromStr,
    sync::Arc,
    path::{Path, PathBuf},
    time::Duration,
};
use tokio::runtime::Handle;

mod canister_identity;

const DEFAULT_IC_URL: &str = "https://ic0.app";
const DEFAULT_DEPOSITS_CANISTER_ID: &str = "hnwvc-lyaaa-aaaal-aaf6q-cai";
const DEFAULT_GOVERNANCE_CANISTER_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";

#[derive(Args, Debug)]
pub struct IdentityArgs {
    /// PEM Key file to use for the identity
    #[arg(long)]
    pub private_pem: Option<PathBuf>,

    // Principal of the ECDSA signing canister
    #[arg(long)]
    pub signing_canister: String,

    /// Principal of the deposits canister
    #[arg(long, default_value = DEFAULT_DEPOSITS_CANISTER_ID)]
    pub deposits: String,

    /// Principal of the governance canister
    #[arg(long, default_value = DEFAULT_GOVERNANCE_CANISTER_ID)]
    pub governance: String,

    /// Url of the IC replica
    #[arg(long, env = "IC_URL", default_value = DEFAULT_IC_URL)]
    pub ic_url: String,
}

impl IdentityArgs {
    pub fn should_fetch_root_key(&self) -> bool {
        self.ic_url != DEFAULT_IC_URL
    }

    pub async fn create_agent(&self) -> anyhow::Result<Agent> {
        let timeout = Duration::from_secs(60 * 5);
        let auth = self.get_auth()?;
        let identity = get_identity(&auth)?;
        let agent = Agent::builder()
            .with_transport(
                ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport::create(&self.ic_url)?,
            )
            .with_ingress_expiry(Some(timeout))
            .with_boxed_identity(identity)
            .build()
            .map_err(|err| anyhow!(err))?;

        if self.should_fetch_root_key() {
            // Not on the main net, we need to fetch the root key.
            agent.fetch_root_key().await?;
        }

        Ok(agent)
    }

    fn get_auth(&self) -> anyhow::Result<AuthInfo> {
        let handle = Handle::try_current().map_err(|e| anyhow!(e))?;
        // Get PEM from the file if provided, or try to convert from the seed file
        let local = match &self.private_pem {
            Some(pem_file) => AuthInfo::PemFile(read_file(pem_file, "PEM")?),
            None => AuthInfo::NoAuth,
        };
        // Wrap this in a canister-signer
        Ok(AuthInfo::Canister(CanisterInfo {
            signer: Principal::from_text(&self.signing_canister)?,
            local: Arc::from(get_identity(&local)?),
            fetch_root_key: self.should_fetch_root_key(),
            handle,
        }))
    }

    pub fn account_id(&self) -> anyhow::Result<AccountIdentifier> {
        PrincipalId::from_str(&self.signing_canister)
            .map(|p| AccountIdentifier::new(p, None))
            .map_err(|err| anyhow!(err))
    }
}


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

fn read_file(path: impl AsRef<Path>, name: &str) -> anyhow::Result<String> {
    let path = path.as_ref();
    if path == Path::new("-") {
        // read from STDIN
        let mut buffer = String::new();
        use std::io::Read;
        std::io::stdin()
            .read_to_string(&mut buffer)
            .map(|_| buffer)
            .context(format!("Couldn't read {} from STDIN", name))
    } else {
        std::fs::read_to_string(path).with_context(|| format!("Couldn't read {} file", name))
    }
}
