use egui::{Response, Ui, Color32};
use crate::core::node::Node;

pub fn node_widget(ui: &mut Ui, node: &Node) -> Response {
    ui.group(|ui| {
        ui.label(&node.title);
        ui.horizontal(|ui| {
            ui.label(&node.content);
        });
    })
    .response
}

pub fn connection_widget(ui: &mut Ui, start_pos: (f32, f32), end_pos: (f32, f32)) -> Response {
    let color = Color32::from_rgb(0, 255, 0);
    ui.painter().line_segment([start_pos.into(), end_pos.into()], (2.0, color));
    ui.allocate_response(ui.available_size(), egui::Sense::hover())
}
