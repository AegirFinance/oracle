pub trait Service {
    async fn disburse_neuron(&self, neuron_id: u64) -> Result<(), Error>;
    async fn split_new_withdrawal_neurons(&self, neurons_to_split: Vec<(u64, u64)>) -> Result<(), Error>;
}

pub enum Error {
    // Add custom error types as needed, e.g. communication errors, invalid arguments, etc.
    CustomError(String),
}

pub struct Agent {
    agent: ic_agent::Agent,
    canister_id: ic_types::Principal,
}

impl Service for Agent {
    async fn disburse_neuron(&self, neuron_id: u64, address: &[u8]) -> Result<(), Error> {
        todo!("Agent.disburse_neuron");
    }

    async fn split_new_withdrawal_neurons(&self, neurons_to_split: Vec<(u64, u64)>) -> Result<(), Error> {
        todo!("Agent.split_new_withdrawal_neurons");
    }
}
