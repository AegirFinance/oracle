use async_trait::async_trait;
use candid::Principal;
use icp_ledger::AccountIdentifier;

#[async_trait]
pub trait Service {
    // Disburse all disburseable neurons to the target address
    // 1. Fetch a list of any disburseable neurons from the governance service
    // 2. Disburse any disburseable neurons into the deposits canister
    async fn disburse_neurons(&self, address: &AccountIdentifier) -> anyhow::Result<()>;

    // Apply the given list of neuron splits, adding the given hotkeys to each new neuron, and
    // starting the new neurons dissolving.
    //
    // These steps are handled previously in canister updates (for better atomicity), and passed in.
    // 1. Query dissolving neurons total & pending total, to calculate dissolving target from the
    //    Deposits service
    // 2. Calculate which staking neurons to split and how much

    // 3. Split & dissolve new neurons as needed
    async fn split_new_withdrawal_neurons(
        &self,
        neurons_to_split: Vec<(u64, u64)>,
        hotkeys: Vec<Principal>,
    ) -> anyhow::Result<()>;
}

pub struct Agent<'a> {
    pub agent: &'a ic_agent::Agent,
    pub canister_id: Principal,
}

#[async_trait]
impl Service for Agent<'_> {
    async fn disburse_neurons(&self, address: &AccountIdentifier) -> anyhow::Result<()> {
        todo!("Agent.disburse_neurons");
    }

    async fn split_new_withdrawal_neurons(
        &self,
        neurons_to_split: Vec<(u64, u64)>,
        hotkeys: Vec<Principal>,
    ) -> anyhow::Result<()> {
        todo!("Agent.split_new_withdrawal_neurons");
    }
}
