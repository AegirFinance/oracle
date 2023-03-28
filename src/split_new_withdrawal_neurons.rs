use ic_types::Principal;

use crate::governance;

pub async fn run(g: &impl governance::Service, neurons_to_split: Vec<(u64, u64)>, hotkeys: Vec<Principal>) -> Result<(), governance::Error> {
    // These steps are handled previously in canister updates (for better atomicity), and passed in.
    // 1. Query dissolving neurons total & pending total, to calculate dissolving target from the
    //    Deposits service
    // 2. Calculate which staking neurons to split and how much


    // 3. Split & dissolve new neurons as needed
    g.split_new_withdrawal_neurons(neurons_to_split, hotkeys).await
}
