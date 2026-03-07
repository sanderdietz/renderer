use core::fmt;

use eframe::egui;

pub enum Shape {
    Path(PathShape),
    Polygon(PolygonShape),
    Ellipse(EllipseShape)
}

#[derive(PartialEq, Debug)]
pub enum DefaultShape {
    LineSegment,
    Rectangle,
    RegularPolygon,
    Circle
}

impl fmt::Display for DefaultShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DefaultShape::LineSegment => write!(f, "Line segment"),
            DefaultShape::Rectangle => write!(f, "Rectangle"),
            DefaultShape::RegularPolygon => write!(f, "Regular polygon"),
            DefaultShape::Circle => write!(f, "Circle"),
        }
    }
}

pub struct DefaultValues {
    start: egui::Pos2,
    end: egui::Pos2,
    width: f32,
    height: f32,
    num_edges: u32,
    radius: f32
}

impl DefaultValues {
    pub fn new() -> Self {
        Self {
            start: egui::Pos2::new(-0.5, 0.0),
            end: egui::Pos2::new(0.5, 0.0),
            width: 1.5,
            height: 1.0,
            num_edges: 3,
            radius: 0.5
        }
    }

    pub fn start(&self) -> egui::Pos2 {
        self.start
    }

    pub fn end(&self) -> egui::Pos2 {
        self.end
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn num_edges(&self) -> u32 {
        self.num_edges
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn set_start(&mut self, pos: egui::Pos2) {
        self.start = pos;
    }

    pub fn set_end(&mut self, pos: egui::Pos2) {
        self.end = pos;
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    pub fn set_num_edges(&mut self, num_edges: u32) {
        self.num_edges = num_edges;
    }

    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

}

enum Segment {
    Straight(LineSegment),
    Curve(BezierCurve)
}

struct LineSegment {
    start: egui::Pos2,
    end: egui::Pos2
}

struct BezierCurve {
    start: egui::Pos2,
    start_handle: egui::Pos2,
    end_handle: egui::Pos2,
    end: egui::Pos2
}

pub struct PathShape {
    segments: Vec<Segment>,
    stroke: egui::Stroke
}

pub struct PolygonShape {
    segments: Vec<Segment>,
    stroke: egui::Stroke,
    color: egui::Color32
}

pub struct EllipseShape {
    width: f32,
    height: f32,
    stroke: egui::Stroke,
    color: egui::Color32
}

// pub struct Polygon {
//     vertices: Vec<egui::Pos2>
// }

// impl Polygon {
//     pub fn new(vertices: Vec<egui::Pos2>) -> Self {
//         Self { vertices: vertices }
//     }

//     pub fn add(&mut self, vertex: egui::Pos2) {
//         self.vertices.push(vertex);
//     }

//     pub fn get_vertices(&self) -> &Vec<egui::Pos2> {
//         &self.vertices
//     }
// }