use anyhow::anyhow;
use async_trait::async_trait;
use candid::{CandidType, Decode, Encode, Principal};
use icp_ledger::{AccountIdentifier, AccountBalanceArgs, TransferArgs, TransferError};
use serde::Deserialize;

#[async_trait]
pub trait Service {
    async fn account_balance(&self, id: AccountIdentifier) -> anyhow::Result<u64>;
    async fn transfer(&self, to: AccountIdentifier, amount: u64, memo: u64) -> anyhow::Result<u64>;
}

pub struct Agent<'a> {
    pub agent: &'a ic_agent::Agent,
    pub canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Tokens {
    pub e8s: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub enum Result_1 {
    Ok(u64),
    Err(TransferError),
}

#[async_trait]
impl Service for Agent<'_> {
    async fn account_balance(&self, id: AccountIdentifier) -> anyhow::Result<u64> {
        let response = self
            .agent
            .update(&self.canister_id, "account_balance_dfx")
            .with_arg(&Encode!(&AccountBalanceArgs::new(id))?)
            .call_and_wait()
            .await?;

        let result = Decode!(response.as_slice(), Tokens)
            .map_err(|err| anyhow!(err))?;
        Ok(result.e8s)
    }

    async fn transfer(&self, to: AccountIdentifier, amount: u64, memo: u64) -> anyhow::Result<u64> {
        let response = self
            .agent
            .update(&self.canister_id, "transfer")
            .with_arg(&Encode!(&TransferArgs {
                memo: icp_ledger::Memo(memo),
                amount: icp_ledger::Tokens::from_e8s(amount),
                fee: icp_ledger::DEFAULT_TRANSFER_FEE,
                from_subaccount: None,
                to: to.to_address(),
                created_at_time: None,
            })?)
            .call_and_wait()
            .await?;

        let result = Decode!(response.as_slice(), Result_1)
            .map_err(|err| anyhow!(err))?;
        match result {
            Result_1::Ok(height) => Ok(height),
            Result_1::Err(err) => Err(anyhow!(err)),
        }
    }
}
