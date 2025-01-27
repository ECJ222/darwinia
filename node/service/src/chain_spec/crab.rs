// This file is part of Darwinia.
//
// Copyright (C) 2018-2022 Darwinia Network
// SPDX-License-Identifier: GPL-3.0
//
// Darwinia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Darwinia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Darwinia. If not, see <https://www.gnu.org/licenses/>.

// --- std ---
use std::collections::BTreeMap;
// --- paritytech ---
use fp_evm::GenesisAccount;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainType;
use sc_finality_grandpa::AuthorityId as GrandpaId;
use sc_service::Properties;
use sc_telemetry::TelemetryEndpoints;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519};
use sp_runtime::Perbill;
// --- darwinia-network ---
use super::*;
use crab_runtime::*;
use darwinia_primitives::{AccountId, Balance, COIN};

/// The `ChainSpec parametrised for Crab runtime`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

const CRAB_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Session keys for Crab.
pub fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online, authority_discovery }
}

/// Properties for Crab.
pub fn properties() -> Properties {
	let mut properties = Properties::new();

	properties.insert("ss58Format".into(), 42.into());
	properties.insert("tokenDecimals".into(), vec![9, 9].into());
	properties.insert("tokenSymbol".into(), vec!["CRAB", "CKTON"].into());

	properties
}

/// Crab config.
pub fn config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../../res/crab/crab.json")[..])
}

/// Crab genesis config.
pub fn genesis_config() -> ChainSpec {
	fn genesis() -> GenesisConfig {
		const CRAB_ENDOWMENT: Balance = 1_000_000 * COIN;
		const CKTON_ENDOWMENT: Balance = 10_000 * COIN;

		const ROOT: &str = "0x0a66532a23c418cca12183fee5f6afece770a0bb8725f459d7d1b1b598f91c49";
		const MULTI_SIG: &str =
			"0x8db5c746c14cf05e182b10576a9ee765265366c3b7fd53c41d43640c97f4a8b8";
		const GENESIS_VALIDATOR_SR: &str =
			"0xb4f7f03bebc56ebe96bc52ea5ed3159d45a0ce3a8d7f082983c33ef133274747";
		const GENESIS_VALIDATOR_ED: &str =
			"0x6a282c7674945c039a9289b702376ae168e8b67c9ed320054e2a019015f236fd";

		let root: AccountId = array_bytes::hex_into_unchecked(ROOT);
		let multi_sig: AccountId = array_bytes::hex_into_unchecked(MULTI_SIG);
		let genesis_validator: (
			AccountId,
			AccountId,
			BabeId,
			GrandpaId,
			ImOnlineId,
			AuthorityDiscoveryId,
		) = {
			let stash = array_bytes::hex_into_unchecked(GENESIS_VALIDATOR_SR);
			let controller = array_bytes::hex_into_unchecked(GENESIS_VALIDATOR_SR);
			let session = array_bytes::hex2array_unchecked(GENESIS_VALIDATOR_SR);
			let grandpa = array_bytes::hex2array_unchecked(GENESIS_VALIDATOR_ED);

			(
				stash,
				controller,
				session.unchecked_into(),
				grandpa.unchecked_into(),
				session.unchecked_into(),
				session.unchecked_into(),
			)
		};
		let endowed_accounts = [
			// AlexChien
			"0x80a5d9612f5504f3e04a31ca19f1d6108ca77252bd05940031eb446953409c1a",
			// clearloop
			"0x6e6844ba5c73db6c4c6b67ea59c2787dd6bd2f9b8139a69c33e14a722d1e801d",
			// freehere107
			"0xc4429847f3598f40008d0cbab53476a2f19165696aa41002778524b3ecf82938",
			// HackFisher
			"0xb62d88e3f439fe9b5ea799b27bf7c6db5e795de1784f27b1bc051553499e420f",
			// WoeOm
			"0x0331760198d850b159844f3bfa620f6e704167973213154aca27675f7ddd987e",
			// yanganto
			"0xc45f075b5b1aa0145c469f57bd741c02272c1c0c41e9518d5a32426030d98232",
		]
		.iter()
		.map(|s| array_bytes::hex_into_unchecked(s))
		.collect::<Vec<_>>();

		GenesisConfig {
			system: SystemConfig { code: wasm_binary_unwrap().to_vec() },
			babe: BabeConfig { authorities: vec![], epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG) },
			indices: Default::default(),
			balances: BalancesConfig {
				balances: endowed_accounts
					.iter()
					.cloned()
					.map(|k| (k, CRAB_ENDOWMENT))
					.chain(
						vec![
							(root, 25_000_000 * COIN),
							(multi_sig, 700_000_000 * COIN),
							(genesis_validator.0.clone(), CRAB_ENDOWMENT),
						]
						.into_iter(),
					)
					.collect(),
			},
			kton: KtonConfig {
				balances: endowed_accounts.iter().cloned().map(|k| (k, CKTON_ENDOWMENT)).collect(),
			},
			staking: StakingConfig {
				minimum_validator_count: 1,
				validator_count: 15,
				stakers: vec![(
					genesis_validator.0.clone(),
					genesis_validator.1.clone(),
					COIN,
					StakerStatus::Validator,
				)],
				force_era: Forcing::ForceNew,
				slash_reward_fraction: Perbill::from_percent(10),
				payout_fraction: Perbill::from_percent(50),
				..Default::default()
			},
			session: SessionConfig {
				keys: vec![(
					genesis_validator.0.clone(),
					genesis_validator.0,
					session_keys(
						genesis_validator.2,
						genesis_validator.3,
						genesis_validator.4,
						genesis_validator.5,
					),
				)],
			},
			grandpa: Default::default(),
			im_online: Default::default(),
			authority_discovery: Default::default(),
			democracy: Default::default(),
			council: Default::default(),
			technical_committee: Default::default(),
			phragmen_election: Default::default(),
			technical_membership: Default::default(),
			treasury: Default::default(),
			kton_treasury: Default::default(),
			vesting: Default::default(),
			evm: EVMConfig { accounts: BTreeMap::new() },
			ethereum: Default::default(),
			base_fee: Default::default(),
			from_darwinia_issuing: Default::default(),
			to_crab_parachain_backing: Default::default(),
		}
	}

	let boot_nodes = vec![
		"/dns/g1.crab-p2p.darwinia.network/tcp/30333/p2p/12D3KooWFqHZkyv6iabxxqiHdNjWb4c7EfmBqMNCyqLCCVZm8yyQ".parse().unwrap(),
		"/dns/g2.crab-p2p.darwinia.network/tcp/30333/p2p/12D3KooWPiza2NAD6CjdBGtfUd3pfDnZXysYKzumejGHafW3Y8xP".parse().unwrap(),
		"/dns/g1.crab-p2p.darwinia.network/tcp/30334/ws/p2p/12D3KooWFqHZkyv6iabxxqiHdNjWb4c7EfmBqMNCyqLCCVZm8yyQ".parse().unwrap(),
		"/dns/g2.crab-p2p.darwinia.network/tcp/30334/ws/p2p/12D3KooWPiza2NAD6CjdBGtfUd3pfDnZXysYKzumejGHafW3Y8xP".parse().unwrap(),
	];

	ChainSpec::from_genesis(
		"Darwinia Crab",
		"crab",
		ChainType::Live,
		genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(CRAB_TELEMETRY_URL.to_string(), 0)])
				.expect("Crab telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		Some(properties()),
		Default::default(),
	)
}

