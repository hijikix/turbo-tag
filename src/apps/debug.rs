use std::collections::HashMap;

use crate::anchor::AppPage;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct DebugApp {
    counter: u32,
}

impl AppPage for DebugApp {
    fn on_move_page(&mut self, _ctx: &egui::Context, _params: &HashMap<String, String>) {}
}

impl eframe::App for DebugApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("debug page");
        });
    }
}
