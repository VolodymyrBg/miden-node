use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

use miden_node_utils::config::{
    Endpoint, Protocol, DEFAULT_FAUCET_SERVER_PORT, DEFAULT_NODE_RPC_PORT,
};
use serde::{Deserialize, Serialize};

// Faucet config
// ================================================================================================

/// Default path to the faucet account file
pub const DEFAULT_FAUCET_ACCOUNT_PATH: &str = "accounts/faucet.mac";

/// Default timeout for RPC requests
pub const DEFAULT_RPC_TIMEOUT_MS: u64 = 10000;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FaucetConfig {
    /// Endpoint of the faucet
    pub endpoint: Endpoint,
    /// Node RPC gRPC endpoint in the format `http://<host>[:<port>]`
    pub node_url: String,
    /// Timeout for RPC requests in milliseconds
    pub timeout_ms: u64,
    /// Possible options on the amount of asset that should be dispersed on each faucet request
    pub asset_amount_options: Vec<u64>,
    /// Path to the faucet account file
    pub faucet_account_path: PathBuf,
}

impl Display for FaucetConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{{ endpoint: \"{}\", node_url: \"{}\", timeout_ms: \"{}\", asset_amount_options: {:?}, faucet_account_path: \"{}\" }}",
            self.endpoint, self.node_url, self.timeout_ms, self.asset_amount_options, self.faucet_account_path.display()
        ))
    }
}

impl Default for FaucetConfig {
    fn default() -> Self {
        Self {
            endpoint: Endpoint {
                host: "0.0.0.0".to_string(),
                port: DEFAULT_FAUCET_SERVER_PORT,
                protocol: Protocol::Http,
            },
            node_url: Endpoint::localhost(DEFAULT_NODE_RPC_PORT).to_string(),
            timeout_ms: DEFAULT_RPC_TIMEOUT_MS,
            asset_amount_options: vec![100, 500, 1000],
            faucet_account_path: DEFAULT_FAUCET_ACCOUNT_PATH.into(),
        }
    }
}
