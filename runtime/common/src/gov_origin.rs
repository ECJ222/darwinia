// --- paritytech ---
use frame_system::{EnsureOneOf, EnsureRoot};
use pallet_collective::{EnsureProportionAtLeast, EnsureProportionMoreThan};
use sp_core::u32_trait::{_1, _2, _3, _5};
// --- darwinia-network ---
use darwinia_primitives::AccountId;

pub type Root = EnsureRoot<AccountId>;

pub type RootOrAtLeastHalf<Collective> =
	EnsureOneOf<AccountId, Root, EnsureProportionAtLeast<_1, _2, AccountId, Collective>>;

pub type RootOrMoreThanHalf<Collective> =
	EnsureOneOf<AccountId, Root, EnsureProportionMoreThan<_1, _2, AccountId, Collective>>;

pub type RootOrAtLeastTwoThird<Collective> =
	EnsureOneOf<AccountId, Root, EnsureProportionAtLeast<_2, _3, AccountId, Collective>>;

pub type RootOrAtLeastThreeFifth<Collective> =
	EnsureOneOf<AccountId, Root, EnsureProportionAtLeast<_3, _5, AccountId, Collective>>;

pub type RootOrAll<Collective> =
	EnsureOneOf<AccountId, Root, EnsureProportionAtLeast<_1, _1, AccountId, Collective>>;
