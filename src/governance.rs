use anyhow::{anyhow, bail};
use async_trait::async_trait;
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use icp_ledger::AccountIdentifier;

const ICP_FEE: u64 = 10_000;

#[async_trait]
pub trait Service {
    // Disburse all disburseable neurons to the target address
    // 1. Fetch a list of any disburseable neurons from the governance service
    // 2. Disburse any disburseable neurons into the deposits canister
    async fn disburse_neurons(&self, now: u64, address: &AccountIdentifier) -> anyhow::Result<()>;

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

impl Agent<'_> {
    // TODO: Load the args etc here from a local candid file
    async fn list_neurons(&self) -> anyhow::Result<Vec<Neuron>> {
        let response = self
            .agent
            .update(&self.canister_id, "list_neurons")
            .with_arg(&Encode!(&ListNeurons {
                neuron_ids: vec![],
                include_neurons_readable_by_caller: true,
            })?)
            .call_and_wait()
            .await?;

        let result =
            Decode!(response.as_slice(), ListNeuronsResponse).map_err(|err| anyhow!(err))?;
        Ok(result.full_neurons)
    }

    async fn manage_neuron(
        &self,
        id: u64,
        command: Command,
    ) -> anyhow::Result<ManageNeuronResponse> {
        let response = self
            .agent
            .update(&self.canister_id, "manage_neuron")
            .with_arg(&Encode!(&ManageNeuron {
                id: None,
                command: Some(command),
                neuron_id_or_subaccount: Some(NeuronIdOrSubaccount::NeuronId(NeuronId { id })),
            })?)
            .call_and_wait()
            .await?;

        Decode!(response.as_slice(), ManageNeuronResponse).map_err(|err| anyhow!(err))
    }
}

#[async_trait]
impl Service for Agent<'_> {
    async fn disburse_neurons(&self, now: u64, address: &AccountIdentifier) -> anyhow::Result<()> {
        let neurons = self.list_neurons().await?;
        for n in neurons.iter() {
            let Some(NeuronId { id }) = n.id else {
                continue;
            };
            let Some(DissolveState::WhenDissolvedTimestampSeconds(dissolved_at)) = n.dissolve_state else {
                continue;
            };
            if now < dissolved_at {
                continue;
            }
            self.manage_neuron(
                id,
                Command::Disburse(Disburse {
                    to_account: Some(address.clone()),
                    amount: None, // all
                }),
            )
            .await?;
        }
        Ok(())
    }

    async fn split_new_withdrawal_neurons(
        &self,
        neurons_to_split: Vec<(u64, u64)>,
        hotkeys: Vec<Principal>,
    ) -> anyhow::Result<()> {
        for (id, amount_e8s) in neurons_to_split.iter() {
            let ManageNeuronResponse{
                command: Some(Command_1::Split(SpawnResponse {
                    created_neuron_id: Some(NeuronId {
                        id: new_id,
                    }),
                }))
            } = self.manage_neuron(
                id.clone(),
                Command::Split(Split { amount_e8s: amount_e8s.clone() }),
            )
            .await? else {
                bail!("Unexpected response when splitting neuron {}", id)
            };

            // Add the hotkeys
            // TODO: Check if we need to do this or if they get carried over when splitting.
            for hotkey in hotkeys.iter() {
                self.manage_neuron(
                    new_id,
                    Command::Configure(Configure {
                        operation: Some(Operation::AddHotKey(AddHotKey {
                            new_hot_key: Some(hotkey.clone()),
                        })),
                    }),
                )
                .await?;
            }

            // Start the new neuron dissolving
            self.manage_neuron(
                new_id,
                Command::Configure(Configure {
                    operation: Some(Operation::StartDissolving {}),
                }),
            )
            .await?;
        }
        Ok(())
    }
}

// Generated from canlista 2023-04-01

#[derive(CandidType, Deserialize)]
struct NeuronId {
    id: u64,
}

#[derive(CandidType, Deserialize)]
struct Followees {
    followees: Vec<NeuronId>,
}

