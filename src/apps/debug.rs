#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct DebugApp {
    counter: u32,
}

impl eframe::App for DebugApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("debug page");
        });
    }
}
