//! Chain Client Configuration
use crate::ExternalStorageLocation::GCS;
use crate::MutableConfigValue;
use near_primitives::types::{
    AccountId, BlockHeight, BlockHeightDelta, Gas, NumBlocks, NumSeats, ShardId,
};
use near_primitives::version::Version;
use std::cmp::{max, min};
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

pub const TEST_STATE_SYNC_TIMEOUT: u64 = 5;

#[derive(Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum LogSummaryStyle {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "colored")]
    Colored,
}

/// Minimum number of epochs for which we keep store data
pub const MIN_GC_NUM_EPOCHS_TO_KEEP: u64 = 3;

/// Default number of epochs for which we keep store data
pub const DEFAULT_GC_NUM_EPOCHS_TO_KEEP: u64 = 5;

/// Default number of concurrent requests to external storage to fetch state parts.
pub const DEFAULT_STATE_SYNC_NUM_CONCURRENT_REQUESTS_EXTERNAL: u32 = 25;
pub const DEFAULT_STATE_SYNC_NUM_CONCURRENT_REQUESTS_ON_CATCHUP_EXTERNAL: u32 = 5;

/// Configuration for garbage collection.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(default)]
pub struct GCConfig {
    /// Maximum number of blocks to garbage collect at every garbage collection
    /// call.
    pub gc_blocks_limit: NumBlocks,

    /// Maximum number of height to go through at each garbage collection step
    /// when cleaning forks during garbage collection.
    pub gc_fork_clean_step: u64,

    /// Number of epochs for which we keep store data.
    pub gc_num_epochs_to_keep: u64,
}

impl Default for GCConfig {
    fn default() -> Self {
        Self {
            gc_blocks_limit: 2,
            gc_fork_clean_step: 100,
            gc_num_epochs_to_keep: DEFAULT_GC_NUM_EPOCHS_TO_KEEP,
        }
    }
}

impl GCConfig {
    pub fn gc_num_epochs_to_keep(&self) -> u64 {
        max(MIN_GC_NUM_EPOCHS_TO_KEEP, self.gc_num_epochs_to_keep)
    }
}

fn default_num_concurrent_requests() -> u32 {
    DEFAULT_STATE_SYNC_NUM_CONCURRENT_REQUESTS_EXTERNAL
}

fn default_num_concurrent_requests_during_catchup() -> u32 {
    DEFAULT_STATE_SYNC_NUM_CONCURRENT_REQUESTS_ON_CATCHUP_EXTERNAL
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ExternalStorageConfig {
    /// Location of state parts.
    pub location: ExternalStorageLocation,
    /// When fetching state parts from external storage, throttle fetch requests
    /// to this many concurrent requests.
    #[serde(default = "default_num_concurrent_requests")]
    pub num_concurrent_requests: u32,
    /// During catchup, the node will use a different number of concurrent requests
    /// to reduce the performance impact of state sync.
    #[serde(default = "default_num_concurrent_requests_during_catchup")]
    pub num_concurrent_requests_during_catchup: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum ExternalStorageLocation {
    S3 {
        /// Location of state dumps on S3.
        bucket: String,
        /// Data may only be available in certain locations.
        region: String,
    },
    Filesystem {
        root_dir: PathBuf,
    },
    GCS {
        bucket: String,
    },
}

/// Configures how to dump state to external storage.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DumpConfig {
    /// Specifies where to write the obtained state parts.
    pub location: ExternalStorageLocation,
    /// Use in case a node that dumps state to the external storage
    /// gets in trouble.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_dump_for_shards: Option<Vec<ShardId>>,
    /// How often to check if a new epoch has started.
    /// Feel free to set to `None`, defaults are sensible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iteration_delay: Option<Duration>,
    /// Location of a json file with credentials allowing write access to the bucket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_file: Option<PathBuf>,
}

/// Configures how to fetch state parts during state sync.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum SyncConfig {
    /// Syncs state from the peers without reading anything from external storage.
    Peers,
    /// Expects parts to be available in external storage.
    ExternalStorage(ExternalStorageConfig),
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self::Peers
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Default)]
/// Options for dumping state to S3.
pub struct StateSyncConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `none` value disables state dump to external storage.
    pub dump: Option<DumpConfig>,
    #[serde(skip_serializing_if = "SyncConfig::is_default", default = "SyncConfig::default")]
    pub sync: SyncConfig,
}

impl SyncConfig {
    /// Checks whether the object equals its default value.
    fn is_default(&self) -> bool {
        matches!(self, Self::Peers)
    }
}

