use benlnproto::benln::{
    ben_ln_client::BenLnClient, AddPeerRequest, GetNodeInfoRequest, GetTotalOnchainBalanceRequest,
    ListPeersRequest, NewAddressRequest, SignMessageRequest, StopRequest, SyncWalletRequest,
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
    /// Get on-chain balance.
    WalletBalance,
    /// Sign a message with the node keys.
    SignMessage(SignMessageArgs),
    /// Sync the on-chain wallet.
    SyncWallet,
    /// List node peers.
    ListPeers,
    /// Connect to a node on the network.
    AddPeer(AddPeerArgs),
    /// Stop the node.
    Stop,
}

#[derive(Debug, Args)]
pub struct AddPeerArgs {
    #[arg(short, long)]
    pubkey: String,
    #[arg(short, long)]
    uri: String,
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
        Command::WalletBalance => {
            let msg = GetTotalOnchainBalanceRequest {};
            let response = client.get_total_onchain_balance(msg).await?.into_inner();

            let json = serde_json::to_string(&response)?;

            println!("{}", json)
        }
        Command::SignMessage(msg) => {
            let msg = SignMessageRequest { msg: msg.msg };
            let response = client.sign_message(msg).await?.into_inner();

            let json = serde_json::to_string(&response)?;

            println!("{}", json)
        }
        Command::SyncWallet => {
            let msg = SyncWalletRequest {};
            let _response = client.sync_wallet(msg).await?.into_inner();

            println!("Synced wallet.")
        }
        Command::ListPeers => {
            let msg = ListPeersRequest {};

            let response = client.list_peers(msg).await?.into_inner();

            let json = serde_json::to_string(&response)?;

            println!("{}", json)
        }
        Command::AddPeer(args) => {
            let msg = AddPeerRequest {
                pubkey: args.pubkey,
                uri: args.uri,
                persist: false,
            };

            let _response = client.add_peer(msg).await?.into_inner();

            println!("Added peer.")
        }
        Command::Stop => {
            let msg = StopRequest {};
            let _response = client.stop(msg).await?.into_inner();

            println!("Stopped node.")
        }
    }

    Ok(())
}
