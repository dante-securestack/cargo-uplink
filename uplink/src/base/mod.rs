use std::{collections::HashMap, fmt::Debug};

use serde::{Deserialize, Serialize};

pub mod actions;
pub mod middleware;
pub mod monitor;
pub mod mqtt;
pub mod serializer;

pub const DEFAULT_TIMEOUT: u64 = 60;

#[inline]
fn default_timeout() -> u64 {
    DEFAULT_TIMEOUT
}

fn default_file_size() -> usize {
    104857600 // 100MB
}

fn default_file_count() -> usize {
    3
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct StreamConfig {
    pub topic: Option<String>,
    pub buf_size: usize,
    #[serde(default = "default_timeout")]
    /// Duration(in seconds) that bridge collector waits from
    /// receiving first element, before the stream gets flushed.
    pub flush_period: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Persistence {
    pub path: String,
    #[serde(default = "default_file_size")]
    pub max_file_size: usize,
    #[serde(default = "default_file_count")]
    pub max_file_count: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Authentication {
    pub ca_certificate: String,
    pub device_certificate: String,
    pub device_private_key: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Stats {
    pub enabled: bool,
    pub process_names: Vec<String>,
    pub update_period: u64,
    pub stream_size: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SimulatorConfig {
    /// number of devices to be simulated
    pub num_devices: u32,
    /// path to directory containing files with gps paths to be used in simulation
    pub gps_paths: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct JournalctlConfig {
    pub tags: Vec<String>,
    pub priority: u8,
    pub stream_size: Option<usize>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Downloader {
    pub actions: Vec<String>,
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct StreamMetricsConfig {
    pub enabled: bool,
    pub blacklist: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SerializerMetricsConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    pub project_id: String,
    pub device_id: String,
    pub broker: String,
    pub port: u16,
    pub authentication: Option<Authentication>,
    pub bridge_port: u16,
    pub max_packet_size: usize,
    pub max_inflight: u16,
    pub keep_alive: u64,
    pub actions: Vec<String>,
    pub persistence: Option<Persistence>,
    pub streams: HashMap<String, StreamConfig>,
    pub action_status: StreamConfig,
    pub serializer_metrics: StreamMetricsConfig,
    pub stream_metrics: SerializerMetricsConfig,
    /// List of streams that are to be ignored by uplink's internal metrics collectors
    #[serde(default)]
    pub bypass_streams: Vec<String>,
    pub downloader: Option<Downloader>,
    pub stats: Stats,
    pub simulator: Option<SimulatorConfig>,
    #[serde(default)]
    pub ignore_actions_if_no_clients: bool,
    #[cfg(target_os = "linux")]
    pub journalctl: Option<JournalctlConfig>,
    #[cfg(target_os = "android")]
    pub run_logcat: bool,
}
