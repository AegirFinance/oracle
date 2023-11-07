use anyhow::anyhow;
use async_trait::async_trait;
use candid::{CandidType, Decode, Encode, Principal};
use ic_base_types::PrincipalId;
use ic_nns_governance::pb::v1::neuron::DissolveState;
use icp_ledger::AccountIdentifier;
use serde::Deserialize;

#[async_trait]
pub trait Service {
    async fn list_neurons_to_disburse(&self, now: u64) -> anyhow::Result<Vec<u64>>;

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
    async fn refresh_neurons_and_apply_interest(&self) -> anyhow::Result<Vec<(u64, u64)>>;

    // Calculate the deposit canister's account id for disbursing neurons to
    fn account_id(&self) -> anyhow::Result<AccountIdentifier>;
}

pub struct Agent<'a> {
    pub agent: &'a ic_agent::Agent,
    pub canister_id: Principal,
}

#[derive(CandidType)]
pub struct ListNeuronsToDisburseArgs {}

pub type ListNeuronsToDisburseResult = Vec<Neuron>;

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Neuron {
    pub id: u64,
    #[serde(rename = "accountId")]
    pub account_id: AccountIdentifier,
    #[serde(rename = "dissolveState")]
    pub dissolve_state: Option<DissolveState>,
    #[serde(rename = "cachedNeuronStakeE8s")]
    pub cached_neuron_stake_e8s: u64,
    #[serde(rename = "stakedMaturityE8sEquivalent")]
    pub staked_maturity_e8s_equivalent: Option<u64>,
}

#[derive(CandidType)]
pub struct RefreshNeuronsAndApplyInterestArgs {}

pub type RefreshNeuronsAndApplyInterestResult = Vec<(u64, u64)>;

#[async_trait]
impl Service for Agent<'_> {
    async fn list_neurons_to_disburse(&self, now: u64) -> anyhow::Result<Vec<u64>> {
        let response = self
            .agent
            .update(&self.canister_id, "listNeuronsToDisburse")
            .with_arg(&Encode!(&ListNeuronsToDisburseArgs {})?)
            .call_and_wait()
            .await?;

        let result = Decode!(response.as_slice(), ListNeuronsToDisburseResult)
            .map_err(|err| anyhow!(err))?
            .iter()
            .filter(|n| {
                let Some(DissolveState::WhenDissolvedTimestampSeconds(dissolved_at)) = n.dissolve_state else {
                    return false;
                };
                if now < dissolved_at {
                    return false;
                }
                return true;
            })
            .map(|n| n.id)
            .collect();
        Ok(result)
    }

    async fn refresh_neurons_and_apply_interest(&self) -> anyhow::Result<Vec<(u64, u64)>> {
        let response = self
            .agent
            .update(&self.canister_id, "refreshNeuronsAndApplyInterest")
            .with_arg(&Encode!(&RefreshNeuronsAndApplyInterestArgs {})?)
            .call_and_wait()
            .await?;

        let result = Decode!(response.as_slice(), RefreshNeuronsAndApplyInterestResult)
            .map_err(|err| anyhow!(err))?;
        Ok(result)
    }

    fn account_id(&self) -> anyhow::Result<AccountIdentifier> {
        PrincipalId::try_from(self.canister_id.as_slice())
            .map(|p| AccountIdentifier::new(p, None))
            .map_err(|err| anyhow!(err))
    }
}