// A handle that allows the main process to interrupt resharding if needed.
// This typically happens when the main process is interrupted.
#[derive(Clone)]
pub struct StateSplitHandle {
    keep_going: Arc<AtomicBool>,
}

impl StateSplitHandle {
    pub fn new() -> Self {
        Self { keep_going: Arc::new(AtomicBool::new(true)) }
    }

    pub fn get(&self) -> bool {
        self.keep_going.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn stop(&self) -> () {
        self.keep_going.store(false, std::sync::atomic::Ordering::Relaxed);
    }
}

/// Configuration for resharding.
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(default)]
pub struct StateSplitConfig {
    /// The soft limit on the size of a single batch. The batch size can be
    /// decreased if resharding is consuming too many resources and interfering
    /// with regular node operation.
    pub batch_size: bytesize::ByteSize,

    /// The delay between writing batches to the db. The batch delay can be
    /// increased if resharding is consuming too many resources and interfering
    /// with regular node operation.
    pub batch_delay: Duration,

    /// The delay between attempts to start resharding while waiting for the
    /// state snapshot to become available.
    pub retry_delay: Duration,

    /// The delay between the resharding request is received and when the actor
    /// actually starts working on it. This delay should only be used in tests.
    pub initial_delay: Duration,

    /// The maximum time that the actor will wait for the snapshot to be ready,
    /// before starting resharding. Do not wait indefinitely since we want to
    /// report error early enough for the node maintainer to have time to recover.
    pub max_poll_time: Duration,
}

impl Default for StateSplitConfig {
    fn default() -> Self {
        // Conservative default for a slower resharding that puts as little
        // extra load on the node as possible.
        Self {
            batch_size: bytesize::ByteSize::kb(500),
            batch_delay: Duration::from_millis(100),
            retry_delay: Duration::from_secs(10),
            initial_delay: Duration::from_secs(0),
            // The snapshot typically is available within a minute from the
            // epoch start. Set the default higher in case we need to wait for
            // state sync.
            max_poll_time: Duration::from_secs(2 * 60 * 60), // 2 hours
        }
    }
}

pub fn default_header_sync_initial_timeout() -> Duration {
    Duration::from_secs(10)
}

pub fn default_header_sync_progress_timeout() -> Duration {
    Duration::from_secs(2)
}

pub fn default_header_sync_stall_ban_timeout() -> Duration {
    Duration::from_secs(120)
}

pub fn default_state_sync_timeout() -> Duration {
    Duration::from_secs(60)
}

pub fn default_header_sync_expected_height_per_second() -> u64 {
    10
}

pub fn default_sync_check_period() -> Duration {
    Duration::from_secs(10)
}

pub fn default_sync_step_period() -> Duration {
    Duration::from_millis(10)
}

pub fn default_sync_height_threshold() -> u64 {
    1
}

pub fn default_epoch_sync_enabled() -> bool {
    false
}

pub fn default_state_sync() -> Option<StateSyncConfig> {
    Some(StateSyncConfig {
        dump: None,
        sync: SyncConfig::ExternalStorage(ExternalStorageConfig {
            location: GCS { bucket: "state-parts".to_string() },
            num_concurrent_requests: DEFAULT_STATE_SYNC_NUM_CONCURRENT_REQUESTS_EXTERNAL,
            num_concurrent_requests_during_catchup:
                DEFAULT_STATE_SYNC_NUM_CONCURRENT_REQUESTS_ON_CATCHUP_EXTERNAL,
        }),
    })
}

pub fn default_state_sync_enabled() -> bool {
    true
}

pub fn default_view_client_threads() -> usize {
    4
}

pub fn default_log_summary_period() -> Duration {
    Duration::from_secs(10)
}

pub fn default_view_client_throttle_period() -> Duration {
    Duration::from_secs(30)
}

pub fn default_trie_viewer_state_size_limit() -> Option<u64> {
    Some(50_000)
}

pub fn default_transaction_pool_size_limit() -> Option<u64> {
    Some(100_000_000) // 100 MB.
}

pub fn default_tx_routing_height_horizon() -> BlockHeightDelta {
    4
}

pub fn default_enable_multiline_logging() -> Option<bool> {
    Some(true)
}

pub fn default_produce_chunk_add_transactions_time_limit() -> Option<Duration> {
    Some(Duration::from_millis(200))
}

