use serde::{Serialize, Deserialize};
use crate::core::node::Node;

#[derive(Serialize, Deserialize)]
pub struct MindMap {
    pub nodes: Vec<Node>,
    pub connections: Vec<(u32, u32)>,
}

impl MindMap {
    pub fn new() -> Self {
        MindMap {
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn connect_nodes(&mut self, node1_id: u32, node2_id: u32) {
        self.connections.push((node1_id, node2_id));
    }

    pub fn remove_node(&mut self, node_id: u32) {
        self.nodes.retain(|node| node.id != node_id);
        self.connections.retain(|(a, b)| *a != node_id && *b != node_id);
    }

    pub fn find_node_by_title(&self, title: &str) -> Option<&Node> {
        self.nodes.iter().find(|&node| node.title == title)
    }
}
