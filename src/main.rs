use std::env;
use ic_agent::{Agent, Identity};
use ic_types::Principal;

mod deposits;
mod governance;
mod disburse;
mod canister_updates;
mod split_new_withdrawal_neurons;

// TODO: Error handling throughout
#[tokio::main]
async fn main() {
    let ic_url = get_ic_url();
    let agent = create_agent(&ic_url);

    let deposits_canister_id = Principal::from_text("your_deposits_canister_id_here").unwrap();
    let d = deposits::Agent {
        agent,
        canister_id: deposits_canister_id,
    };

    let governance_canister_id = Principal::from_text("your_governance_canister_id_here").unwrap();
    let g = governance::Agent {
        agent,
        canister_id: governance_canister_id,
    };

    // Disburse any pending neurons
    disburse::run(&d, &g).await;

    let result = canister_updates::run(&d).await;
    let neurons_to_split = match result {
        Ok(n) => n,
        Err(err) => {
            // TODO: Error handling
            eprintln!(format!("Error: {:?}", err));
            return;
        }
    }


    split_new_withdrawal_neurons::run(&d, &g, neurons_to_split).await;
}

async fn create_agent() -> Agent {
    // TODO: Set up the agent with the appropriate configuration and identity
    // Refer to the ic_agent library documentation for details
}