/// ClientConfig where some fields can be updated at runtime.
#[derive(Clone, serde::Serialize)]
pub struct ClientConfig {
    /// Version of the binary.
    pub version: Version,
    /// Chain id for status.
    pub chain_id: String,
    /// Listening rpc port for status.
    pub rpc_addr: Option<String>,
    /// Graceful shutdown at expected block height.
    pub expected_shutdown: MutableConfigValue<Option<BlockHeight>>,
    /// Duration to check for producing / skipping block.
    pub block_production_tracking_delay: Duration,
    /// Minimum duration before producing block.
    pub min_block_production_delay: Duration,
    /// Maximum wait for approvals before producing block.
    pub max_block_production_delay: Duration,
    /// Maximum duration before skipping given height.
    pub max_block_wait_delay: Duration,
    /// Skip waiting for sync (for testing or single node testnet).
    pub skip_sync_wait: bool,
    /// How often to check that we are not out of sync.
    pub sync_check_period: Duration,
    /// While syncing, how long to check for each step.
    pub sync_step_period: Duration,
    /// Sync height threshold: below this difference in height don't start syncing.
    pub sync_height_threshold: BlockHeightDelta,
    /// How much time to wait after initial header sync
    pub header_sync_initial_timeout: Duration,
    /// How much time to wait after some progress is made in header sync
    pub header_sync_progress_timeout: Duration,
    /// How much time to wait before banning a peer in header sync if sync is too slow
    pub header_sync_stall_ban_timeout: Duration,
    /// Expected increase of header head height per second during header sync
    pub header_sync_expected_height_per_second: u64,
    /// How long to wait for a response during state sync
    pub state_sync_timeout: Duration,
    /// Minimum number of peers to start syncing.
    pub min_num_peers: usize,
    /// Period between logging summary information.
    pub log_summary_period: Duration,
    /// Enable coloring of the logs
    pub log_summary_style: LogSummaryStyle,
    /// Produce empty blocks, use `false` for testing.
    pub produce_empty_blocks: bool,
    /// Epoch length.
    pub epoch_length: BlockHeightDelta,
    /// Number of block producer seats
    pub num_block_producer_seats: NumSeats,
    /// Time to persist Accounts Id in the router without removing them.
    pub ttl_account_id_router: Duration,
    /// Horizon at which instead of fetching block, fetch full state.
    pub block_fetch_horizon: BlockHeightDelta,
    /// Time between check to perform catchup.
    pub catchup_step_period: Duration,
    /// Time between checking to re-request chunks.
    pub chunk_request_retry_period: Duration,
    /// Time between running doomslug timer.
    pub doosmslug_step_period: Duration,
    /// Behind this horizon header fetch kicks in.
    pub block_header_fetch_horizon: BlockHeightDelta,
    /// Garbage collection configuration.
    pub gc: GCConfig,
    /// Accounts that this client tracks.
    pub tracked_accounts: Vec<AccountId>,
    /// Shards that this client tracks.
    pub tracked_shards: Vec<ShardId>,
    /// Rotate between these sets of tracked shards.
    /// Used to simulate the behavior of chunk only producers without staking tokens.
    /// This field is only used if `tracked_shards` is empty.
    pub tracked_shard_schedule: Vec<Vec<ShardId>>,
    /// Not clear old data, set `true` for archive nodes.
    pub archive: bool,
    /// save_trie_changes should be set to true iff
    /// - archive if false - non-archivale nodes need trie changes to perform garbage collection
    /// - archive is true, cold_store is configured and migration to split_storage is finished - node
    /// working in split storage mode needs trie changes in order to do garbage collection on hot.
    pub save_trie_changes: bool,
    /// Number of threads for ViewClientActor pool.
    pub view_client_threads: usize,
    /// Run Epoch Sync on the start.
    pub epoch_sync_enabled: bool,
    /// Number of seconds between state requests for view client.
    pub view_client_throttle_period: Duration,
    /// Upper bound of the byte size of contract state that is still viewable. None is no limit
    pub trie_viewer_state_size_limit: Option<u64>,
    /// Max burnt gas per view method.  If present, overrides value stored in
    /// genesis file.  The value only affects the RPCs without influencing the
    /// protocol thus changing it per-node doesn’t affect the blockchain.
    pub max_gas_burnt_view: Option<Gas>,
    /// Re-export storage layer statistics as prometheus metrics.
    pub enable_statistics_export: bool,
    /// Number of threads to execute background migration work in client.
    pub client_background_migration_threads: usize,
    /// Enables background flat storage creation.
    pub flat_storage_creation_enabled: bool,
    /// Duration to perform background flat storage creation step.
    pub flat_storage_creation_period: Duration,
    /// Whether to use the State Sync mechanism.
    /// If disabled, the node will do Block Sync instead of State Sync.
    pub state_sync_enabled: bool,
    /// Options for syncing state.
    pub state_sync: StateSyncConfig,
    /// Limit of the size of per-shard transaction pool measured in bytes. If not set, the size
    /// will be unbounded.
    pub transaction_pool_size_limit: Option<u64>,
    // Allows more detailed logging, for example a list of orphaned blocks.
    pub enable_multiline_logging: bool,
    // Configuration for resharding.
    pub state_split_config: MutableConfigValue<StateSplitConfig>,
    /// If the node is not a chunk producer within that many blocks, then route
    /// to upcoming chunk producers.
    pub tx_routing_height_horizon: BlockHeightDelta,
    /// Limit the time of adding transactions to a chunk.
    /// A node produces a chunk by adding transactions from the transaction pool until
    /// some limit is reached. This time limit ensures that adding transactions won't take
    /// longer than the specified duration, which helps to produce the chunk quickly.
    pub produce_chunk_add_transactions_time_limit: MutableConfigValue<Option<Duration>>,
}

