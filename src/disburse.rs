use std::env;
use ic_agent::{Agent, Identity};
use ic_types::Principal;

use crate::deposits;
use crate::governance;

pub async fn run(d: &deposits::Service, g: &governance::Service) -> Result<Vec<u64>, _> {
    // 1. Fetch a list of any disburseable neurons from the Deposits service
    let disburseable_neurons = fetch_disburseable_neurons(&d).await?;

    // 2. Disburse any disburseable neurons into the deposits canister
    let disbursed_neurons = disburse_neurons(&g, &d.address(), disburseable_neurons).await?;

    Ok(disbursed_neurons)
}

async fn fetch_disburseable_neurons(d: &deposits::Service) -> Vec<u64> {
    // TODO: Make an API call to the Deposits service to fetch disburseable neurons
    // The exact implementation depends on the API documentation
    let response: Result<Vec<u64>, _> = d.get_disburseable_neurons().await;
    match response {
        Ok(disburseable_neurons) => disburseable_neurons,
        Err(err) => {
            // TODO: Error-handling
            eprintln!("Error fetching disburseable neurons: {:?}", err);
            Vec::new()
        }
    }
}

async fn disburse_neurons(g: &governance::Service, deposits_address: &[u8], disburseable_neurons: Vec<u64>) -> Vec<u64> {
    let mut disbursed_neurons = Vec::new();

    for neuron_id in disburseable_neurons {
        let response: Result<(), _> = g.disburse_neuron(neuron_id, deposits_address).await;

        match response {
            Ok(_) => {
                println!("Disbursed neuron with ID: {}", neuron_id);
                disbursed_neurons.push(neuron_id);
            }
            Err(err) => {
                // TODO: Error handling
                eprintln!("Error disbursing neuron with ID {}: {:?}", neuron_id, err);
            }
        }
    }

    disbursed_neurons
}
