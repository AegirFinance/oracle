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
}

impl Command {
    // TODO: Error handling throughout
    // TODO: Logging throughout
    pub async fn run(&self) -> anyhow::Result<()> {
        println!("Starting Setup");
        let mut rng = rand::thread_rng();
        let agent = self.identity.create_agent().await?;

        let deposits_principal = Principal::from_text(&self.identity.deposits_canister)?;
        let d = deposits::Agent {
            agent: &agent,
            canister_id: deposits_principal,
        };

        let governance_principal = Principal::from_text(&self.identity.governance)?;
        let g = governance::Agent {
            agent: &agent,
            canister_id: governance_principal,
        };

        let identity_principal = self.identity.principal().await?;
        let identity_account_id = AccountIdentifier::new(identity_principal.into(), None);

        let icp = ledger::Agent {
            agent: &agent,
            canister_id: Principal::from_text(&self.icp_ledger)?,
        };

        let delays = vec![
            15778800, 31557600, 47336400, 63115200, 78894000, 94672800, 110451600, 126230400,
            142009200, 157788000, 173566800, 189345600, 205124400, 220903200, 236682000, 252460800,
        ];

        let initial_balance = icp.account_balance(identity_account_id).await?;
        println!("Initial account balance: {}", initial_balance);
        let minimum_required: u64 = ((delays.len() as u64) * 100_000_000) + 10_000;
        if initial_balance < minimum_required {
            bail!(
                "Initial Balance is too low. Requires >= {}. Please transfer to account: {}",
                minimum_required,
                identity_account_id.to_hex()
            );
        }

        println!("Create the new neurons");
        // Create the new neurons (mirror delays from existing staking neurons)
        let mut new_neuron_ids: Vec<u64> = vec![];
        for delay in delays.iter() {
            // Pick a random memo
            let memo: u64 = rng.gen();
            println!("Delay: {}, Memo: {}", delay, memo);
            let address = neuron_account_id(governance_principal, identity_principal, memo)?;
            // Transfer 1 ICP
            println!("Transferring 1 ICP to: {}", address.to_hex());
            let height = icp.transfer(address, 100_000_000, memo).await?;
            println!("Transferred at block height: {}", height);
            // Create the neuron
            let neuron_id = g.claim_neuron(Some(identity_principal), memo).await?;
            println!("Created Neuron: {}", neuron_id);
            new_neuron_ids.push(neuron_id);
            eprintln!("Adding hotkey: {}", deposits_principal);
            g.add_hotkey(neuron_id, deposits_principal).await?;
            eprintln!("Increasing delay to: {}", *delay);
            g.increase_neuron_delay(neuron_id, *delay).await?;
            eprintln!("Enabling auto-merge-maturity");
            g.enable_auto_merge_maturity(neuron_id).await?;
        }
        println!("Created {} new neurons", new_neuron_ids.len());

        // Remove old neurons from the deposits canister
        // - staking neurons
        // - withdrawal neurons
        // deposits.setTotalMaturity(0)
        // Add the new neurons to the deposits canister
        println!("Reset the deposits canister");
        d.reset_staking_neurons(&new_neuron_ids).await?;

        // Transfer the remaining total amount to the deposits canister
        let balance = icp.account_balance(identity_account_id).await?;
        println!(
            "Transfer the remaining balance to the deposits canister: {} e8s",
            balance - 10_000
        );
        icp.transfer(d.account_id()?, balance - 10_000, 0).await?;

        // Pay out pending deposits and top up neurons by calling flushPendingDeposits
        println!("Flush pending deposits to rebalance neurons");
        d.flush_pending_deposits().await?;

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
