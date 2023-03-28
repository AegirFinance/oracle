use crate::deposits;
use crate::governance;

pub async fn run(g: &impl governance::Service, deposits_address: &Vec<u8>) -> Result<(), governance::Error> {
    // 1. Fetch a list of any disburseable neurons from the governance service
    // 2. Disburse any disburseable neurons into the deposits canister
    g.disburse_neurons(deposits_address).await
}
