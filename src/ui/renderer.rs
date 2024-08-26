use crate::core::mind_map::MindMap;
use egui::{Context, Ui};

pub struct Renderer;

impl Renderer {
    pub fn new() -> Self {
        Renderer{

        }
    }
    pub fn render(&self, mind_map: &MindMap, ctx: &Context, ui: &mut Ui) {
         for node in &mind_map.nodes {
            ui.label(&node.title);
        }
    
        for connection in &mind_map.connections {
            ui.label(format!("Connection between {} and {}", connection.0, connection.1));
        }
    }
}
