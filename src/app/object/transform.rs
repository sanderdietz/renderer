use eframe::egui;

pub struct Transform {
    pub pos: egui::Pos2,
    pub scale: egui::Vec2,
    pub angle: f32
}

impl Transform {
    pub const ZERO: Self = Self {
        pos: egui::Pos2::ZERO,
        scale: egui::Vec2::ONE,
        angle: 0.0
    };
}

