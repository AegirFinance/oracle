use ic_agent::{Agent, Identity};
use ic_types::Principal;

mod deposits;
mod governance;
mod disburse;
mod canister_updates;
mod split_new_withdrawal_neurons;

// TODO: Error handling throughout
#[tokio::main]
async fn main() -> Result<(), Error> {
    let ic_url = get_ic_url();
    let agent = create_agent(&ic_url).await;

    let deposits_canister_id = Principal::from_text("your_deposits_canister_id_here").unwrap();
    let deposits_address = address_from_principal(&deposits_canister_id);
    let d = deposits::Agent {
        agent: &agent,
        canister_id: deposits_canister_id,
    };

    let governance_canister_id = Principal::from_text("your_governance_canister_id_here").unwrap();
    let g = governance::Agent {
        agent: &agent,
        canister_id: governance_canister_id,
    };

    // Disburse any pending neurons
    disburse::run(&g, &deposits_address).await.map_err(Error::Governance)?;

    let neurons_to_split = canister_updates::run(&d).await.map_err(Error::Deposits)?;

    // List of hotkeys to add to each new neuron
    let hotkeys = vec![deposits_canister_id];

    // TODO Error handling
    split_new_withdrawal_neurons::run(&g, neurons_to_split, hotkeys).await.map_err(Error::Governance)?;

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Deposits(deposits::Error),
    Governance(governance::Error),
}

async fn create_agent(ic_url: &str) -> Agent {
    // TODO: Set up the agent with the appropriate configuration and identity
    // Refer to the ic_agent library documentation for details
    todo!("create_agent");
}

fn get_ic_url() -> String {
    todo!("get_ic_url");
}

fn address_from_principal(p: &Principal) -> Vec<u8> {
    todo!("address_from_principal");
}
