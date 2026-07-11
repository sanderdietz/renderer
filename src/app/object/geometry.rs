use core::{f32, fmt};

use eframe::egui;

pub enum Shape {
    Path(PathShape),
    Polygon(PolygonShape),
    Ellipse(EllipseShape)
}

impl Shape {
    pub fn new(default_values: &DefaultValues) -> Shape {
        match default_values.shape() {
            DefaultShape::LineSegment => {
                let line_segment = LineSegment{ start: default_values.start(), end: default_values.end() };
                let segment = Segment::Straight(line_segment);
                let path_shape = PathShape{ segments: vec![segment], stroke: default_values.stroke() };
                Shape::Path(path_shape)
            },
            DefaultShape::Rectangle => {
                let width = default_values.width();
                let height = default_values.height();
                
                let bottom_left = egui::Pos2::new(-0.5 * width, -0.5 * height);
                let bottom_right = egui::Pos2::new(0.5 * width, -0.5 * height);
                let top_right = egui::Pos2::new(0.5 * width, 0.5 * height);
                let top_left = egui::Pos2::new(-0.5 * width, 0.5 * height);

                let polygon_shape = PolygonShape {
                    vertices: vec![bottom_left, bottom_right, top_right, top_left],
                    color: default_values.color(),
                    stroke: default_values.stroke()
                };
                Shape::Polygon(polygon_shape)
            },
            DefaultShape::RegularPolygon => {
                let mut vertices = Vec::new();
                let n = default_values.num_edges();
                let angle = f32::consts::TAU / n as f32;
                let radius = default_values.radius();
                for i in 0..n {
                    let x = radius * f32::cos(i as f32 * angle);
                    let y = radius * f32::sin(i as f32 * angle);
                    let vertex = egui::Pos2::new(x, y);
                    vertices.push(vertex);
                }

                let polygon_shape = PolygonShape {
                    vertices: vertices,
                    color: default_values.color(),
                    stroke: default_values.stroke()
                };
                Shape::Polygon(polygon_shape)
            },
            DefaultShape::Circle => {
                let ellipse_shape = EllipseShape {
                    width: default_values.radius(),
                    height: default_values.radius(),
                    stroke: default_values.stroke(),
                    color: default_values.color()
                };
                Shape::Ellipse(ellipse_shape)
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
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
    shape: DefaultShape,
    start: egui::Pos2,
    end: egui::Pos2,
    width: f32,
    height: f32,
    num_edges: u32,
    radius: f32,
    stroke: egui::Stroke,
    color: egui::Color32
}

impl DefaultValues {
    pub fn new() -> Self {
        Self {
            shape: DefaultShape::LineSegment,
            start: egui::Pos2::new(-0.5, 0.0),
            end: egui::Pos2::new(0.5, 0.0),
            width: 1.5,
            height: 1.0,
            num_edges: 3,
            radius: 0.5,
            stroke: egui::Stroke::new(3.0, egui::Color32::DARK_GREEN),
            color: egui::Color32::LIGHT_GREEN
        }
    }

    pub fn shape(&self) -> DefaultShape {
        self.shape
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

    pub fn stroke(&self) -> egui::Stroke {
        self.stroke
    }

    pub fn color(&self) -> egui::Color32 {
        self.color
    }

    pub fn set_shape(&mut self, shape: DefaultShape) {
        self.shape = shape;
    }

    pub fn set_start(&mut self, pos: egui::Pos2) {
        self.start = pos;
    }

    pub fn set_end(&mut self, pos: egui::Pos2) {
        self.end = pos;
    }

    pub fn set_width(&mut self, width: f32) {
        if width < 0.0 {
            panic!("Width must be nonnegative")
        }
        self.width = width;
    }

    pub fn set_height(&mut self, height: f32) {
        if height < 0.0 {
            panic!("Height must be nonnegative")
        }
        self.height = height;
    }

    pub fn set_num_edges(&mut self, num_edges: u32) {
        if num_edges < 3 {
            panic!("Polygon must have at least 3 edges")
        }
        self.num_edges = num_edges;
    }

    pub fn set_radius(&mut self, radius: f32) {
        if radius < 0.0 {
            panic!("Radius must be nonnegative")
        }
        self.radius = radius;
    }

    pub fn set_stroke(&mut self, stroke: egui::Stroke) {
        if stroke.width < 0.0 {
            panic!("Stroke width must be nonnegative")
        }
        self.stroke = stroke;
    }

    pub fn set_color(&mut self, color: egui::Color32) {
        self.color = color;
    }

}

pub enum Segment {
    Straight(LineSegment),
    Curve(BezierCurve)
}

pub struct LineSegment {
    pub start: egui::Pos2,
    pub end: egui::Pos2
}

pub struct BezierCurve {
    pub start: egui::Pos2,
    pub start_handle: egui::Pos2,
    pub end_handle: egui::Pos2,
    pub end: egui::Pos2
}

pub struct PathShape {
    segments: Vec<Segment>,
    stroke: egui::Stroke
}

impl PathShape {
    pub fn new(segments: Vec<Segment>, stroke: egui::Stroke) -> Self {
        Self {
            segments, stroke
        }
    }

    pub fn segments(&self) -> &Vec<Segment> {
        &self.segments
    }

    pub fn segments_mut(&mut self) -> &mut Vec<Segment> {
        &mut self.segments
    }

    pub fn stroke(&self) -> egui::Stroke {
        self.stroke
    }

    pub fn set_stroke(&mut self, stroke: egui::Stroke) {
        if stroke.width < 0.0 {
            panic!("Stroke width must be nonnegative")
        }
        self.stroke = stroke;
    }
}

pub struct PolygonShape {
    vertices: Vec<egui::Pos2>,
    color: egui::Color32,
    stroke: egui::Stroke
}

impl PolygonShape {
    pub fn vertices(&self) -> &Vec<egui::Pos2> {
        &self.vertices
    }
    
    pub fn vertices_mut(&mut self) -> &mut Vec<egui::Pos2> {
        &mut self.vertices
    }
    
    pub fn color(&self) -> egui::Color32 {
        self.color
    }

    pub fn stroke(&self) -> egui::Stroke {
        self.stroke
    }

    pub fn set_color(&mut self, color: egui::Color32) {
        self.color = color;
    }

    pub fn set_stroke(&mut self, stroke: egui::Stroke) {
        if stroke.width < 0.0 {
            panic!("Stroke width must be nonnegative")
        }
        self.stroke = stroke;
    }
}

pub struct EllipseShape {
    width: f32,
    height: f32,
    color: egui::Color32,
    stroke: egui::Stroke
}

impl EllipseShape {
    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn color(&self) -> egui::Color32 {
        self.color
    }

    pub fn stroke(&self) -> egui::Stroke {
        self.stroke
    }

    pub fn set_width(&mut self, width: f32) {
        if width < 0.0 {
            panic!("Width must be nonnegative")
        }
        self.width = width;
    }

    pub fn set_height(&mut self, height: f32) {
        if height < 0.0 {
            panic!("Height must be nonnegative")
        }
        self.height = height;
    }
    
    pub fn set_color(&mut self, color: egui::Color32) {
        self.color = color;
    }

    pub fn set_stroke(&mut self, stroke: egui::Stroke) {
        if stroke.width < 0.0 {
            panic!("Stroke width must be nonnegative")
        }
        self.stroke = stroke;
    }
}