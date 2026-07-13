use eframe::egui;

mod enums;
mod app;
use app::App;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([1200.0, 800.0])
        .with_position(egui::Pos2::new(300.0, 100.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Renderer",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}