#[derive(CandidType, Deserialize)]
struct NodeProvider {
    id: Option<candid::Principal>,
    reward_account: Option<AccountIdentifier>,
}

#[derive(CandidType, Deserialize)]
struct RewardToNeuron {
    dissolve_delay_seconds: u64,
}

#[derive(CandidType, Deserialize)]
struct RewardToAccount {
    to_account: Option<AccountIdentifier>,
}

#[derive(CandidType, Deserialize)]
enum RewardMode {
    RewardToNeuron(RewardToNeuron),
    RewardToAccount(RewardToAccount),
}

#[derive(CandidType, Deserialize)]
struct RewardNodeProvider {
    node_provider: Option<NodeProvider>,
    reward_mode: Option<RewardMode>,
    amount_e8s: u64,
}

#[derive(CandidType, Deserialize)]
struct MostRecentMonthlyNodeProviderRewards {
    timestamp: u64,
    rewards: Vec<RewardNodeProvider>,
}

#[derive(CandidType, Deserialize)]
struct GovernanceCachedMetrics {
    not_dissolving_neurons_e8s_buckets: Vec<(u64, f64)>,
    garbage_collectable_neurons_count: u64,
    neurons_with_invalid_stake_count: u64,
    not_dissolving_neurons_count_buckets: Vec<(u64, u64)>,
    total_supply_icp: u64,
    neurons_with_less_than_6_months_dissolve_delay_count: u64,
    dissolved_neurons_count: u64,
    community_fund_total_maturity_e8s_equivalent: u64,
    total_staked_e8s: u64,
    not_dissolving_neurons_count: u64,
    total_locked_e8s: u64,
    dissolved_neurons_e8s: u64,
    neurons_with_less_than_6_months_dissolve_delay_e8s: u64,
    dissolving_neurons_count_buckets: Vec<(u64, u64)>,
    dissolving_neurons_count: u64,
    dissolving_neurons_e8s_buckets: Vec<(u64, f64)>,
    community_fund_total_staked_e8s: u64,
    timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
struct NetworkEconomics {
    neuron_minimum_stake_e8s: u64,
    max_proposals_to_keep_per_topic: u32,
    neuron_management_fee_per_proposal_e8s: u64,
    reject_cost_e8s: u64,
    transaction_fee_e8s: u64,
    neuron_spawn_dissolve_delay_seconds: u64,
    minimum_icp_xdr_rate: u64,
    maximum_node_provider_rewards_e8s: u64,
}

#[derive(CandidType, Deserialize)]
struct RewardEvent {
    day_after_genesis: u64,
    actual_timestamp_seconds: u64,
    total_available_e8s_equivalent: u64,
    distributed_e8s_equivalent: u64,
    settled_proposals: Vec<NeuronId>,
}

#[derive(CandidType, Deserialize)]
struct NeuronStakeTransfer {
    to_subaccount: Vec<u8>,
    neuron_stake_e8s: u64,
    from: Option<candid::Principal>,
    memo: u64,
    from_subaccount: Vec<u8>,
    transfer_timestamp: u64,
    block_height: u64,
}

#[derive(CandidType, Deserialize)]
struct GovernanceError {
    error_message: String,
    error_type: i32,
}

#[derive(CandidType, Deserialize)]
struct CfNeuron {
    nns_neuron_id: u64,
    amount_icp_e8s: u64,
}

#[derive(CandidType, Deserialize)]
struct CfParticipant {
    hotkey_principal: String,
    cf_neurons: Vec<CfNeuron>,
}

#[derive(CandidType, Deserialize)]
struct Ballot {
    vote: i32,
    voting_power: u64,
}

#[derive(CandidType, Deserialize)]
struct CanisterStatusResultV2 {
    status: Option<i32>,
    freezing_threshold: Option<u64>,
    controllers: Vec<candid::Principal>,
    memory_size: Option<u64>,
    cycles: Option<u64>,
    idle_cycles_burned_per_day: Option<u64>,
    module_hash: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
struct CanisterSummary {
    status: Option<CanisterStatusResultV2>,
    canister_id: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize)]
struct SwapBackgroundInformation {
    ledger_index_canister_summary: Option<CanisterSummary>,
    fallback_controller_principal_ids: Vec<candid::Principal>,
    ledger_archive_canister_summaries: Vec<CanisterSummary>,
    ledger_canister_summary: Option<CanisterSummary>,
    swap_canister_summary: Option<CanisterSummary>,
    governance_canister_summary: Option<CanisterSummary>,
    root_canister_summary: Option<CanisterSummary>,
    dapp_canister_summaries: Vec<CanisterSummary>,
}

#[derive(CandidType, Deserialize)]
struct DerivedProposalInformation {
    swap_background_information: Option<SwapBackgroundInformation>,
}

#[derive(CandidType, Deserialize)]
struct Tally {
    no: u64,
    yes: u64,
    total: u64,
    timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
struct KnownNeuronData {
    name: String,
    description: Option<String>,
}

#[derive(CandidType, Deserialize)]
struct KnownNeuron {
    id: Option<NeuronId>,
    known_neuron_data: Option<KnownNeuronData>,
}

#[derive(CandidType, Deserialize)]
struct Spawn {
    percentage_to_spawn: Option<u32>,
    new_controller: Option<candid::Principal>,
    nonce: Option<u64>,
}

#[derive(CandidType, Deserialize)]
struct Split {
    amount_e8s: u64,
}

#[derive(CandidType, Deserialize)]
struct Follow {
    topic: i32,
    followees: Vec<NeuronId>,
}

#[derive(CandidType, Deserialize)]
struct ClaimOrRefreshNeuronFromAccount {
    controller: Option<candid::Principal>,
    memo: u64,
}

#[derive(CandidType, Deserialize)]
enum By {
    NeuronIdOrSubaccount {},
    MemoAndController(ClaimOrRefreshNeuronFromAccount),
    Memo(u64),
}

#[derive(CandidType, Deserialize)]
struct ClaimOrRefresh {
    by: Option<By>,
}

#[derive(CandidType, Deserialize)]
struct RemoveHotKey {
    hot_key_to_remove: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize)]
struct AddHotKey {
    new_hot_key: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize)]
