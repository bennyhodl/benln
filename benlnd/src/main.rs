use benln::ben_ln_server::{BenLn, BenLnServer};
use benln::{GetNodeInfoRequest, GetNodeInfoResponse};
use ldk_node::{bitcoin::Network, io::sqlite_store::SqliteStore, Builder, LogLevel, Node};
use tonic::{transport::Server, Request, Response, Status};

pub mod benln {
    tonic::include_proto!("benln");
}

struct BenLnNode {
    node: Node<SqliteStore>,
}

impl BenLnNode {
    fn new(node: Node<SqliteStore>) -> BenLnNode {
        BenLnNode { node }
    }
}

#[tonic::async_trait]
impl BenLn for BenLnNode {
    async fn get_node_info(
        &self,
        _request: Request<GetNodeInfoRequest>,
    ) -> Result<Response<GetNodeInfoResponse>, Status> {
        let node_id = self.node.node_id().to_string();
        let reply = GetNodeInfoResponse { node_id };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let builder = Builder::new()
        .set_network(Network::Signet)
        .set_esplora_server("https://mutinynet.com/api/v1".to_string())
        .set_storage_dir_path("../../.benln".to_string())
        .set_log_level(LogLevel::Debug)
        .build()
        .unwrap();

    builder.start().unwrap();

    println!("heyhowareya, ben!");

    let addr = "[::1]:3030".parse().unwrap();
    let node = BenLnNode::new(builder); 

    Server::builder()
        .add_service(BenLnServer::new(node))
        .serve(addr)
        .await?;

    Ok(())
}
