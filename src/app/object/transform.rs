use eframe::egui;

pub struct Transform {
    pos: egui::Pos2,
    scale: egui::Vec2,
    angle: f32
}

impl Transform {
    pub const ZERO: Self = Self {
        pos: egui::Pos2::ZERO,
        scale: egui::Vec2::ONE,
        angle: 0.0
    };
    
    pub fn new(pos: egui::Pos2, scale: egui::Vec2, angle: f32) -> Self {
        // if scale.x == 0.0 || scale.y == 0.0 {
        //     panic!("scale cannot be zero");
        // }
        Self {
            pos: pos,
            scale: scale,
            angle: angle 
        }
    }

    pub fn pos(&self) -> egui::Pos2 {
        self.pos
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn scale(&self) -> egui::Vec2 {
        self.scale
    }
}

