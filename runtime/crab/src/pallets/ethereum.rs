// --- darwinia-network ---
use crate::*;
use darwinia_ethereum::{Config, IntermediateStateRoot};
// --- paritytech ---
use frame_support::PalletId;

frame_support::parameter_types! {
	pub const DvmPalletId: PalletId = PalletId(*b"dar/dvmp");
}

impl Config for Runtime {
	type Event = Event;
	type PalletId = DvmPalletId;
	type StateRoot = IntermediateStateRoot;
}
