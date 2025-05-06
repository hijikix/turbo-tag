use egui::{FontData, FontDefinitions, FontFamily};

use crate::{
    anchor::{get_next_page, Anchor, AppWithPage},
    apps::{DebugApp, DetailApp, ListApp},
};

/// The state that we persist (serialize).
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct State {
    list: ListApp,
    detail: DetailApp,
    debug: DebugApp,

    selected_anchor: Anchor,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    pub state: State,

    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            state: State::default(),
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        ////////////////////////////////
        // setup fonts
        ////////////////////////////////
        let mut fonts = FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters):
        fonts.font_data.insert(
            "my_font".to_owned(),
            std::sync::Arc::new(
                // .ttf and .otf supported
                FontData::from_static(include_bytes!("../fonts/NotoSansJP-Regular.ttf")),
            ),
        );

        // Put my font first (highest priority):
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_font".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .push("my_font".to_owned());

        cc.egui_ctx.set_fonts(fonts);
        ////////////////////////////////
        // setup fonts end
        ////////////////////////////////

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl TemplateApp {
    pub fn apps_iter_mut(
        &mut self,
    ) -> impl Iterator<Item = (&'static str, Anchor, &mut dyn AppWithPage)> {
        let vec = vec![
            (
                "✨ List",
                Anchor::List,
                &mut self.state.list as &mut dyn AppWithPage,
            ),
            (
                "🕑 Detail",
                Anchor::Detail,
                &mut self.state.detail as &mut dyn AppWithPage,
            ),
            (
                "⬇ Debug",
                Anchor::Debug,
                &mut self.state.debug as &mut dyn AppWithPage,
            ),
        ];

        vec.into_iter()
    }

    fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let selected_anchor = self.state.selected_anchor;
        for (_name, anchor, app) in self.apps_iter_mut() {
            if anchor == selected_anchor || ctx.memory(|mem| mem.everything_is_visible()) {
                app.update(ctx, frame);
            }
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_switch(ui);

                ui.separator();

                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);

                    ui.separator();
                }

                let mut selected_anchor = self.state.selected_anchor;
                for (name, anchor, _app) in self.apps_iter_mut() {
                    if ui
                        .selectable_label(selected_anchor == anchor, name)
                        .clicked()
                    {
                        selected_anchor = anchor;
                        if frame.is_web() {
                            ui.ctx()
                                .open_url(egui::OpenUrl::same_tab(format!("#{anchor}")));
                        }
                    }
                }
                self.state.selected_anchor = selected_anchor;

                // move page if move_to_page method called
                if let Some(next_page) = get_next_page(ctx) {
                    for (_name, anchor, app) in self.apps_iter_mut() {
                        if anchor == next_page.anchor
                            || ctx.memory(|mem| mem.everything_is_visible())
                        {
                            // initialize next_page
                            app.on_move_page(ctx, &next_page.params);
                        }
                    }
                    self.state.selected_anchor = next_page.anchor;
                }
            });
        });

        self.show_selected_app(ctx, frame);
    }
}
