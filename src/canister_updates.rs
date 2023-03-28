use std::env;
use ic_agent::{Agent, Identity};
use ic_types::Principal;

use crate::deposits;
use crate::governance;

// TODO: Could we set auto-merge-maturity and simplify this by just tracking the balance? The
// balance shouldn't change unless we flush through, and it would simplify the job.
pub async fn run(d: &deposits::Service) -> Result<Vec<(u64, u64)>, _> {
    // This will do all of the following in the canister:
    //
    // 1. Garbage-collect disbursed neurons from the withdrawal module tracking
    //    a. This should figure out which neurons *might* have been disbursed, and querying the
    //    governance canister to confirm their state. This will make it idempotent.
    //    b. If there are unknown dissolving neurons, they should be considered as new withdrawal
    //    neurons. This will make it idempotent.
    // 2. Apply Interest
    //    a. Update cached neuron stakes (to figure out how much interest we gained today)
    //    b. Take a new holders snapshot for the next day
    //    c. Mint new tokens to holders
    //    d. Update the holders snapshot for tomorrow
    //    e. Log interest and update meanAprMicrobips
    // 3. Flush Pending Deposits
    //    a. Query token total supply & canister balance
    //    b. Fulfill pending deposits from canister balance if possible
    //    c. Deposit incoming ICP into neurons
    //    d. Refresh staking neuron balances & cache
    // 4. Split New Withdrawal Neurons
    //    a. Query dissolving neurons total & pending total, to calculate dissolving target
    //    b. Calculate which staking neurons to split and how much
    //
    // This is all done in a single call, so that it is more atomic (not fully), and there is less
    // back-and-forth between this script and the canister.
    d.refresh_neurons_and_apply_interest().await
}
