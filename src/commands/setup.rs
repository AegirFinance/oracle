use candid::Principal;
use clap::Args;
use std::time::SystemTime;

use crate::deposits::{self, Service as DepositsService};
use crate::governance::{self, Service as GovernanceService};
use crate::identity;

#[derive(Args, Debug)]
pub struct Command {
    #[command(flatten)]
    identity: identity::IdentityArgs,
}

impl Command {
    // TODO: Error handling throughout
    // TODO: Logging throughout
    pub async fn run(&self) -> anyhow::Result<()> {
        let agent = self.identity.create_agent().await?;

        let deposits_canister_id = Principal::from_text(&self.identity.deposits)?;
        let d = deposits::Agent {
            agent: &agent,
            canister_id: deposits_canister_id,
        };

        let governance_canister_id = Principal::from_text(&self.identity.governance)?;
        let g = governance::Agent {
            agent: &agent,
            canister_id: governance_canister_id,
        };

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        // Check we have enough balance to do this job
        // We should have enough to:
        // - Buy out all neurons
        // - Pay out all pending withdrawals
        // - Match the deposits canister balance

        // Create the new neurons

        // Add the new neurons to the deposits canister

        // Transfer the rest to the deposits canister

        // Pay out pending deposits

        Ok(())
    }
}
