use anyhow::{anyhow, Context};
use candid::Principal;
use clap::Parser;
use ic_agent::Agent;
use ic_base_types::PrincipalId;
use icp_ledger::AccountIdentifier;
use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::runtime::Runtime;

mod deposits;
mod governance;
mod identity;

const DEFAULT_IC_URL: &str = "https://ic0.app";
const DEFAULT_DEPOSITS_CANISTER_ID: &str = "hnwvc-lyaaa-aaaal-aaf6q-cai";
const DEFAULT_GOVERNANCE_CANISTER_ID: &str = "rrkah-fqaaa-aaaaa-aaaaq-cai";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// PEM Key file to use for the identity
    #[arg(long)]
    private_pem: Option<PathBuf>,

    // Principal of the ECDSA signing canister
    #[arg(long)]
    signing_canister: String,

    /// Principal of the deposits canister
    #[arg(long, default_value = DEFAULT_DEPOSITS_CANISTER_ID)]
    deposits: String,

    /// Principal of the governance canister
    #[arg(long, default_value = DEFAULT_GOVERNANCE_CANISTER_ID)]
    governance: String,

    /// Url of the IC replica
    #[arg(long, env = "IC_URL", default_value = DEFAULT_IC_URL)]
    ic_url: String,
}

impl Args {
    fn fetch_root_key(&self) -> bool {
        self.ic_url != DEFAULT_IC_URL
    }

    fn get_auth(&self, handle: tokio::runtime::Handle) -> anyhow::Result<identity::AuthInfo> {
        // Get PEM from the file if provided, or try to convert from the seed file
        let local = match &self.private_pem {
            Some(pem_file) => identity::AuthInfo::PemFile(read_file(pem_file, "PEM")?),
            None => identity::AuthInfo::NoAuth,
        };
        // Wrap this in a canister-signer
        Ok(identity::AuthInfo::Canister(identity::CanisterInfo {
            signer: Principal::from_text(&self.signing_canister)?,
            local: Arc::from(identity::get_identity(&local)?),
            fetch_root_key: self.fetch_root_key(),
            handle,
        }))
    }
}

// TODO: Error handling throughout
// TODO: Logging throughout
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let runtime = Runtime::new().expect("Unable to create a runtime");
    let handle = runtime.handle().clone();
    let agent = create_agent(&args.ic_url, &args.get_auth(handle)?).await?;

    let deposits_canister_id = Principal::from_text(args.deposits)?;
    let deposits_address = get_account_id(&deposits_canister_id)?;
    let d = deposits::Agent {
        agent: &agent,
        canister_id: deposits_canister_id,
    };

    let governance_canister_id = Principal::from_text(args.governance)?;
    let g = governance::Agent {
        agent: &agent,
        canister_id: governance_canister_id,
    };

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    // Disburse any pending neurons
    governance::Service::disburse_neurons(&g, now, &deposits_address).await?;

    // Run canister updates and figure out which neurons to split
    let neurons_to_split = deposits::Service::refresh_neurons_and_apply_interest(&d).await?;

    // TODO Error handling
    governance::Service::split_new_withdrawal_neurons(&g, neurons_to_split).await?;

    Ok(())
}

async fn create_agent(ic_url: &str, auth: &identity::AuthInfo) -> anyhow::Result<Agent> {
    let timeout = Duration::from_secs(60 * 5);
    let identity = identity::get_identity(auth)?;
    let agent = Agent::builder()
        .with_transport(
            ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport::create(ic_url)?,
        )
        .with_ingress_expiry(Some(timeout))
        .with_boxed_identity(identity)
        .build()
        .map_err(|err| anyhow!(err))?;

    if ic_url != DEFAULT_IC_URL {
        // Not on the main net, we need to fetch the root key.
        agent.fetch_root_key().await?;
    }

    Ok(agent)
}

fn get_account_id(principal_id: &Principal) -> anyhow::Result<AccountIdentifier> {
    let base_types_principal =
        PrincipalId::try_from(principal_id.as_slice()).map_err(|err| anyhow!(err))?;
    Ok(AccountIdentifier::new(base_types_principal, None))
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
