use anyhow::anyhow;
use candid::Principal;

use ic_agent::{Agent, Identity};
use ic_base_types::PrincipalId;
use icp_ledger::AccountIdentifier;
use std::{
    convert::TryFrom,
    env,
    time::{Duration, SystemTime},
};

mod deposits;
mod governance;

// TODO: Error handling throughout
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ic_url = get_ic_url();
    let agent = create_agent(&ic_url).await?;

    let deposits_canister_id = Principal::from_text("your_deposits_canister_id_here").unwrap();
    let deposits_address = get_account_id(&deposits_canister_id)?;
    let d = deposits::Agent {
        agent: &agent,
        canister_id: deposits_canister_id,
    };

    let governance_canister_id = Principal::from_text("your_governance_canister_id_here").unwrap();
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

    // List of hotkeys to add to each new neuron
    let hotkeys = vec![deposits_canister_id];

    // TODO Error handling
    governance::Service::split_new_withdrawal_neurons(&g, neurons_to_split, hotkeys).await?;

    Ok(())
}

// TODO: Set up the agent with the appropriate configuration and identity
async fn create_agent(ic_url: &str) -> anyhow::Result<Agent> {
    let timeout = Duration::from_secs(60 * 5);
    let agent = Agent::builder()
        .with_transport(
            ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport::create({
                get_ic_url()
            })?,
        )
        .with_ingress_expiry(Some(timeout))
        //.with_boxed_identity(get_identity(auth)?)
        .build()
        .map_err(|err| anyhow!(err))?;

    if ic_url != IC_URL {
        // Not on the main net, we need to fetch the root key.
        agent.fetch_root_key().await?;
    }

    Ok(agent)
}

const IC_URL: &str = "https://ic0.app";

fn get_ic_url() -> String {
    env::var("IC_URL").unwrap_or_else(|_| IC_URL.to_string())
}

fn get_account_id(principal_id: &Principal) -> anyhow::Result<AccountIdentifier> {
    let base_types_principal =
        PrincipalId::try_from(principal_id.as_slice()).map_err(|err| anyhow!(err))?;
    Ok(AccountIdentifier::new(base_types_principal, None))
}
