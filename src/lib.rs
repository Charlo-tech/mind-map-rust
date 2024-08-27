use eframe::egui;
use wasm_bindgen::prelude::*;
use mind_map_rust::core::mind_map::MindMap;
use mind_map_rust::ui::renderer::Renderer;

pub mod core;
pub mod storage;
pub mod ui;

struct MyApp {
    mind_map: MindMap,
    renderer: Renderer,
}

impl MyApp {
    fn new() -> Self {
        Self {
            mind_map: MindMap::new(),
            renderer: Renderer::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.renderer.render(&self.mind_map, ctx, ui);
        });
    }
}

#[wasm_bindgen]
pub fn start_app(canvas_id: &str) -> Result<(), JsValue> {
    eframe::start_web(canvas_id, Box::new(|_cc| Box::new(MyApp::new())))
}
