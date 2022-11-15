use uuid::Uuid;

use super::node::Node;

#[derive(Debug, Clone)]
pub struct Edge {
    pub id: Uuid,
    pub to: Box<Node>,
    pub from: Box<Node>,
}

impl Edge {
    pub fn create(to: Box<Node>, from: Box<Node>) -> Edge {
        Edge {
            id: Uuid::new_v4(),
            to,
            from,
        }
    }
}
