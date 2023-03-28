use crate::deposits;
use crate::governance;

async fn run(d: deposits::Service, g: governance::Service, neurons_to_split: Vec<(u64, u64)>) -> Result<(), _> {
    // These steps are handled previously in canister updates (for better atomicity), and passed in.
    // 1. Query dissolving neurons total & pending total, to calculate dissolving target from the
    //    Deposits service
    // 2. Calculate which staking neurons to split and how much


    // 3. Split & dissolve new neurons as needed
    g.split_new_withdrawal_neurons(g, neurons_to_split).await?;
}
