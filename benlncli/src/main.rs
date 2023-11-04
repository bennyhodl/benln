pub mod benln {
    tonic::include_proto!("benln");
}
use benln::{
    ben_ln_client::BenLnClient, GetNodeInfoRequest, NewAddressRequest, SignMessageRequest,
    StopRequest,
};
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct BenLnCliArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Gets information about the node.
    Info,
    /// Generate a new on-chain address.
    NewAddress,
    /// Sign a message with the node keys.
    SignMessage(SignMessageArgs),
    /// Stop the node.
    Stop,
}

#[derive(Debug, Args)]
pub struct SignMessageArgs {
    #[arg(short, long)]
    msg: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: BenLnCliArgs = BenLnCliArgs::parse();

    let mut client = BenLnClient::connect("http://[::1]:3030").await?;

    match args.command {
        Command::Info => {
            let msg = GetNodeInfoRequest {};
            let response = client.get_node_info(msg).await?.into_inner();

            let json = serde_json::to_string(&response)?;

            println!("{}", json)
        }
        Command::NewAddress => {
            let msg = NewAddressRequest {};
            let response = client.new_address(msg).await?.into_inner();

            let json = serde_json::to_string(&response)?;

            println!("{}", json)
        }
        Command::SignMessage(msg) => {
            let msg = SignMessageRequest { msg: msg.msg };
            let response = client.sign_message(msg).await?.into_inner();

            let json = serde_json::to_string(&response)?;

            println!("{}", json)
        }
        Command::Stop => {
            let msg = StopRequest {};
            let _response = client.stop(msg).await?.into_inner();

            println!("Stopped node.")
        }
    }

    Ok(())
}
