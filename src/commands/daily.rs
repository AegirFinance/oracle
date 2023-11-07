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
    pub async fn run(&self) -> anyhow::Result<()> {
        let agent = self.identity.create_agent().await?;
        let local_agent = self.identity.create_local_agent().await?;

        let deposits_canister_id = Principal::from_text(&self.identity.deposits_canister)?;
        let d = deposits::Agent {
            agent: &local_agent,
            canister_id: deposits_canister_id,
        };
        let deposits_address = d.account_id()?;

        let governance_canister_id = Principal::from_text(&self.identity.governance)?;
        let g = governance::Agent {
            agent: &agent,
            canister_id: governance_canister_id,
        };

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        // Disburse any pending neurons
        eprintln!("Disbursing any pending neurons");
        let neurons_to_disburse = d.list_neurons_to_disburse(now).await?;
        eprintln!("Found {} neurons to disburse", neurons_to_disburse.len());
        g.disburse_neurons(&deposits_address, &neurons_to_disburse).await?;

        // Run canister updates and figure out which neurons to split
        eprintln!("Refreshing staking neurons and applying interest");
        let neurons_to_split = d.refresh_neurons_and_apply_interest().await?;

        eprintln!("Splitting {} neurons", neurons_to_split.len());
        let neurons_to_replace = g.split_new_withdrawal_neurons(neurons_to_split).await?;

        for (id, new_id) in neurons_to_replace.iter() {
            d.replace_staking_neuron(id.clone(), new_id.clone()).await?;
            eprintln!("Replaced neuron {} with new neuron {}", id, new_id);
        }

        Ok(())
    }
}
