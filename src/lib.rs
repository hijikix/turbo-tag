#![warn(clippy::all, rust_2018_idioms)]

mod anchor;
mod app;
mod apps;
pub use app::TemplateApp;

// ----------------------------------------------------------------------------

/// Detect narrow screens. This is used to show a simpler UI on mobile devices,
/// especially for the web demo at <https://egui.rs>.
pub fn is_mobile(ctx: &egui::Context) -> bool {
    let screen_size = ctx.screen_rect().size();
    screen_size.x < 550.0
}
