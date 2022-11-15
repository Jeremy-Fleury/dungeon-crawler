use super::{edge::Edge, node::Node};
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Map {
    pub seed: u32,
    pub nodes: Vec<Box<Node>>,
}

impl Map {
    pub fn create() -> Map {
        let mut map = Map {
            seed: 1,
            nodes: Vec::new(),
        };

        for _ in 0..3 {
            let node: Box<Node> = Box::new(Node::create());
            map.nodes.push(node);
        }

        for _ in 0..5 {
            let to_index = thread_rng().gen_range(0..3);
            let from_index = thread_rng().gen_range(0..3);

            let to_node = Box::clone(&map.nodes[to_index]);
            let from_node = Box::clone(&map.nodes[from_index]);

            let edge = Edge::create(to_node, from_node);
            map.nodes[to_index].edges.push(edge);
        }

        return map;
    }

    pub fn print(&self) {
        for node in &self.nodes {
            println!("Node: {:?}", node.id);
            for edge in &node.edges {
                println!("\tEdge: {:?}", edge.from.id);
            }
        }
    }
}