impl ClientConfig {
    pub fn test(
        skip_sync_wait: bool,
        min_block_prod_time: u64,
        max_block_prod_time: u64,
        num_block_producer_seats: NumSeats,
        archive: bool,
        save_trie_changes: bool,
        epoch_sync_enabled: bool,
        state_sync_enabled: bool,
    ) -> Self {
        assert!(
            archive || save_trie_changes,
            "Configuration with archive = false and save_trie_changes = false is not supported \
            because non-archival nodes must save trie changes in order to do do garbage collection."
        );

        Self {
            version: Default::default(),
            chain_id: "unittest".to_string(),
            rpc_addr: Some("0.0.0.0:3030".to_string()),
            expected_shutdown: MutableConfigValue::new(None, "expected_shutdown"),
            block_production_tracking_delay: Duration::from_millis(std::cmp::max(
                10,
                min_block_prod_time / 5,
            )),
            min_block_production_delay: Duration::from_millis(min_block_prod_time),
            max_block_production_delay: Duration::from_millis(max_block_prod_time),
            max_block_wait_delay: Duration::from_millis(3 * min_block_prod_time),
            skip_sync_wait,
            sync_check_period: Duration::from_millis(100),
            sync_step_period: Duration::from_millis(10),
            sync_height_threshold: 1,
            header_sync_initial_timeout: Duration::from_secs(10),
            header_sync_progress_timeout: Duration::from_secs(2),
            header_sync_stall_ban_timeout: Duration::from_secs(30),
            state_sync_timeout: Duration::from_secs(TEST_STATE_SYNC_TIMEOUT),
            header_sync_expected_height_per_second: 1,
            min_num_peers: 1,
            log_summary_period: Duration::from_secs(10),
            produce_empty_blocks: true,
            epoch_length: 10,
            num_block_producer_seats,
            ttl_account_id_router: Duration::from_secs(60 * 60),
            block_fetch_horizon: 50,
            catchup_step_period: Duration::from_millis(1),
            chunk_request_retry_period: min(
                Duration::from_millis(100),
                Duration::from_millis(min_block_prod_time / 5),
            ),
            doosmslug_step_period: Duration::from_millis(100),
            block_header_fetch_horizon: 50,
            gc: GCConfig { gc_blocks_limit: 100, ..GCConfig::default() },
            tracked_accounts: vec![],
            tracked_shards: vec![],
            tracked_shard_schedule: vec![],
            archive,
            save_trie_changes,
            log_summary_style: LogSummaryStyle::Colored,
            view_client_threads: 1,
            epoch_sync_enabled,
            view_client_throttle_period: Duration::from_secs(1),
            trie_viewer_state_size_limit: None,
            max_gas_burnt_view: None,
            enable_statistics_export: true,
            client_background_migration_threads: 1,
            flat_storage_creation_enabled: true,
            flat_storage_creation_period: Duration::from_secs(1),
            state_sync_enabled,
            state_sync: StateSyncConfig::default(),
            transaction_pool_size_limit: None,
            enable_multiline_logging: false,
            state_split_config: MutableConfigValue::new(
                StateSplitConfig::default(),
                "state_split_config",
            ),
            tx_routing_height_horizon: 4,
            produce_chunk_add_transactions_time_limit: MutableConfigValue::new(
                default_produce_chunk_add_transactions_time_limit(),
                "produce_chunk_add_transactions_time_limit",
            ),
        }
    }
}