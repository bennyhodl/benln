use ldk_node::{io::sqlite_store::SqliteStore, Node};
use std::sync::Arc;

pub struct BenLnNode {
    pub node: Arc<Node<SqliteStore>>,
}

impl BenLnNode {
    pub fn new(node: Arc<Node<SqliteStore>>) -> BenLnNode {
        BenLnNode { node }
    }
}
