use mind_map_rust::core::mind_map::MindMap;
use mind_map_rust::ui::renderer::Renderer;
use eframe::egui;

struct MyApp {
    mind_map: MindMap,
    renderer: Renderer,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
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

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Mind Map Tool Stackup",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    );
}
