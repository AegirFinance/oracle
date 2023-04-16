// Generated from canlista 2023-04-01

use candid::{CandidType, Decode, Deserialize, Encode};

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct NeuronId {
    pub id: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Followees {
    pub followees: Vec<NeuronId>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct AccountIdentifier { pub hash: Vec<u8> }

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct NodeProvider {
    pub id: Option<candid::Principal>,
    pub reward_account: Option<AccountIdentifier>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct RewardToNeuron {
    pub dissolve_delay_seconds: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct RewardToAccount {
    pub to_account: Option<AccountIdentifier>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub enum RewardMode {
    RewardToNeuron(RewardToNeuron),
    RewardToAccount(RewardToAccount),
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct RewardNodeProvider {
    node_provider: Option<NodeProvider>,
    reward_mode: Option<RewardMode>,
    amount_e8s: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct MostRecentMonthlyNodeProviderRewards {
    timestamp: u64,
    rewards: Vec<RewardNodeProvider>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct GovernanceCachedMetrics {
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
#[derive(Clone, PartialEq)]
pub struct NetworkEconomics {
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
#[derive(Clone, PartialEq)]
pub struct RewardEvent {
    day_after_genesis: u64,
    actual_timestamp_seconds: u64,
    total_available_e8s_equivalent: u64,
    distributed_e8s_equivalent: u64,
    settled_proposals: Vec<NeuronId>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct NeuronStakeTransfer {
    to_subaccount: Vec<u8>,
    neuron_stake_e8s: u64,
    from: Option<candid::Principal>,
    memo: u64,
    from_subaccount: Vec<u8>,
    transfer_timestamp: u64,
    block_height: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct GovernanceError {
    error_message: String,
    error_type: i32,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct CfNeuron {
    nns_neuron_id: u64,
    amount_icp_e8s: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct CfParticipant {
    pub hotkey_principal: String,
    pub cf_neurons: Vec<CfNeuron>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Ballot {
    vote: i32,
    voting_power: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct CanisterStatusResultV2 {
    status: Option<i32>,
    freezing_threshold: Option<u64>,
    controllers: Vec<candid::Principal>,
    memory_size: Option<u64>,
    cycles: Option<u64>,
    idle_cycles_burned_per_day: Option<u64>,
    module_hash: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct CanisterSummary {
    status: Option<CanisterStatusResultV2>,
    canister_id: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct SwapBackgroundInformation {
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
#[derive(Clone, PartialEq)]
pub struct DerivedProposalInformation {
    swap_background_information: Option<SwapBackgroundInformation>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Tally {
    no: u64,
    yes: u64,
    total: u64,
    timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct KnownNeuronData {
    name: String,
    description: Option<String>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct KnownNeuron {
    id: Option<NeuronId>,
    known_neuron_data: Option<KnownNeuronData>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Spawn {
    percentage_to_spawn: Option<u32>,
    new_controller: Option<candid::Principal>,
    nonce: Option<u64>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Split {
    pub amount_e8s: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Follow {
    topic: i32,
    followees: Vec<NeuronId>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ClaimOrRefreshNeuronFromAccount {
    pub controller: Option<candid::Principal>,
    pub memo: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub enum By {
    NeuronIdOrSubaccount,
    MemoAndController(ClaimOrRefreshNeuronFromAccount),
    Memo(u64),
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ClaimOrRefresh {
    pub by: Option<By>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct RemoveHotKey {
    pub hot_key_to_remove: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct AddHotKey {
    pub new_hot_key: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ChangeAutoStakeMaturity {
    pub requested_setting_for_auto_stake_maturity: bool,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct IncreaseDissolveDelay {
    pub additional_dissolve_delay_seconds: u32,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct SetDissolveTimestamp {
    pub dissolve_timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub enum Operation {
    RemoveHotKey(RemoveHotKey),
    AddHotKey(AddHotKey),
    ChangeAutoStakeMaturity(ChangeAutoStakeMaturity),
    StopDissolving,
    StartDissolving,
    IncreaseDissolveDelay(IncreaseDissolveDelay),
    JoinCommunityFund,
    LeaveCommunityFund,
    SetDissolveTimestamp(SetDissolveTimestamp),
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Configure {
    pub operation: Option<Operation>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct RegisterVote {
    pub vote: i32,
    pub proposal: Option<NeuronId>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Merge {
    pub source_neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct DisburseToNeuron {
    pub dissolve_delay_seconds: u64,
    pub kyc_verified: bool,
    pub amount_e8s: u64,
    pub new_controller: Option<candid::Principal>,
    pub nonce: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct StakeMaturity {
    pub percentage_to_stake: Option<u32>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct MergeMaturity {
    pub percentage_to_merge: u32,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Amount {
    pub e8s: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Disburse {
    pub to_account: Option<AccountIdentifier>,
    pub amount: Option<Amount>,
}

#[derive(candid::CandidType, candid::Deserialize)]
#[allow(clippy::large_enum_variant)]
#[derive(Clone, PartialEq)]
pub enum Command {
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
#[derive(Clone, PartialEq)]
pub enum NeuronIdOrSubaccount {
    Subaccount(Vec<u8>),
    NeuronId(NeuronId),
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ManageNeuron {
    pub id: Option<NeuronId>,
    pub command: Option<Command>,
    pub neuron_id_or_subaccount: Option<NeuronIdOrSubaccount>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ExecuteNnsFunction {
    nns_function: i32,
    payload: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct NeuronBasketConstructionParameters {
    dissolve_delay_interval_seconds: u64,
    count: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Params {
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
#[derive(Clone, PartialEq)]
pub struct OpenSnsTokenSwap {
    community_fund_investment_e8s: Option<u64>,
    target_swap_canister_id: Option<candid::Principal>,
    params: Option<Params>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct TimeWindow {
    start_timestamp_seconds: u64,
    end_timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct SetOpenTimeWindowRequest {
    open_time_window: Option<TimeWindow>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct SetSnsTokenSwapOpenTimeWindow {
    request: Option<SetOpenTimeWindowRequest>,
    swap_canister_id: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct SetDefaultFollowees {
    default_followees: Vec<(i32, Followees)>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct RewardNodeProviders {
    use_registry_derived_rewards: Option<bool>,
    rewards: Vec<RewardNodeProvider>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ApproveGenesisKyc {
    principals: Vec<candid::Principal>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub enum Change {
    ToRemove(NodeProvider),
    ToAdd(NodeProvider),
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct AddOrRemoveNodeProvider {
    change: Option<Change>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Motion {
    motion_text: String,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub enum Action {
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
#[derive(Clone, PartialEq)]
pub struct Proposal {
    url: String,
    title: Option<String>,
    action: Option<Action>,
    summary: String,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct WaitForQuietState {
    current_deadline_timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ProposalData {
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
#[derive(Clone, PartialEq)]
pub enum Command_2 {
    Spawn(NeuronId),
    Split(Split),
    Configure(Configure),
    Merge(Merge),
    DisburseToNeuron(DisburseToNeuron),
    SyncCommand,
    ClaimOrRefreshNeuron(ClaimOrRefresh),
    MergeMaturity(MergeMaturity),
    Disburse(Disburse),
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct NeuronInFlightCommand {
    command: Option<Command_2>,
    timestamp: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct BallotInfo {
    vote: i32,
    proposal_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub enum DissolveState {
    DissolveDelaySeconds(u64),
    WhenDissolvedTimestampSeconds(u64),
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Neuron {
    pub id: Option<NeuronId>,
    pub staked_maturity_e8s_equivalent: Option<u64>,
    pub controller: Option<candid::Principal>,
    pub recent_ballots: Vec<BallotInfo>,
    pub kyc_verified: bool,
    pub not_for_profit: bool,
    pub maturity_e8s_equivalent: u64,
    pub cached_neuron_stake_e8s: u64,
    pub created_timestamp_seconds: u64,
    pub auto_stake_maturity: Option<bool>,
    pub aging_since_timestamp_seconds: u64,
    pub hot_keys: Vec<candid::Principal>,
    pub account: Vec<u8>,
    pub joined_community_fund_timestamp_seconds: Option<u64>,
    pub dissolve_state: Option<DissolveState>,
    pub followees: Vec<(i32, Followees)>,
    pub neuron_fees_e8s: u64,
    pub transfer: Option<NeuronStakeTransfer>,
    pub known_neuron_data: Option<KnownNeuronData>,
    pub spawn_at_timestamp_seconds: Option<u64>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Governance {
    pub default_followees: Vec<(i32, Followees)>,
    pub most_recent_monthly_node_provider_rewards: Option<MostRecentMonthlyNodeProviderRewards>,
    pub maturity_modulation_last_updated_at_timestamp_seconds: Option<u64>,
    pub wait_for_quiet_threshold_seconds: u64,
    pub metrics: Option<GovernanceCachedMetrics>,
    pub node_providers: Vec<NodeProvider>,
    pub cached_daily_maturity_modulation_basis_points: Option<i32>,
    pub economics: Option<NetworkEconomics>,
    pub spawning_neurons: Option<bool>,
    pub latest_reward_event: Option<RewardEvent>,
    pub to_claim_transfers: Vec<NeuronStakeTransfer>,
    pub short_voting_period_seconds: u64,
    pub proposals: Vec<(u64, ProposalData)>,
    pub in_flight_commands: Vec<(u64, NeuronInFlightCommand)>,
    pub neurons: Vec<(u64, Neuron)>,
    pub genesis_timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub enum Result {
    Ok,
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub enum Result_1 {
    Error(GovernanceError),
    NeuronId(NeuronId),
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ClaimOrRefreshNeuronFromAccountResponse {
    pub result: Option<Result_1>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub enum Result_2 {
    Ok(Neuron),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub enum Result_3 {
    Ok(GovernanceCachedMetrics),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub enum Result_4 {
    Ok(RewardNodeProviders),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct NeuronInfo {
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
#[derive(Clone, PartialEq)]
pub enum Result_5 {
    Ok(NeuronInfo),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub enum Result_6 {
    Ok(NodeProvider),
    Err(GovernanceError),
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ProposalInfo {
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
#[derive(Clone, PartialEq)]
pub struct ListKnownNeuronsResponse {
    pub known_neurons: Vec<KnownNeuron>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ListNeurons {
    pub neuron_ids: Vec<u64>,
    pub include_neurons_readable_by_caller: bool,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ListNeuronsResponse {
    pub neuron_infos: Vec<(u64, NeuronInfo)>,
    pub full_neurons: Vec<Neuron>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ListNodeProvidersResponse {
    node_providers: Vec<NodeProvider>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ListProposalInfo {
    include_reward_status: Vec<i32>,
    before_proposal: Option<NeuronId>,
    limit: u32,
    exclude_topic: Vec<i32>,
    include_status: Vec<i32>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ListProposalInfoResponse {
    proposal_info: Vec<ProposalInfo>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct SpawnResponse {
    pub created_neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ClaimOrRefreshResponse {
    pub refreshed_neuron_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct MakeProposalResponse {
    proposal_id: Option<NeuronId>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct StakeMaturityResponse {
    maturity_e8s: u64,
    staked_maturity_e8s: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct MergeMaturityResponse {
    merged_maturity_e8s: u64,
    new_stake_e8s: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct DisburseResponse {
    transfer_block_height: u64,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub enum Command_1 {
    Error(GovernanceError),
    Spawn(SpawnResponse),
    Split(SpawnResponse),
    Follow,
    ClaimOrRefresh(ClaimOrRefreshResponse),
    Configure,
    RegisterVote,
    Merge,
    DisburseToNeuron(SpawnResponse),
    MakeProposal(MakeProposalResponse),
    StakeMaturity(StakeMaturityResponse),
    MergeMaturity(MergeMaturityResponse),
    Disburse(DisburseResponse),
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct ManageNeuronResponse {
    pub command: Option<Command_1>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct Committed {
    sns_governance_canister_id: Option<candid::Principal>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub enum Result_7 {
    Committed(Committed),
    Aborted,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct SettleCommunityFundParticipation {
    result: Option<Result_7>,
    open_sns_token_swap_proposal_id: Option<u64>,
}

#[derive(CandidType, Deserialize)]
#[derive(Clone, PartialEq)]
pub struct UpdateNodeProvider {
    reward_account: Option<AccountIdentifier>,
}
