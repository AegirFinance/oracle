use async_trait::async_trait;
use ic_types::Principal;

#[async_trait]
pub trait Service {
    // Disburse all disburseable neurons to the target address
    async fn disburse_neurons(&self, address: &Vec<u8>) -> Result<(), Error>;
    // Apply the given list of neuron splits, adding the given hotkeys to each new neuron, and
    // starting the new neurons dissolving.
    async fn split_new_withdrawal_neurons(&self, neurons_to_split: Vec<(u64, u64)>, hotkeys: Vec<Principal>) -> Result<(), Error>;
}

#[derive(Debug)]
pub enum Error {
    // Add custom error types as needed, e.g. communication errors, invalid arguments, etc.
    Custom(String),
}

pub struct Agent<'a> {
    pub agent: &'a ic_agent::Agent,
    pub canister_id: ic_types::Principal,
}

#[async_trait]
impl Service for Agent<'_> {
    async fn disburse_neurons(&self, address: &Vec<u8>) -> Result<(), Error> {
        todo!("Agent.disburse_neurons");
    }

    async fn split_new_withdrawal_neurons(&self, neurons_to_split: Vec<(u64, u64)>, hotkeys: Vec<Principal>) -> Result<(), Error> {
        todo!("Agent.split_new_withdrawal_neurons");
    }
}
