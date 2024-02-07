use lazy_static::lazy_static;
use phf::phf_map;
use prometheus::{opts, register_gauge_vec, register_int_gauge_vec, GaugeVec, IntGaugeVec};

pub(crate) static COINGECKO_IDS: phf::Map<&'static str, &'static str> = phf_map! {
    "BTC/USD" => "bitcoin",
    "ETH/USD" => "ethereum",
    "LUSD/USD" => "liquity-usd",
    "WBTC/USD" => "wrapped-bitcoin",
    "DAI/USD" => "dai",
    "USDC/USD" => "usd-coin",
    "USDT/USD" => "tether",
    "WSTETH/USD" => "wrapped-steth",
    "LORDS/USD" => "lords",
};

lazy_static! {
    pub static ref TIME_SINCE_LAST_UPDATE_PUBLISHER: GaugeVec = register_gauge_vec!(
        opts!(
            "time_since_last_update_seconds",
            "Time since the last update in seconds."
        ),
        &["network", "publisher"]
    )
    .unwrap();
    pub static ref PAIR_PRICE: GaugeVec = register_gauge_vec!(
        opts!("pair_price", "Price of the pair from the source."),
        &["network", "pair", "source", "type"]
    )
    .unwrap();
    pub static ref TIME_SINCE_LAST_UPDATE_PAIR_ID: GaugeVec = register_gauge_vec!(
        opts!(
            "time_since_last_update_pair_id",
            "Time since the last update in seconds."
        ),
        &["network", "pair", "type"]
    )
    .unwrap();
    pub static ref PRICE_DEVIATION: GaugeVec = register_gauge_vec!(
        opts!(
            "price_deviation",
            "Price deviation from the reference price."
        ),
        &["network", "pair", "source", "type"]
    )
    .unwrap();
    pub static ref PRICE_DEVIATION_SOURCE: GaugeVec = register_gauge_vec!(
        opts!(
            "price_deviation_source",
            "Price deviation from the reference price."
        ),
        &["network", "pair", "source", "type"]
    )
    .unwrap();
    pub static ref NUM_SOURCES: IntGaugeVec = register_int_gauge_vec!(
        opts!(
            "num_sources",
            "Number of sources that have published data for a pair."
        ),
        &["network", "pair", "type"]
    )
    .unwrap();
    pub static ref INDEXER_BLOCKS_LEFT: IntGaugeVec = register_int_gauge_vec!(
        opts!(
            "indexer_blocks_left",
            "Number of blocks left to index for a given indexer."
        ),
        &["network", "type"]
    )
    .unwrap();
    pub static ref PUBLISHER_BALANCE: GaugeVec = register_gauge_vec!(
        opts!("publisher_balance", "Balance of the publisher in ETH"),
        &["network", "publisher"]
    )
    .unwrap();
    pub static ref API_PRICE_DEVIATION: GaugeVec = register_gauge_vec!(
        opts!(
            "api_price_deviation",
            "Price deviation from the reference price."
        ),
        &["network", "pair"]
    )
    .unwrap();
    pub static ref ON_OFF_PRICE_DEVIATION: GaugeVec = register_gauge_vec!(
        opts!(
            "on_off_price_deviation",
            "On chain price deviation from the reference price"
        ),
        &["network", "pair", "type"]
    )
    .unwrap();
    pub static ref API_TIME_SINCE_LAST_UPDATE: GaugeVec = register_gauge_vec!(
        opts!(
            "api_time_since_last_update",
            "Time since the last update in seconds."
        ),
        &["network", "pair"]
    )
    .unwrap();
    pub static ref API_NUM_SOURCES: IntGaugeVec = register_int_gauge_vec!(
        opts!(
            "api_num_sources",
            "Number of sources aggregated for a pair."
        ),
        &["network", "pair"]
    )
    .unwrap();
    pub static ref API_SEQUENCER_DEVIATION: GaugeVec = register_gauge_vec!(
        opts!(
            "api_sequencer_deviation",
            "Price deviation from starknet gateway price."
        ),
        &["network"]
    )
    .unwrap();
    pub static ref VRF_REQUESTS: IntGaugeVec = register_int_gauge_vec!(
        opts!("vrf_requests", "Number of VRF requests."),
        &["network", "status", "requestor"]
    )
    .unwrap();
}

pub const FEE_TOKEN_DECIMALS: i32 = 18;
pub const FEE_TOKEN_ADDRESS: &str =
    "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7";

pub const CONFIG_UPDATE_INTERVAL: u64 = 3 * 3600;
