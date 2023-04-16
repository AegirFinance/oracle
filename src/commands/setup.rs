use anyhow::bail;
use candid::Principal;
use clap::Args;
use rand::Rng;
use std::time::SystemTime;

use crate::deposits::{self, Service as DepositsService};
use crate::governance::{self, Service as GovernanceService};
use crate::ledger::{self, Service as LedgerService};
use crate::identity;

const DEFAULT_ICP_LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";

#[derive(Args, Debug)]
pub struct Command {
    #[command(flatten)]
    identity: identity::IdentityArgs,

    /// Principal of the deposits canister
    #[arg(long, default_value = DEFAULT_ICP_LEDGER_CANISTER_ID)]
    icp_ledger: String,
}

impl Command {
    // TODO: Error handling throughout
    // TODO: Logging throughout
    pub async fn run(&self) -> anyhow::Result<()> {
        let mut rng = rand::thread_rng();
        let agent = self.identity.create_agent().await?;

        let deposits_principal = Principal::from_text(&self.identity.deposits)?;
        let d = deposits::Agent {
            agent: &agent,
            canister_id: deposits_principal,
        };

        let g = governance::Agent {
            agent: &agent,
            canister_id: Principal::from_text(&self.identity.governance)?,
        };

        let icp = ledger::Agent {
            agent: &agent,
            canister_id: Principal::from_text(&self.icp_ledger)?,
        };

        let delays = vec![
            15778800, 31557600, 47336400, 63115200, 78894000, 94672800, 110451600, 126230400,
            142009200, 157788000, 173566800, 189345600, 205124400, 220903200, 236682000, 252460800,
        ];

        // Create the new neurons (mirror delays from existing staking neurons)
        let mut new_neuron_ids: Vec<u64> = vec![];
        for delay in delays.iter() {
            // Pick a random memo
            let memo: u64 = rng.gen();
            // Transfer 1 ICP
            icp.transfer(g.account_id()?, 100_000_000, memo).await?;
            // Create the neuron
            let neuron_id = g.claim_neuron(memo).await?;
            new_neuron_ids.push(neuron_id);
            g.increase_neuron_delay(neuron_id, *delay).await?;
            g.add_hotkey(neuron_id, deposits_principal).await?;
            g.enable_auto_merge_maturity(neuron_id).await?;
        }

        // Remove old neurons from the deposits canister
        // - staking neurons
        // - withdrawal neurons

        // Add the new neurons to the deposits canister

        // Transfer the remaining total amount to the deposits canister

        // Pay out pending deposits and top up neurons by calling flushPendingDeposits

        // deposits.setTotalMaturity(0)


        Ok(())
    }
}
