use eframe::egui;

// pub enum Shape {
//     Polygon
// }

pub struct Polygon {
    vertices: Vec<egui::Pos2>
}

impl Polygon {
    pub fn new(vertices: Vec<egui::Pos2>) -> Self {
        Self { vertices: vertices }
    }

    pub fn add(&mut self, vertex: egui::Pos2) {
        self.vertices.push(vertex);
    }

    pub fn get_vertices(&self) -> &Vec<egui::Pos2> {
        &self.vertices
    }
}