struct ChangeAutoStakeMaturity {
    requested_setting_for_auto_stake_maturity: bool,
}

#[derive(CandidType, Deserialize)]
struct IncreaseDissolveDelay {
    additional_dissolve_delay_seconds: u32,
}

#[derive(CandidType, Deserialize)]
struct SetDissolveTimestamp {
    dissolve_timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
enum Operation {
    RemoveHotKey(RemoveHotKey),
    AddHotKey(AddHotKey),
    ChangeAutoStakeMaturity(ChangeAutoStakeMaturity),
    StopDissolving {},
    StartDissolving {},
    IncreaseDissolveDelay(IncreaseDissolveDelay),
    JoinCommunityFund {},
    LeaveCommunityFund {},
    SetDissolveTimestamp(SetDissolveTimestamp),
}

#[derive(CandidType, Deserialize)]
struct Configure {
    operation: Option<Operation>,
}

#[derive(CandidType, Deserialize)]
struct RegisterVote {
    vote: i32,
    proposal: Option<NeuronId>,
}

#[derive(CandidType, Deserialize)]
struct Merge {
    source_neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize)]
struct DisburseToNeuron {
    dissolve_delay_seconds: u64,
    kyc_verified: bool,
    amount_e8s: u64,
    new_controller: Option<candid::Principal>,
    nonce: u64,
}

#[derive(CandidType, Deserialize)]
struct StakeMaturity {
    percentage_to_stake: Option<u32>,
}

#[derive(CandidType, Deserialize)]
struct MergeMaturity {
    percentage_to_merge: u32,
}

#[derive(CandidType, Deserialize)]
struct Amount {
    e8s: u64,
}

#[derive(CandidType, Deserialize)]
struct Disburse {
    to_account: Option<AccountIdentifier>,
    amount: Option<Amount>,
}

