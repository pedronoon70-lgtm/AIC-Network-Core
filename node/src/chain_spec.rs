use node_template_runtime::{AccountId, RuntimeGenesisConfig, Signature, WASM_BINARY};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// Unidade básica de 12 casas decimais
const DOLLARS: u128 = 1_000_000_000_000;

pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

type AccountPublic = <Signature as Verify>::Signer;

/// Helper para gerar AccountId a partir de seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(
		TPublic::Pair::from_string(&format!("//{}", seed), None)
			.expect("static values are valid; qed")
			.public(),
	)
	.into_account()
}

/// Helper para gerar chaves de autoridade (Aura e Grandpa)
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(
		sr25519::Pair::from_string(&format!("//{}", s), None)
			.expect("static values are valid; qed")
			.public()
			.into(),
		sp_consensus_grandpa::AuthorityId::from(
			sp_core::ed25519::Pair::from_string(&format!("//{}", s), None)
				.expect("static values are valid; qed")
				.public(),
		),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Wasm not available".to_string())?;

	let mut props = sc_service::Properties::new();
	props.insert("tokenSymbol".into(), "AIC".into());
	props.insert("tokenDecimals".into(), 12.into());
	props.insert("ss58Format".into(), 42.into());

	Ok(ChainSpec::builder(wasm_binary, None)
		.with_name("AI-Chain Development")
		.with_id("ai_chain_dev")
		.with_chain_type(ChainType::Development)
		.with_properties(props)
		.with_genesis_config_patch(testnet_genesis(
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			vec![authority_keys_from_seed("Alice")],
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
			],
		))
		.build())
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Wasm not available".to_string())?;

	let mut props = sc_service::Properties::new();
	props.insert("tokenSymbol".into(), "AIC".into());
	props.insert("tokenDecimals".into(), 12.into());
	props.insert("ss58Format".into(), 42.into());

	Ok(ChainSpec::builder(wasm_binary, None)
		.with_name("AI-Chain Local Testnet")
		.with_id("ai_chain_local")
		.with_chain_type(ChainType::Local)
		.with_properties(props)
		.with_genesis_config_patch(testnet_genesis(
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
			vec![
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_account_id_from_seed::<sr25519::Public>("Charlie"),
				get_account_id_from_seed::<sr25519::Public>("Dave"),
				get_account_id_from_seed::<sr25519::Public>("Eve"),
			],
		))
		.build())
}

fn testnet_genesis(
	root_key: AccountId,
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	endowed_accounts: Vec<AccountId>,
) -> serde_json::Value {
	let total_supply: u128 = 21_000_000 * DOLLARS;
	let share = total_supply / endowed_accounts.len() as u128;

	serde_json::json!({
		"balances": {
			"balances": endowed_accounts.iter().cloned().map(|k| (k, share)).collect::<Vec<_>>(),
		},
		"aura": {
			"authorities": initial_authorities.iter().map(|x| (x.0.clone())).collect::<Vec<_>>(),
		},
		"grandpa": {
			"authorities": initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect::<Vec<_>>(),
		},
		"sudo": {
			"key": Some(root_key),
		},
	})
}
