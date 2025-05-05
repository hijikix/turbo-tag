#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct ListApp {
    counter: u32,
}

impl eframe::App for ListApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("list page");
        });
    }
}