#[derive(CandidType, Deserialize)]
enum Command {
    Spawn(Spawn),
    Split(Split),
    Follow(Follow),
    ClaimOrRefresh(ClaimOrRefresh),
    Configure(Configure),
    RegisterVote(RegisterVote),
    Merge(Merge),
    DisburseToNeuron(DisburseToNeuron),
    MakeProposal(Box<Proposal>),
    StakeMaturity(StakeMaturity),
    MergeMaturity(MergeMaturity),
    Disburse(Disburse),
}

#[derive(CandidType, Deserialize)]
enum NeuronIdOrSubaccount {
    Subaccount(Vec<u8>),
    NeuronId(NeuronId),
}

#[derive(CandidType, Deserialize)]
struct ManageNeuron {
    id: Option<NeuronId>,
    command: Option<Command>,
    neuron_id_or_subaccount: Option<NeuronIdOrSubaccount>,
}

#[derive(CandidType, Deserialize)]
struct ExecuteNnsFunction {
    nns_function: i32,
    payload: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
struct NeuronBasketConstructionParameters {
    dissolve_delay_interval_seconds: u64,
    count: u64,
}

#[derive(CandidType, Deserialize)]
struct Params {
    min_participant_icp_e8s: u64,
    neuron_basket_construction_parameters: Option<NeuronBasketConstructionParameters>,
    max_icp_e8s: u64,
    swap_due_timestamp_seconds: u64,
    min_participants: u32,
    sns_token_e8s: u64,
    sale_delay_seconds: Option<u64>,
    max_participant_icp_e8s: u64,
    min_icp_e8s: u64,
}

#[derive(CandidType, Deserialize)]
struct OpenSnsTokenSwap {
    community_fund_investment_e8s: Option<u64>,
    target_swap_canister_id: Option<candid::Principal>,
    params: Option<Params>,
}

#[derive(CandidType, Deserialize)]
struct TimeWindow {
    start_timestamp_seconds: u64,
    end_timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
struct SetOpenTimeWindowRequest {
    open_time_window: Option<TimeWindow>,
}

#[derive(CandidType, Deserialize)]
struct SetSnsTokenSwapOpenTimeWindow {
    request: Option<SetOpenTimeWindowRequest>,
    swap_canister_id: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize)]
struct SetDefaultFollowees {
    default_followees: Vec<(i32, Followees)>,
}

#[derive(CandidType, Deserialize)]
struct RewardNodeProviders {
    use_registry_derived_rewards: Option<bool>,
    rewards: Vec<RewardNodeProvider>,
}

#[derive(CandidType, Deserialize)]
struct ApproveGenesisKyc {
    principals: Vec<candid::Principal>,
}

#[derive(CandidType, Deserialize)]
enum Change {
    ToRemove(NodeProvider),
    ToAdd(NodeProvider),
}

#[derive(CandidType, Deserialize)]
struct AddOrRemoveNodeProvider {
    change: Option<Change>,
}

#[derive(CandidType, Deserialize)]
struct Motion {
    motion_text: String,
}

#[derive(CandidType, Deserialize)]
enum Action {
    RegisterKnownNeuron(KnownNeuron),
    ManageNeuron(ManageNeuron),
    ExecuteNnsFunction(ExecuteNnsFunction),
    RewardNodeProvider(RewardNodeProvider),
    OpenSnsTokenSwap(OpenSnsTokenSwap),
    SetSnsTokenSwapOpenTimeWindow(SetSnsTokenSwapOpenTimeWindow),
    SetDefaultFollowees(SetDefaultFollowees),
    RewardNodeProviders(RewardNodeProviders),
    ManageNetworkEconomics(NetworkEconomics),
    ApproveGenesisKyc(ApproveGenesisKyc),
    AddOrRemoveNodeProvider(AddOrRemoveNodeProvider),
    Motion(Motion),
}

#[derive(CandidType, Deserialize)]
struct Proposal {
    url: String,
    title: Option<String>,
    action: Option<Action>,
    summary: String,
}

#[derive(CandidType, Deserialize)]
struct WaitForQuietState {
    current_deadline_timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
struct ProposalData {
    id: Option<NeuronId>,
    failure_reason: Option<GovernanceError>,
    cf_participants: Vec<CfParticipant>,
    ballots: Vec<(u64, Ballot)>,
    proposal_timestamp_seconds: u64,
    reward_event_round: u64,
    failed_timestamp_seconds: u64,
    reject_cost_e8s: u64,
    derived_proposal_information: Option<DerivedProposalInformation>,
    latest_tally: Option<Tally>,
    sns_token_swap_lifecycle: Option<i32>,
    decided_timestamp_seconds: u64,
    proposal: Option<Box<Proposal>>,
    proposer: Option<NeuronId>,
    wait_for_quiet_state: Option<WaitForQuietState>,
    executed_timestamp_seconds: u64,
    original_total_community_fund_maturity_e8s_equivalent: Option<u64>,
}

#[derive(CandidType, Deserialize)]
enum Command_2 {
    Spawn(NeuronId),
    Split(Split),
    Configure(Configure),
    Merge(Merge),
    DisburseToNeuron(DisburseToNeuron),
    SyncCommand {},
    ClaimOrRefreshNeuron(ClaimOrRefresh),
    MergeMaturity(MergeMaturity),
    Disburse(Disburse),
}

#[derive(CandidType, Deserialize)]
struct NeuronInFlightCommand {
    command: Option<Command_2>,
    timestamp: u64,
}

#[derive(CandidType, Deserialize)]
struct BallotInfo {
    vote: i32,
    proposal_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize)]
enum DissolveState {
    DissolveDelaySeconds(u64),
    WhenDissolvedTimestampSeconds(u64),
}

#[derive(CandidType, Deserialize)]
struct Neuron {
    id: Option<NeuronId>,
    staked_maturity_e8s_equivalent: Option<u64>,
    controller: Option<candid::Principal>,
    recent_ballots: Vec<BallotInfo>,
    kyc_verified: bool,
    not_for_profit: bool,
    maturity_e8s_equivalent: u64,
    cached_neuron_stake_e8s: u64,
    created_timestamp_seconds: u64,
    auto_stake_maturity: Option<bool>,
    aging_since_timestamp_seconds: u64,
    hot_keys: Vec<candid::Principal>,
    account: Vec<u8>,
    joined_community_fund_timestamp_seconds: Option<u64>,
    dissolve_state: Option<DissolveState>,
    followees: Vec<(i32, Followees)>,
    neuron_fees_e8s: u64,
    transfer: Option<NeuronStakeTransfer>,
    known_neuron_data: Option<KnownNeuronData>,
    spawn_at_timestamp_seconds: Option<u64>,
}

#[derive(CandidType, Deserialize)]
struct Governance {
    default_followees: Vec<(i32, Followees)>,
    most_recent_monthly_node_provider_rewards: Option<MostRecentMonthlyNodeProviderRewards>,
    maturity_modulation_last_updated_at_timestamp_seconds: Option<u64>,
    wait_for_quiet_threshold_seconds: u64,
    metrics: Option<GovernanceCachedMetrics>,
    node_providers: Vec<NodeProvider>,
    cached_daily_maturity_modulation_basis_points: Option<i32>,
    economics: Option<NetworkEconomics>,
    spawning_neurons: Option<bool>,
    latest_reward_event: Option<RewardEvent>,
    to_claim_transfers: Vec<NeuronStakeTransfer>,
    short_voting_period_seconds: u64,
    proposals: Vec<(u64, ProposalData)>,
    in_flight_commands: Vec<(u64, NeuronInFlightCommand)>,
    neurons: Vec<(u64, Neuron)>,
    genesis_timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
enum Result {
    Ok,
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize)]
enum Result_1 {
    Error(GovernanceError),
    NeuronId(NeuronId),
}

#[derive(CandidType, Deserialize)]
struct ClaimOrRefreshNeuronFromAccountResponse {
    result: Option<Result_1>,
}

#[derive(CandidType, Deserialize)]
enum Result_2 {
    Ok(Neuron),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize)]
