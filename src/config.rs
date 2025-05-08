use alloy::primitives::{address, Address};
use config::Config as ConfigHelper;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::time::Duration;

const DEFAULT_CHAIN_ID: u64 = 80000;
const DEFAULT_DISTRIBUTOR_ADDRESS: Address = address!("0x211bE45338B7C6d5721B5543Eb868547088Aca39");
const DEFAULT_BLOCK_POLL_INTERVAL: u64 = 250;
const DEFAULT_BEACON_POLL_INTERVAL: u64 = 250;
const DEFAULT_BEACON_MAX_RETRIES: usize = 12;
const DEFAULT_FALLBACK_WAIT_INTERVAL: u64 = 30000;

#[derive(Clone, Debug)]
pub struct Config {
    pub chain_id: u64,
    pub distributor: Address,
    pub block_poll_interval: Duration,
    pub beacon_poll_interval: Duration,
    pub beacon_max_retries: usize,
    pub fallback_wait_interval: Duration,
    pub keystore_password: Option<String>,
    pub healthcheck_id: Option<String>,
}

fn build_config() -> eyre::Result<Config> {
    let cfg = ConfigHelper::builder().add_source(config::File::with_name("config")).build()?;
    let distributor = cfg.get::<Address>("distributor").unwrap_or(DEFAULT_DISTRIBUTOR_ADDRESS);
    let block_poll_interval = Duration::from_millis(
        cfg.get::<u64>("block_poll_interval").unwrap_or(DEFAULT_BLOCK_POLL_INTERVAL),
    );
    let beacon_poll_interval = Duration::from_millis(
        cfg.get::<u64>("beacon_poll_interval").unwrap_or(DEFAULT_BEACON_POLL_INTERVAL),
    );
    let beacon_max_retries =
        cfg.get::<usize>("beacon_max_retries").unwrap_or(DEFAULT_BEACON_MAX_RETRIES);
    let fallback_wait_interval = Duration::from_millis(
        cfg.get::<u64>("fallback_wait_interval").unwrap_or(DEFAULT_FALLBACK_WAIT_INTERVAL),
    );
    let chain_id = cfg.get::<u64>("chain_id").unwrap_or(DEFAULT_CHAIN_ID);
    let keystore_password = cfg.get::<Option<String>>("keystore_password").unwrap_or(None);
    let healthcheck_id = cfg.get::<Option<String>>("healthcheck_id").unwrap_or(None);

    Ok(Config {
        chain_id,
        distributor,
        block_poll_interval,
        beacon_poll_interval,
        beacon_max_retries,
        fallback_wait_interval,
        keystore_password,
        healthcheck_id,
    })
}

lazy_static! {
    static ref CONFIG: Arc<Config> = {
        let config = build_config().expect("Failed to build config");
        Arc::new(config)
    };
}

pub fn get_config() -> Arc<Config> {
    Arc::clone(&CONFIG)
}
