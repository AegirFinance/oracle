use anyhow::{anyhow, bail};
use candid::Principal;
use clap::Args;
use icp_ledger::{AccountIdentifier, Subaccount};
use k256::sha2::{Digest, Sha256};
use rand::Rng;
use std::time::SystemTime;

use crate::deposits::{self, Service as DepositsService};
use crate::governance::{self, Service as GovernanceService};
use crate::identity;
use crate::ledger::{self, Service as LedgerService};

const DEFAULT_ICP_LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";

#[derive(Args, Debug)]
pub struct Command {
    #[command(flatten)]
    identity: identity::IdentityArgs,

    /// Principal of the deposits canister
    #[arg(long, default_value = DEFAULT_ICP_LEDGER_CANISTER_ID)]
    icp_ledger: String,

    /// Memo to use when creating the neuron, if 0, use random number
    #[arg(long, default_value = "0")]
    memo: u64,

    /// Delay to set on the neuron,
    #[arg(long, default_value = "0")]
    delay: u32,
}

impl Command {
    pub async fn run(&self) -> anyhow::Result<()> {
        let agent = self.identity.create_agent().await?;
        let local_agent = self.identity.create_local_agent().await?;

        let deposits_principal = Principal::from_text(&self.identity.deposits_canister)?;

        let governance_principal = Principal::from_text(&self.identity.governance)?;
        let g = governance::Agent {
            agent: &agent,
            canister_id: governance_principal,
        };

        let identity_principal = self.identity.principal().await?;

        let icp = ledger::Agent {
            agent: &local_agent,
            canister_id: Principal::from_text(&self.icp_ledger)?,
        };

        let memo = if self.memo == 0 {
            // Pick a random memo
            rand::thread_rng().gen()
        } else {
            self.memo
        };

        let address = neuron_account_id(governance_principal, identity_principal, memo)?;

        // Transfer 1 ICP
        eprintln!("Transfer 1 ICP to {}, memo: {}", address.to_hex(), memo);
        let height = icp.transfer(address, 100_000_000, memo).await?;
        eprintln!("Transferred at block height: {}", height);

        // Create the Neuron
        let neuron_id = g.claim_neuron(Some(identity_principal), memo).await?;
        eprintln!("Created neuron: {}", neuron_id);

        eprintln!("Add hot key to neuron: {}", deposits_principal);
        g.add_hotkey(neuron_id, deposits_principal).await?;

        if self.delay > 0 {
            eprintln!("Set the neuron delay: {}", self.delay);
            g.increase_neuron_delay(neuron_id, self.delay).await?;
        }

        eprintln!("Enabling auto-merge-maturity");
        g.enable_auto_merge_maturity(neuron_id).await?;

        println!("{}", neuron_id);
        Ok(())
    }
}

fn neuron_account_id(
    governance: Principal,
    controller: Principal,
    nonce: u64,
) -> anyhow::Result<AccountIdentifier> {
    let mut hasher = Sha256::new();
    hasher.update(vec![
        0x0c, 0x6e, 0x65, 0x75, 0x72, 0x6f, 0x6e, 0x2d, 0x73, 0x74, 0x61, 0x6b, 0x65,
    ]);
    hasher.update(controller.as_slice());
    hasher.update(nonce.to_be_bytes());
    let subaccount: Subaccount = hasher.finalize().as_slice().try_into()?;

    Ok(AccountIdentifier::new(governance.into(), Some(subaccount)))
}