enum Result_3 {
    Ok(GovernanceCachedMetrics),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize)]
enum Result_4 {
    Ok(RewardNodeProviders),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize)]
struct NeuronInfo {
    dissolve_delay_seconds: u64,
    recent_ballots: Vec<BallotInfo>,
    created_timestamp_seconds: u64,
    state: i32,
    stake_e8s: u64,
    joined_community_fund_timestamp_seconds: Option<u64>,
    retrieved_at_timestamp_seconds: u64,
    known_neuron_data: Option<KnownNeuronData>,
    voting_power: u64,
    age_seconds: u64,
}

#[derive(CandidType, Deserialize)]
enum Result_5 {
    Ok(NeuronInfo),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize)]
enum Result_6 {
    Ok(NodeProvider),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize)]
struct ProposalInfo {
    id: Option<NeuronId>,
    status: i32,
    topic: i32,
    failure_reason: Option<GovernanceError>,
    ballots: Vec<(u64, Ballot)>,
    proposal_timestamp_seconds: u64,
    reward_event_round: u64,
    deadline_timestamp_seconds: Option<u64>,
    failed_timestamp_seconds: u64,
    reject_cost_e8s: u64,
    derived_proposal_information: Option<DerivedProposalInformation>,
    latest_tally: Option<Tally>,
    reward_status: i32,
    decided_timestamp_seconds: u64,
    proposal: Option<Box<Proposal>>,
    proposer: Option<NeuronId>,
    executed_timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
struct ListKnownNeuronsResponse {
    known_neurons: Vec<KnownNeuron>,
}

#[derive(CandidType, Deserialize)]
struct ListNeurons {
    neuron_ids: Vec<u64>,
    include_neurons_readable_by_caller: bool,
}

#[derive(CandidType, Deserialize)]
struct ListNeuronsResponse {
    neuron_infos: Vec<(u64, NeuronInfo)>,
    full_neurons: Vec<Neuron>,
}

#[derive(CandidType, Deserialize)]
struct ListNodeProvidersResponse {
    node_providers: Vec<NodeProvider>,
}

#[derive(CandidType, Deserialize)]
struct ListProposalInfo {
    include_reward_status: Vec<i32>,
    before_proposal: Option<NeuronId>,
    limit: u32,
    exclude_topic: Vec<i32>,
    include_status: Vec<i32>,
}

#[derive(CandidType, Deserialize)]
struct ListProposalInfoResponse {
    proposal_info: Vec<ProposalInfo>,
}

#[derive(CandidType, Deserialize)]
struct SpawnResponse {
    created_neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize)]
