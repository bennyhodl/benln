use ldk_node::{io::sqlite_store::SqliteStore, Node};

pub struct BenLnNode {
    pub node: Node<SqliteStore>,
}

impl BenLnNode {
    pub fn new(node: Node<SqliteStore>) -> BenLnNode {
        BenLnNode { node }
    }
}
