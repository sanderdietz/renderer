use eframe::egui;

pub struct Transform {
    pos: egui::Pos2,
    angle: f32
}

impl Transform {
    pub const ZERO: Self = Self { pos: egui::Pos2::ZERO, angle: 0.0 };
    
    pub fn new(pos: egui::Pos2, angle: f32) -> Self {
        Self { pos: pos, angle: angle }
    }

    pub fn _set_pos(&mut self, pos: egui::Pos2) {
        self.pos = pos;
    }

    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    pub fn get_pos(&self) -> egui::Pos2 {
        self.pos
    }
}

