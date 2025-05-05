#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct DetailApp {
    counter: u32,
}

impl eframe::App for DetailApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("detail page");
        });
    }
}
