pub mod benln {
    tonic::include_proto!("benln");
}
use clap::{Parser, Subcommand};
use benln::{ben_ln_client::BenLnClient, GetNodeInfoRequest};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct BenLnCliArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Gets information about the node.
    Info,
    // Decode(DecodeCommand)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: BenLnCliArgs = BenLnCliArgs::parse();

    let mut client = BenLnClient::connect("http://[::1]:3030").await?;

    match args.entity_type {
        EntityType::Info => {
            let msg = GetNodeInfoRequest {};
            let response = client.get_node_info(msg).await?.into_inner();

            println!("Node Id: {}", response.node_id)
        }
    }

    Ok(())
}
