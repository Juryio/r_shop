mod app;
mod core;
mod tools;
mod filters;

use app::RustyPsApp;

fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Rusty Photoshop Clone",
        native_options,
        Box::new(|cc| Box::new(RustyPsApp::new(cc))),
    )
}
