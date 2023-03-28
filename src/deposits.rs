pub trait Service {
    fn address(&self) -> [u8];
    async fn get_disburseable_neurons(&self) -> Result<Vec<u64>, Error>;
    async fn refresh_neurons_and_apply_interest(&self) -> Result<Vec<(u64, u64)>, Error>;
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
    fn address(&self) -> [u8] {
        todo!("Agent.address");
    }

    async fn get_disburseable_neurons(&self) -> Result<Vec<u64>, Error> {
        todo!("Agent.get_disburseable_neurons");
    }

    async fn refresh_neurons_and_apply_interest(&self) -> Result<Vec<(u64, u64)>, Error> {
        todo!("Agent.refresh_neurons_and_apply_interest");
    }
}
