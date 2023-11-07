mod grpc;
mod node;

use crate::node::BenLnNode;
use benlnproto::benln::ben_ln_server::BenLnServer;
use ldk_node::{bitcoin::Network, lightning::ln::msgs::SocketAddress, Builder, LogLevel, Event};
use std::str::FromStr;
use tonic::transport::Server;
use tracing::{info, Level};
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let builder = Builder::new()
        .set_network(Network::Regtest)
        // .set_esplora_server("https://blockstream.info/testnet/api".to_string())
        .set_bitcoind_server(
            "http://127.0.0.1:18444".to_string(),
            "polaruser".to_string(),
            "polarpass".to_string(),
        )
        .set_storage_dir_path("../.benln".to_string())
        .set_log_level(LogLevel::Trace)
        .set_listening_address(SocketAddress::from_str("0.0.0.0:9735").unwrap())
        .build()
        .unwrap();

    builder.start()?;

    info!("heyhowareya, ben!");

    let addr = "[::1]:3030".parse().unwrap();
    let node = BenLnNode::new(Arc::new(builder));
    let node_watcher = node.node.clone();

    tokio::task::spawn(async move {
        loop {
            match node_watcher.wait_next_event() {
                Event::PaymentSuccessful { payment_hash }=> println!("EVENT: Payment successful {:?}", payment_hash),
                Event::PaymentReceived { .. } => println!("EVENT: Payment received"),
                Event::ChannelPending { .. } => println!("EVENT: Channel pending!"),
                Event::ChannelReady { .. } => println!("EVENT: Channel ready!"),
                Event::PaymentFailed { .. }=> println!("EVENT: Payment failed!"),
                Event::ChannelClosed { .. } => println!("EVENT: Channel closed!"),
            }
        }
    });

    Server::builder()
        .add_service(BenLnServer::new(node))
        .serve(addr)
        .await?;

    Ok(())
}
