use std::{
    collections::{BTreeMap, HashSet},
    str::FromStr,
};

use hex_literal::hex;
use serde::{Deserialize, Serialize};
// Substrate
use sc_chain_spec::{ChainType, Properties};
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
#[allow(unused_imports)]
use sp_core::ecdsa;
use sp_core::{sr25519, storage::Storage, Pair, Public, H160, U256};
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};
use sp_state_machine::BasicExternalities;
// Frontier
use creditcoin3_runtime::{
    opaque::SessionKeys, AccountId, BabeConfig, Balance, EnableManualSeal, ImOnlineId,
    RuntimeGenesisConfig, SS58Prefix, SessionConfig, Signature, StakingConfig, WASM_BINARY,
};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

/// Specialized `ChainSpec` for development.
pub type DevChainSpec = sc_service::GenericChainSpec<DevGenesisExt>;

/// Extension for the dev genesis config to support a custom changes to the genesis state.
#[derive(Serialize, Deserialize)]
pub struct DevGenesisExt {
    /// Genesis config.
    genesis_config: RuntimeGenesisConfig,
    /// The flag that if enable manual-seal mode.
    enable_manual_seal: Option<bool>,
}

impl sp_runtime::BuildStorage for DevGenesisExt {
    fn assimilate_storage(&self, storage: &mut Storage) -> Result<(), String> {
        BasicExternalities::execute_with_storage(storage, || {
            if let Some(enable_manual_seal) = &self.enable_manual_seal {
                EnableManualSeal::set(enable_manual_seal);
            }
        });
        self.genesis_config.assimilate_storage(storage)
    }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

#[allow(dead_code)]
type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
/// For use with `AccountId32`, `dead_code` if `AccountId20`.
#[allow(dead_code)]
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

type AuthorityKeys = (AccountId, GrandpaId, BabeId, ImOnlineId);

/// Generate authority keys.
pub fn authority_keys_from_seed(s: &str) -> AuthorityKeys {
    (
        get_account_id_from_seed::<sr25519::Public>(s),
        get_from_seed::<GrandpaId>(s),
        get_from_seed::<BabeId>(s),
        get_from_seed::<ImOnlineId>(s),
    )
}

pub fn session_keys(grandpa: GrandpaId, babe: BabeId, im_online: ImOnlineId) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
    }
}

fn properties() -> Properties {
    let mut properties = Properties::new();
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), SS58Prefix::get().into());
    properties
}

const UNITS: Balance = 1_000_000_000_000_000_000;

pub fn devnet_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../../chainspecs/devnetSpecRaw.json")[..])
}

pub fn development_config(enable_manual_seal: Option<bool>) -> DevChainSpec {
    let wasm_binary = WASM_BINARY.expect("WASM not available");

    DevChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            DevGenesisExt {
                genesis_config: testnet_genesis(
                    wasm_binary,
                    // Sudo account (Alice)
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    // Pre-funded accounts
                    vec![
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        get_account_id_from_seed::<sr25519::Public>("Dave"),
                        get_account_id_from_seed::<sr25519::Public>("Eve"),
                        get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    ],
                    vec![
                        hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac"), // Alith
                        hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0"), // Baltathar
                        hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc"), // Charleth
                        hex!("773539d4Ac0e786233D90A233654ccEE26a613D9"), // Dorothy
                        hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB"), // Ethan
                        hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d"), // Faith
                    ],
                    // Initial PoA authorities
                    vec![authority_keys_from_seed("Alice")],
                    // Ethereum chain ID
                    SS58Prefix::get() as u64,
                ),
                enable_manual_seal,
            }
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Fork ID
        None,
        // Properties
        Some(properties()),
        // Extensions
        None,
    )
}

