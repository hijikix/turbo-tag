use std::collections::HashMap;

use crate::anchor::{move_to_page, Anchor, AppPage, NextPage};

#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct ListApp {
    counter: u32,
}

impl AppPage for ListApp {
    fn on_move_page(&mut self, _params: &HashMap<String, String>) {}
}

impl eframe::App for ListApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("list page");
            if ui.button("move to detail").clicked() {
                let next_page = NextPage {
                    anchor: Anchor::Detail,
                    params: HashMap::from([
                        (String::from("key1"), String::from("value1")),
                        (String::from("key2"), String::from("value2")),
                    ]),
                };
                move_to_page(ctx, next_page)
            }
        });
    }
}
