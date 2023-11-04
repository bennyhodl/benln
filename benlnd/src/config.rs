use thiserror::Error;
use ldk_node::{LogLevel, bitcoin::Network};

use crate::BenLndArgs;

pub struct BenLndConfig {
    pub level: LogLevel,
    pub listen: u32,
    pub network: Network
}
#[derive(Serialize, Deserialize, Parser)]
#[clap(author, about, version)]
struct BenLndArgs {
    #[arg(short, long, default_value = "info")]
    level: String,

    // #[arg(short, long, default_value = 9735)]
    // listen: u32,

    #[arg(short, long, default_value = "testnet")]
    network: String
}

#[derive(Debug, Error)]
enum BenLndArgsError {
    #[error("Cannot use mainnet just yet.")]
    NoMainnet,
    #[error("This network is not supported yet.")]
    NetworkNotSupported,
    #[error("No log leve given")]
    LogLevel
}

impl BenLndConfig {
    pub fn config() -> anyhow::Result<BenLndConfig> {
        let network = match args.network.as_str() {
            "testnet" => Network::Testnet,
            "mainnet" => return Err(BenLndArgsError::NoMainnet.into()),
            _ => return Err(BenLndArgsError::NetworkNotSupported.into()),
        };

        let level = match args.level.as_str() {
            "level" => LogLevel::Info,
            "debug" => LogLevel::Debug,
            "trace" => LogLevel::Trace,
            _ => return Err(BenLndArgsError::LogLevel.into())
        };

        Ok(BenLndConfig { level, listen: 9735, network })
    }
}

