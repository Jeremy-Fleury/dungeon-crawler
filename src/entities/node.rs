use uuid::Uuid;

use super::edge::Edge;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Uuid,
    pub edges: Vec<Edge>,
}

impl Node {
    pub fn create() -> Node {
        Node {
            id: Uuid::new_v4(),
            edges: Vec::new(),
        }
    }
}