pub fn local_testnet_config() -> ChainSpec {
    let wasm_binary = WASM_BINARY.expect("WASM not available");

    ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                // Sudo account (Alice)
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                ],
                vec![
                    hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac"), // Alith
                    hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0"), // Baltathar
                    hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc"), // Charleth
                    hex!("773539d4Ac0e786233D90A233654ccEE26a613D9"), // Dorothy
                    hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB"), // Ethan
                    hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d"), // Faith
                ],
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                ],
                SS58Prefix::get() as u64,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Fork ID
        None,
        // Properties
        Some(properties()),
        // Extensions
        None,
    )
}

fn genesis_account(balance: U256) -> fp_evm::GenesisAccount {
    fp_evm::GenesisAccount {
        nonce: U256::from(0),
        balance,
        storage: Default::default(),
        code: Default::default(),
    }
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    sudo_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    endowed_evm_accounts: Vec<[u8; 20]>,
    initial_authorities: Vec<AuthorityKeys>,
    chain_id: u64,
) -> RuntimeGenesisConfig {
    use creditcoin3_runtime::{
        BalancesConfig, EVMChainIdConfig, EVMConfig, SudoConfig, SystemConfig,
    };

    const STASH: u128 = 1_000_000 * UNITS;
    const ENDOWMENT: u128 = 1_000_000 * UNITS;

    RuntimeGenesisConfig {
        // System
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            ..Default::default()
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: Some(sudo_key),
        },

        // Monetary
        balances: BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .chain(initial_authorities.iter().map(|x| x.0.clone()))
                .collect::<HashSet<_>>()
                .into_iter()
                .map(|k| (k, ENDOWMENT))
                .collect(),
        },
        transaction_payment: Default::default(),

        // Consensus
        babe: BabeConfig {
            epoch_config: Some(creditcoin3_runtime::BABE_GENESIS_EPOCH_CONFIG),
            ..Default::default()
        },
        grandpa: Default::default(),

        im_online: Default::default(),
        session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|(acct, grandpa, babe, imon)| {
                    (
                        acct.clone(),
                        acct.clone(),
                        session_keys(grandpa.clone(), babe.clone(), imon.clone()),
                    )
                })
                .collect(),
        },
        staking: StakingConfig {
            validator_count: initial_authorities.len() as u32,
            minimum_validator_count: 1,
            stakers: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        STASH,
                        creditcoin3_runtime::StakerStatus::Validator,
                    )
                })
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        },

        // EVM compatibility
        evm_chain_id: EVMChainIdConfig {
            chain_id,
            ..Default::default()
        },
        evm: EVMConfig {
            accounts: {
                let mut map = BTreeMap::new();
                map.insert(
                    // H160 address of Alice dev account
                    // Derived from SS58 (42 prefix) address
                    // SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
                    // hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
                    // Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
                    H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
                        .expect("internal H160 is valid; qed"),
                    fp_evm::GenesisAccount {
                        balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
                            .expect("internal U256 is valid; qed"),
                        code: Default::default(),
                        nonce: Default::default(),
                        storage: Default::default(),
                    },
                );
                map.insert(
                    // H160 address of CI test runner account
                    H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b")
                        .expect("internal H160 is valid; qed"),
                    fp_evm::GenesisAccount {
                        balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
                            .expect("internal U256 is valid; qed"),
                        code: Default::default(),
                        nonce: Default::default(),
                        storage: Default::default(),
                    },
                );
                map.insert(
                    // H160 address for benchmark usage
                    H160::from_str("1000000000000000000000000000000000000001")
                        .expect("internal H160 is valid; qed"),
                    fp_evm::GenesisAccount {
                        nonce: U256::from(1),
                        balance: U256::from(1_000_000_000_000_000_000_000_000u128),
                        storage: Default::default(),
                        code: vec![0x00],
                    },
                );
                let one_mil = U256::from_str("0xd3c21bcecceda1000000").unwrap();
                for acct in endowed_evm_accounts {
                    let acct = H160::from(acct);
                    map.insert(acct, genesis_account(one_mil));
                }
                map
            },
            ..Default::default()
        },
        ethereum: Default::default(),
        dynamic_fee: Default::default(),
        base_fee: Default::default(),
        nomination_pools: Default::default(),
    }
}
