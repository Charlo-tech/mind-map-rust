use egui::{Response, Ui};
use crate::core::{mind_map::MindMap, node::Node};

pub fn handle_node_addition(ui: &mut Ui, mind_map: &mut MindMap, new_node_position: (f32, f32)) {
    if ui.button("Add Node").clicked() {
        let new_node = Node::new(mind_map.nodes.len() as u32, "New Node".into(), "".into(), new_node_position);
        mind_map.add_node(new_node);
    }
}

pub fn handle_node_connection(ui: &mut Ui, mind_map: &mut MindMap) {
    if mind_map.nodes.len() >= 2 {
        if ui.button("Connect Nodes").clicked() {
            let last_two_ids = (mind_map.nodes.len() as u32 - 2, mind_map.nodes.len() as u32 - 1);
            mind_map.connect_nodes(last_two_ids.0, last_two_ids.1);
        }
    }
}
