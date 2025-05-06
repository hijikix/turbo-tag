use std::collections::HashMap;

use crate::anchor::{move_to_page, Anchor, AppPage, NextPage};

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct ListApp {
    counter: u32,
}

impl AppPage for ListApp {
    fn on_move_page(&mut self, _ctx: &egui::Context, _params: &HashMap<String, String>) {}
}

impl eframe::App for ListApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("list page");
            if ui.button("move to detail").clicked() {
                let next_page = NextPage {
                    anchor: Anchor::Detail,
                    params: HashMap::from([(String::from("url"), String::from("https://fastly.picsum.photos/id/318/640/640.jpg?hmac=5cOMICOxIroPZAdiGA4-M50bvlhNo05T5t_FufYyRtI"))]),
                };
                move_to_page(ctx, next_page)
            }
        });
    }
}
