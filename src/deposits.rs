#[async_trait]
pub trait Service {
    async fn refresh_neurons_and_apply_interest(&self) -> Result<Vec<(u64, u64)>, Error>;
}

#[derive(Debug)]
pub enum Error {
    // Add custom error types as needed, e.g. communication errors, invalid arguments, etc.
    Custom(String),
}

pub struct Agent<'a> {
    agent: &'a ic_agent::Agent,
    canister_id: ic_types::Principal,
}

#[async_trait]
impl Service for Agent<'_> {
    async fn refresh_neurons_and_apply_interest(&self) -> Result<Vec<(u64, u64)>, Error> {
        todo!("Agent.refresh_neurons_and_apply_interest");
    }
}
