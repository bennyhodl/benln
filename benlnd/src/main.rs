pub mod benln {
    tonic::include_proto!("benln");
}
mod grpc;
mod node;

use crate::node::BenLnNode;
use benln::ben_ln_server::BenLnServer;
use ldk_node::SocketAddress;
use ldk_node::{bitcoin::Network, Builder, LogLevel};
use std::str::FromStr;
use tonic::transport::Server;
use tracing::{info, Level};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let builder = Builder::new()
        .set_network(Network::Testnet)
        .set_esplora_server("https://blockstream.info/testnet/api".to_string())
        .set_storage_dir_path("../../.benln".to_string())
        .set_log_level(LogLevel::Info)
        .set_listening_address(SocketAddress::from_str("0.0.0.0:9735").unwrap())
        .build()
        .unwrap();

    builder.start().unwrap();

    info!("heyhowareya, ben!");

    let addr = "[::1]:3030".parse().unwrap();
    let node = BenLnNode::new(builder);

    Server::builder()
        .add_service(BenLnServer::new(node))
        .serve(addr)
        .await?;

    Ok(())
}
