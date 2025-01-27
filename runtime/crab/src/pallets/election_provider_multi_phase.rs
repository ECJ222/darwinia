// --- paritytech ---
use frame_election_provider_support::{onchain, SequentialPhragmen};
use pallet_election_provider_multi_phase::{
	BenchmarkingConfig, Config, NoFallback, SolutionAccuracyOf,
};
use sp_runtime::{transaction_validity::TransactionPriority, Perbill};
// --- darwinia-network ---
use crate::*;

sp_npos_elections::generate_solution_type!(
	#[compact]
	pub struct NposCompactSolution24::<
		VoterIndex = u32,
		TargetIndex = u16,
		Accuracy = sp_runtime::PerU16,
	>(24)
);

frame_support::parameter_types! {
	// no signed phase for now, just unsigned.
	pub const SignedPhase: u32 = 0;
	pub const UnsignedPhase: u32 = CRAB_BLOCKS_PER_SESSION / 4;

	// signed config
	pub const SignedMaxSubmissions: u32 = 10;
	// Each good submission will get 10 CRAB as reward.
	pub const SignedRewardBase: Balance = 10 * COIN;
	pub const SignedDepositBase: Balance = MILLI;
	// TODO: update this
	// pub const SignedDepositBase: Balance = crab_deposit(2, 0);
	pub const SignedDepositByte: Balance = MICRO;
	// TODO: update this
	// pub const SignedDepositByte: Balance = deposit(0, 10) / 1024;

	pub SolutionImprovementThreshold: Perbill = Perbill::from_rational(5u32, 10_000);

	// miner configs
	pub NposSolutionPriority: TransactionPriority = Perbill::from_percent(90) * TransactionPriority::max_value();
	pub OffchainRepeat: BlockNumber = 5;

	/// Whilst `UseNominatorsAndUpdateBagsList` or `UseNominatorsMap` is in use, this can still be a
	/// very large value. Once the `BagsList` is in full motion, staking might open its door to many
	/// more nominators, and this value should instead be what is a "safe" number (e.g. 22500).
	pub const VoterSnapshotPerBlock: u32 = 22_500;
}

impl Config for Runtime {
	type BenchmarkingConfig = BenchmarkConfig;
	type Currency = Ring;
	// nothing to do upon rewards
	type DataProvider = Staking;
	type EstimateCallFee = TransactionPayment;
	type Event = Event;
	type Fallback = NoFallback<Self>;
	type ForceOrigin = RootOrAtLeastHalf<CouncilCollective>;
	type MinerMaxLength = OffchainSolutionLengthLimit;
	type MinerMaxWeight = OffchainSolutionWeightLimit;
	type MinerTxPriority = NposSolutionPriority;
	// For now use the one from staking.
	type OffchainRepeat = OffchainRepeat;
	// burn slashes
	type RewardHandler = ();
	type SignedDepositBase = SignedDepositBase;
	type SignedDepositByte = SignedDepositByte;
	type SignedDepositWeight = ();
	type SignedMaxSubmissions = SignedMaxSubmissions;
	type SignedMaxWeight = Self::MinerMaxWeight;
	type SignedPhase = SignedPhase;
	type SignedRewardBase = SignedRewardBase;
	type SlashHandler = ();
	type Solution = NposCompactSolution24;
	type SolutionImprovementThreshold = SolutionImprovementThreshold;
	type Solver = SequentialPhragmen<AccountId, SolutionAccuracyOf<Self>, OffchainRandomBalancing>;
	type UnsignedPhase = UnsignedPhase;
	type VoterSnapshotPerBlock = VoterSnapshotPerBlock;
	type WeightInfo = ();
}

impl onchain::Config for Runtime {
	type Accuracy = Perbill;
	type DataProvider = Staking;
}

/// The numbers configured here could always be more than the the maximum limits of staking pallet
/// to ensure election snapshot will not run out of memory. For now, we set them to smaller values
/// since the staking is bounded and the weight pipeline takes hours for this single pallet.
pub struct BenchmarkConfig;
impl BenchmarkingConfig for BenchmarkConfig {
	const ACTIVE_VOTERS: [u32; 2] = [500, 800];
	const DESIRED_TARGETS: [u32; 2] = [200, 400];
	const MAXIMUM_TARGETS: u32 = 300;
	const MINER_MAXIMUM_VOTERS: u32 = 1000;
	const SNAPSHOT_MAXIMUM_VOTERS: u32 = 1000;
	const TARGETS: [u32; 2] = [500, 1000];
	const VOTERS: [u32; 2] = [1000, 2000];
}
