use crate::anchor::AppPage;
use std::collections::HashMap;

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct DetailApp {
    counter: u32,
}

impl AppPage for DetailApp {
    fn on_move_page(&mut self, params: &HashMap<String, String>) {
        println!("DetailApp on_move_page");
        println!("{:?}", params);
    }
}

impl eframe::App for DetailApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("detail page");
        });
    }
}
