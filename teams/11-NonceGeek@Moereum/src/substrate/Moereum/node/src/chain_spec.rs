use cumulus_primitives_core::ParaId;
use sc_service::ChainType;
use sp_core::{sr25519, Pair, Public, crypto::Ss58Codec, H160, U256};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};
use moereum_runtime::{
    GenesisConfig, SudoConfig, SystemConfig, BalancesConfig, WASM_BINARY, ParachainInfoConfig,
    VestingConfig, MvmConfig, TransactionPauseConfig, ParachainStakingConfig, InflationInfo,
    Range, AuthorFilterConfig, AuthorMappingConfig, TreasuryConfig, TokensConfig,
    DemocracyConfig, PolkadotXcmConfig, EligibilityValue,EvmConfig, EthereumConfig,EthereumChainIdConfig,GenesisAccount
};
use primitives::{currency::CurrencyId, AccountId, Signature, Balance, BlockNumber};
use constants::SS58_PREFIX;
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::{include_bytes, str::from_utf8};

use nimbus_primitives::NimbusId;
use crate::vm_config::build as build_vm_config;

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

// Get a public key from address.
pub fn get_public_from_address<TPublic: Public>(addr: &str) -> TPublic {
    TPublic::from_ss58check(addr).unwrap()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an account ID from address.
pub fn get_account_id_from_address(addr: &str) -> AccountId {
    AccountId::from_ss58check(addr).unwrap()
}

/// The network properties.
fn properties() -> Option<sc_chain_spec::Properties> {
    let currency = CurrencyId::default();

    json!({
        "ss58Format": SS58_PREFIX,
        "tokenDecimals": currency.decimals(),
        "tokenSymbol": from_utf8(&currency.symbol()).unwrap(),
    })
    .as_object()
    .cloned()
}

/// Local testnet configuration.
pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Testnet wasm not available".to_string())?;
    let parachain_id = ParaId::from(2000);

    Ok(ChainSpec::from_genesis(
        // Name
        "Moereum Testnet",
        // ID
        "moereum_testnet",
        ChainType::Local,
        move || {
            genesis(
                wasm_binary,
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Candidates
                vec![(
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_from_seed::<NimbusId>("Alice"),
                    CurrencyId::NATIVE * 10_000,
                )],
                // Nominators
                vec![],
                // Pre-funded accounts
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Dave"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Eve"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                        CurrencyId::NATIVE * 100_000,
                    ),
                ],
                // Vesting accounts
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        1000,
                        150,
                        CurrencyId::NATIVE * 50_000,
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        1000,
                        150,
                        CurrencyId::NATIVE * 50_000,
                    ),
                ],
                // Paused extrinsics
                vec![],
                // Parachain ID
                parachain_id,
                vec![
                    // Alith
                    H160::from(hex_literal::hex!["f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac"]),
                    // Baltathar
                    H160::from(hex_literal::hex!["3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0"]),
                    // Charleth
                    H160::from(hex_literal::hex!["798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc"]),
                    // Dorothy
                    H160::from(hex_literal::hex!["773539d4Ac0e786233D90A233654ccEE26a613D9"]),
                    // Ethan
                    H160::from(hex_literal::hex!["Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB"]),
                    // Faith
                    H160::from(hex_literal::hex!["C0F0f4ab324C46e55D02D0033343B4Be8A55532d"]),
                    // Goliath
                    H160::from(hex_literal::hex!["7BF369283338E12C90514468aa3868A551AB2929"]),
                    // Heath
                    H160::from(hex_literal::hex!["931f3600a299fd9B24cEfB3BfF79388D19804BeA"]),
                    // Ida
                    H160::from(hex_literal::hex!["C41C5F1123ECCd5ce233578B2e7ebd5693869d73"]),
                    // Judith
                    H160::from(hex_literal::hex!["2898FE7a42Be376C8BC7AF536A940F7Fd5aDd423"]),
                ],
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("moereum_testnet"),
        // Fork ID
        None,
        // Properties
        properties(),
        // Extensions
        Extensions {
            relay_chain: "rococo-local".into(),
            para_id: parachain_id.into(),
        },
    ))
}