struct ClaimOrRefreshResponse {
    refreshed_neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize)]
struct MakeProposalResponse {
    proposal_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize)]
struct StakeMaturityResponse {
    maturity_e8s: u64,
    staked_maturity_e8s: u64,
}

#[derive(CandidType, Deserialize)]
struct MergeMaturityResponse {
    merged_maturity_e8s: u64,
    new_stake_e8s: u64,
}

#[derive(CandidType, Deserialize)]
struct DisburseResponse {
    transfer_block_height: u64,
}

#[derive(CandidType, Deserialize)]
enum Command_1 {
    Error(GovernanceError),
    Spawn(SpawnResponse),
    Split(SpawnResponse),
    Follow {},
    ClaimOrRefresh(ClaimOrRefreshResponse),
    Configure {},
    RegisterVote {},
    Merge {},
    DisburseToNeuron(SpawnResponse),
    MakeProposal(MakeProposalResponse),
    StakeMaturity(StakeMaturityResponse),
    MergeMaturity(MergeMaturityResponse),
    Disburse(DisburseResponse),
}

#[derive(CandidType, Deserialize)]
struct ManageNeuronResponse {
    command: Option<Command_1>,
}

#[derive(CandidType, Deserialize)]
struct Committed {
    sns_governance_canister_id: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize)]
enum Result_7 {
    Committed(Committed),
    Aborted {},
}

#[derive(CandidType, Deserialize)]
struct SettleCommunityFundParticipation {
    result: Option<Result_7>,
    open_sns_token_swap_proposal_id: Option<u64>,
}

#[derive(CandidType, Deserialize)]
struct UpdateNodeProvider {
    reward_account: Option<AccountIdentifier>,
}
