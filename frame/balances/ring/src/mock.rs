//! Test utilities

use std::cell::RefCell;

use frame_support::{
	traits::Get,
	weights::{DispatchInfo, Weight},
	{impl_outer_origin, parameter_types},
};
use sp_core::H256;
use sp_io;
use sp_runtime::{
	testing::Header,
	traits::{ConvertInto, IdentityLookup},
	Perbill,
};

use crate::*;

pub type System = system::Module<Test>;

pub type Ring = Module<Test>;

pub const CALL: &<Test as system::Trait>::Call = &();

impl_outer_origin! {
	pub enum Origin for Test {}
}

thread_local! {
	pub(crate) static EXISTENTIAL_DEPOSIT: RefCell<u64> = RefCell::new(0);
	static TRANSFER_FEE: RefCell<u64> = RefCell::new(0);
	static CREATION_FEE: RefCell<u64> = RefCell::new(0);
}

pub struct ExistentialDeposit;
impl Get<u64> for ExistentialDeposit {
	fn get() -> u64 {
		EXISTENTIAL_DEPOSIT.with(|v| *v.borrow())
	}
}

pub struct TransferFee;
impl Get<u64> for TransferFee {
	fn get() -> u64 {
		TRANSFER_FEE.with(|v| *v.borrow())
	}
}

pub struct CreationFee;
impl Get<u64> for CreationFee {
	fn get() -> u64 {
		CREATION_FEE.with(|v| *v.borrow())
	}
}

// Workaround for https://github.com/rust-lang/rust/issues/26925 . Remove when sorted.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}
impl system::Trait for Test {
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
}

parameter_types! {
	pub const TransactionBaseFee: u64 = 0;
	pub const TransactionByteFee: u64 = 1;
}
impl pallet_transaction_payment::Trait for Test {
	type Currency = Module<Test>;
	type OnTransactionPayment = ();
	type TransactionBaseFee = TransactionBaseFee;
	type TransactionByteFee = TransactionByteFee;
	type WeightToFee = ConvertInto;
	type FeeMultiplierUpdate = ();
}
impl Trait for Test {
	type Balance = u64;
	type OnFreeBalanceZero = ();
	type OnNewAccount = ();
	type TransferPayment = ();
	type DustRemoval = ();
	type Event = ();
	type ExistentialDeposit = ExistentialDeposit;
	type TransferFee = TransferFee;
	type CreationFee = CreationFee;
}

pub struct ExtBuilder {
	existential_deposit: u64,
	transfer_fee: u64,
	creation_fee: u64,
	monied: bool,
	vesting: bool,
}
impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			existential_deposit: 0,
			transfer_fee: 0,
			creation_fee: 0,
			monied: false,
			vesting: false,
		}
	}
}
impl ExtBuilder {
	pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
		self.existential_deposit = existential_deposit;
		self
	}
	#[allow(dead_code)]
	pub fn transfer_fee(mut self, transfer_fee: u64) -> Self {
		self.transfer_fee = transfer_fee;
		self
	}
	pub fn creation_fee(mut self, creation_fee: u64) -> Self {
		self.creation_fee = creation_fee;
		self
	}
	pub fn monied(mut self, monied: bool) -> Self {
		self.monied = monied;
		if self.existential_deposit == 0 {
			self.existential_deposit = 1;
		}
		self
	}
	pub fn vesting(mut self, vesting: bool) -> Self {
		self.vesting = vesting;
		self
	}
	pub fn set_associated_consts(&self) {
		EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
		TRANSFER_FEE.with(|v| *v.borrow_mut() = self.transfer_fee);
		CREATION_FEE.with(|v| *v.borrow_mut() = self.creation_fee);
	}
	pub fn build(self) -> sp_io::TestExternalities {
		self.set_associated_consts();
		let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
		GenesisConfig::<Test> {
			balances: if self.monied {
				vec![
					(1, 10 * self.existential_deposit),
					(2, 20 * self.existential_deposit),
					(3, 30 * self.existential_deposit),
					(4, 40 * self.existential_deposit),
					(12, 10 * self.existential_deposit),
				]
			} else {
				vec![]
			},
			vesting: if self.vesting && self.monied {
				vec![
					(1, 0, 10, 5 * self.existential_deposit),
					(2, 10, 20, 0),
					(12, 10, 20, 5 * self.existential_deposit),
				]
			} else {
				vec![]
			},
		}
		.assimilate_storage(&mut t)
		.unwrap();
		t.into()
	}
}

/// create a transaction info struct from weight. Handy to avoid building the whole struct.
pub fn info_from_weight(w: Weight) -> DispatchInfo {
	DispatchInfo {
		weight: w,
		..Default::default()
	}
}