/// Crab development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	fn genesis() -> GenesisConfig {
		let initial_authorities = vec![get_authority_keys_from_seed("Alice")];
		let endowed_accounts = vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
		];
		let evm_accounts = BTreeMap::from_iter([(
			array_bytes::hex_into_unchecked("0x6be02d1d3665660d22ff9624b7be0551ee1ac91b"),
			GenesisAccount {
				balance: (123_456_789_000_000_000_000_090 as Balance).into(),
				code: Default::default(),
				nonce: Default::default(),
				storage: Default::default(),
			},
		)]);

		GenesisConfig {
			system: SystemConfig { code: wasm_binary_unwrap().to_vec() },
			babe: BabeConfig { authorities: vec![], epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG) },
			indices: Default::default(),
			balances: BalancesConfig {
				balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 56)).collect(),
			},
			kton: KtonConfig {
				balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 56)).collect(),
			},
			staking: StakingConfig {
				minimum_validator_count: 1,
				validator_count: 15,
				stakers: initial_authorities
					.iter()
					.cloned()
					.map(|x| (x.0, x.1, COIN, StakerStatus::Validator))
					.collect(),
				invulnerables: initial_authorities.iter().cloned().map(|x| x.0).collect(),
				force_era: Forcing::ForceNew,
				slash_reward_fraction: Perbill::from_percent(10),
				payout_fraction: Perbill::from_percent(50),
				..Default::default()
			},
			session: SessionConfig {
				keys: initial_authorities
					.iter()
					.cloned()
					.map(|x| (x.0.clone(), x.0, session_keys(x.2, x.3, x.4, x.5)))
					.collect(),
			},
			grandpa: Default::default(),
			im_online: Default::default(),
			authority_discovery: Default::default(),
			democracy: Default::default(),
			council: Default::default(),
			technical_committee: Default::default(),
			phragmen_election: Default::default(),
			technical_membership: Default::default(),
			treasury: Default::default(),
			kton_treasury: Default::default(),
			vesting: Default::default(),
			evm: EVMConfig { accounts: evm_accounts },
			ethereum: Default::default(),
			base_fee: Default::default(),
			from_darwinia_issuing: Default::default(),
			to_crab_parachain_backing: Default::default(),
		}
	}

	ChainSpec::from_genesis(
		"Crab Development Testnet",
		"crab_dev",
		ChainType::Development,
		genesis,
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		Some(properties()),
		Default::default(),
	)
}
