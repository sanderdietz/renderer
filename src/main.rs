use eframe::egui;

const INIT_WIDTH: usize = 1200;
const INIT_HEIGHT: usize = 800;

mod app;
use app::App;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([INIT_WIDTH as f32, INIT_HEIGHT as f32])
        .with_position(egui::Pos2::new(300.0, 100.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Renderer",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}