/// Configure initial storage state for FRAME modules.
pub fn genesis(
    wasm_binary: &[u8],
    root_key: AccountId,
    candidates: Vec<(AccountId, NimbusId, Balance)>,
    delegations: Vec<(AccountId, AccountId, Balance)>,
    balances: Vec<(AccountId, Balance)>,
    vesting: Vec<(AccountId, BlockNumber, BlockNumber, Balance)>,
    paused: Vec<(Vec<u8>, Vec<u8>)>,
    id: ParaId,
    addresses: Vec<H160>,
) -> GenesisConfig {
    let (init_module, init_func, init_args) = build_vm_config();

    let move_stdlib =
        include_bytes!("../../move/move-stdlib/build/MoveStdlib/bundles/MoveStdlib.pac").to_vec();
    let pont_framework =
        include_bytes!("../../move/moereum-stdlib/build/MoereumStdlib/bundles/MoereumStdlib.pac").to_vec();

    GenesisConfig {
        tokens: TokensConfig { balances: vec![] },
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
        },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1000 tokens.
            balances,
        },
        parachain_system: Default::default(),
        polkadot_xcm: PolkadotXcmConfig {
            safe_xcm_version: Some(2),
        },
        parachain_info: ParachainInfoConfig { parachain_id: id },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: Some(root_key),
        },
        parachain_staking: ParachainStakingConfig {
            candidates: candidates
                .iter()
                .cloned()
                .map(|(account, _, bond)| (account, bond))
                .collect(),
            delegations,
            inflation_config: moereum_inflation_config(),
        },
        author_filter: AuthorFilterConfig {
            eligible_count: EligibilityValue::new_unchecked(50),
        },
        author_mapping: AuthorMappingConfig {
            mappings: candidates
                .iter()
                .cloned()
                .map(|(account_id, author_id, _)| (author_id, account_id))
                .collect(),
        },
        ethereum_chain_id: EthereumChainIdConfig { chain_id: 1209u64 },
        evm: EvmConfig {
            accounts: addresses
                .into_iter()
                .map(|addr| {
                    (
                        addr,
                        GenesisAccount {
                            balance: U256::from(1_000_000_000_000_000_000_000u128),
                            nonce: Default::default(),
                            code: Default::default(),
                            storage: Default::default(),
                        },
                    )
                })
                .collect(),
        },
        ethereum: EthereumConfig {},
        mvm: MvmConfig {
            move_stdlib,
            pont_framework,
            init_module,
            init_func,
            init_args,
            ..Default::default()
        },
        transaction_pause: TransactionPauseConfig {
            paused,
            ..Default::default()
        },
        vesting: VestingConfig { vesting },
        treasury: TreasuryConfig {},
        democracy: DemocracyConfig::default(),
    }
}

// Moereum inflation.
pub fn moereum_inflation_config() -> InflationInfo<Balance> {
    // Let's say we have 100M total supply coins.
    InflationInfo {
        // How much staked coins we expect.
        expect: Range {
            min: CurrencyId::NATIVE * 10_000_000, // We expect to have staked at least 10M coins.
            ideal: CurrencyId::NATIVE * 20_000_000, // We expect to have staked ideal 20M coins.
            max: CurrencyId::NATIVE * 50_000_000, // We expect to have staked maximum 50M coins.
        },
        annual: Range {
            min: Perbill::from_percent(4),   // We expect minimum inflation is 4%.
            ideal: Perbill::from_percent(4), // We expect ideal inflation is 4%.
            max: Perbill::from_percent(5),   // We expect max inflation is 5%.
        },
        // 8766 rounds (hours) in a year
        round: Range {
            min: Perbill::from_parts(Perbill::from_percent(4).deconstruct() / 8766),
            ideal: Perbill::from_parts(Perbill::from_percent(4).deconstruct() / 8766),
            max: Perbill::from_parts(Perbill::from_percent(5).deconstruct() / 8766),
        },
    }
}
