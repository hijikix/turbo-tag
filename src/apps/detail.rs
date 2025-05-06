use crate::anchor::AppPage;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct DetailApp {
    url: String,
}

impl AppPage for DetailApp {
    fn on_move_page(&mut self, _ctx: &egui::Context, params: &HashMap<String, String>) {
        self.url = params.get("url").unwrap().to_string();
    }
}

impl eframe::App for DetailApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.image(&self.url);
        });
    }
}
