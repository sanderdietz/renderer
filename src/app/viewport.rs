use core::f32;

use eframe::egui::{self, CornerRadius};

use crate::app::Object;
use crate::app::object::transform::Transform;

use crate::app::object::geometry::*;

pub struct Viewport {
    rect: egui::Rect,
    mouse_pos: Option<egui::Pos2>,
    camera_pos: egui::Pos2,
    camera_zoom: f32
}

impl Viewport {
    pub fn new(rect: &egui::Rect) -> Self {
        Self {
            rect: *rect,
            mouse_pos: None,
            camera_pos: egui::Pos2::ZERO,
            camera_zoom: 1.0
        }
    }

    pub fn draw_gizmo(&self, painter: &egui::Painter, transform: &Transform) {
        let pos = transform.pos;
        let angle = transform.angle;
            
        let x_axis_vec = egui::Vec2::new(f32::cos(-angle), f32::sin(-angle));
        let y_axis_vec = egui::Vec2::new(f32::sin(-angle), -f32::cos(-angle));
        
        let origin = self.world_pos_to_viewport_pos(pos);
        let axis_length = 0.1 * self.rect.width();

        let x_axis = origin + x_axis_vec * axis_length;
        let y_axis = origin + y_axis_vec * axis_length;

        painter.line_segment([origin, x_axis], egui::Stroke::new(1.0_f32, egui::Color32::GREEN));
        painter.add(egui::Shape::convex_polygon(vec![
                x_axis,
                x_axis - x_axis_vec * 0.1 * axis_length + y_axis_vec * 0.05 * axis_length,
                x_axis - x_axis_vec * 0.1 * axis_length - y_axis_vec * 0.05 * axis_length
            ], 
            egui::Color32::GREEN, 
            egui::Stroke::NONE
        ));
        painter.line_segment([origin, y_axis], egui::Stroke::new(1.0_f32, egui::Color32::RED));
        painter.add(egui::Shape::convex_polygon(vec![
                y_axis,
                y_axis - y_axis_vec * 0.1 * axis_length + x_axis_vec * 0.05 * axis_length,
                y_axis - y_axis_vec * 0.1 * axis_length - x_axis_vec * 0.05 * axis_length
            ], 
            egui::Color32::RED, 
            egui::Stroke::NONE
        ));

        painter.add(egui::Shape::convex_polygon(vec![
                origin,
                origin + y_axis_vec * 0.2 * axis_length,
                origin + y_axis_vec * 0.2 * axis_length + x_axis_vec * 0.2 * axis_length,
                origin + x_axis_vec * 0.2 * axis_length
            ],
            egui::Color32::from_rgba_unmultiplied(0, 0, 255, 100),
            egui::Stroke::new(1.0_f32, egui::Color32::BLUE)
        ));
    }

    fn rotate(&self, vec: egui::Vec2, angle: f32) -> egui::Vec2 {
        let x = f32::cos(angle) * vec.x - f32::sin(angle) * vec.y;
        let y = f32::sin(angle) * vec.x + f32::cos(angle) * vec.y;
        egui::Vec2::new(x, y)
    }

    pub fn draw_object(&self, painter: &egui::Painter, object: &Object, _selected: bool) {
        let transform = object.transform();
        
        if let Some(shape) = object.shape() {
            match shape {
                Shape::Path(path_shape) => {
                    self.draw_path(painter, path_shape, transform);
                },
                Shape::Polygon(polygon_shape) => {
                    // painter.add(egui::Shape::convex_polygon(
                    //     self.vertices_to_viewport(polygon_shape.vertices(), transform), 
                    //     polygon_shape.color(), 
                    //     polygon_shape.stroke())
                    // );
                    let vertices = polygon_shape.vertices();
                    let n = vertices.len();
                    let mut segments = Vec::new();
                    for i in 0..n {
                        match i {
                            0 => {
                                let line_segment = LineSegment{ start: vertices[n - 1], end: vertices[0] };
                                segments.push(Segment::Straight(line_segment));
                            },
                            i => {
                                let line_segment = LineSegment{ start: vertices[i - 1], end: vertices[i] };
                                segments.push(Segment::Straight(line_segment));
                            }
                        }
                    }
                    let path = PathShape::new(segments, polygon_shape.stroke());

                    self.draw_polygon(
                        painter,
                        &mut self.vertices_to_viewport(vertices, transform),
                        polygon_shape.color()
                    );
                    self.draw_path(
                        painter,
                        &path,
                        transform
                    );
                },
                Shape::Ellipse(ellipse_shape) => {
                    let center = self.world_pos_to_viewport_pos(transform.pos);
                    let a = 0.5 * ellipse_shape.width() * self.rect.size().x * self.camera_zoom;
                    let b = 0.5 * ellipse_shape.height() * self.rect.size().x * self.camera_zoom;
                    let radius = egui::Vec2::new(a, b);
                    painter.add(egui::Shape::ellipse_filled(center, radius, ellipse_shape.color()));
                    painter.add(egui::Shape::ellipse_stroke(center, radius, ellipse_shape.stroke()));
                }
            }
        }
    }

    pub fn draw_grid(&self, painter: &egui::Painter) {
        let x_min = self.camera_pos.x - self.camera_zoom;
        let x_max = self.camera_pos.x + self.camera_zoom;
        let y_min = self.camera_pos.y - self.camera_zoom * self.rect.height() / self.rect.width();
        let y_max = self.camera_pos.y + self.camera_zoom * self.rect.height() / self.rect.width();

        let grid_spacing = 0.25 * f32::powf(2.0, (f32::log(self.camera_zoom, 2.0) as i32) as f32);

        let left_line_index = (x_min / grid_spacing) as i32;
        let right_line_index = (x_max / grid_spacing) as i32;
        let bottom_line_index = (y_min / grid_spacing) as i32;
        let top_line_index = (y_max / grid_spacing) as i32;
        
        for j in left_line_index..=right_line_index {
            let x = j as f32 * grid_spacing;
            let top_pos = self.world_pos_to_viewport_pos(egui::Pos2::new(x, y_max));
            let bottom_pos = self.world_pos_to_viewport_pos(egui::Pos2::new(x, y_min));
            let stroke_width = match j {
                0 => 0.8_f32,
                _ => 0.5_f32
            };
            painter.line_segment([top_pos, bottom_pos], egui::Stroke::new(stroke_width, egui::Color32::LIGHT_GRAY));
        }
        
        for i in bottom_line_index..=top_line_index {
            let y = i as f32 * grid_spacing;
            let left_pos = self.world_pos_to_viewport_pos(egui::Pos2::new(x_min, y));
            let right_pos = self.world_pos_to_viewport_pos(egui::Pos2::new(x_max, y));
            let stroke_width = match i {
                0 => 0.8_f32,
                _ => 0.5_f32
            };
            painter.line_segment([left_pos, right_pos], egui::Stroke::new(stroke_width, egui::Color32::LIGHT_GRAY));   
        }
    }

    pub fn world_pos_to_viewport_pos(&self, world_pos: egui::Pos2) -> egui::Pos2 {
        let size = self.rect.size();
        let direction = world_pos - self.camera_pos;
        let scaled = 0.5 * size.x / self.camera_zoom * direction;
        let inverted = scaled * egui::Vec2::new(1.0, -1.0);
        self.rect.center() + inverted
    }

    pub fn viewport_pos_to_world_pos(&self, viewport_pos: egui::Pos2) -> egui::Pos2 {
        let size = self.rect.size();
        let direction = viewport_pos - self.rect.center();
        let scaled = 2.0 * self.camera_zoom / size.x * direction;
        let inverted = scaled * egui::Vec2::new(1.0, -1.0);
        self.camera_pos + inverted 
    }

    pub fn vertices_to_viewport(&self, vertices: &Vec<egui::Pos2>, transform: &Transform) -> Vec<egui::Pos2> {
        let mut viewport_vertices = Vec::new();
        for vertex in vertices {
            let viewport_vertex = self.world_pos_to_viewport_pos(
                transform.pos + self.rotate(vertex.to_vec2(), transform.angle)
            );
            viewport_vertices.push(viewport_vertex);
        }
        viewport_vertices
    }

    pub fn update(&mut self, ui: &mut egui::Ui) {
        self.rect = ui.clip_rect();
        let response = ui.allocate_response(self.rect.size(), egui::Sense::click_and_drag());

        // if response.clicked(){
        //     match self.hovered_object_id {
        //         Some(id) => self.selected_object_id = Some(id),
        //         None => self.selected_object_id = None
        //     }
        // }

        if let Some(cur_mouse_pos) = ui.ctx().pointer_hover_pos() && self.rect.contains(cur_mouse_pos) {
            if let Some(old_mouse_pos) = self.mouse_pos && response.dragged_by(egui::PointerButton::Secondary) {
                let old_mouse_pos_world = self.viewport_pos_to_world_pos(old_mouse_pos);
                let cur_mouse_pos_world = self.viewport_pos_to_world_pos(cur_mouse_pos);
                self.camera_pos += old_mouse_pos_world - cur_mouse_pos_world;
            }
            self.mouse_pos = Some(cur_mouse_pos);
            let scroll = ui.ctx().input(|i| i.smooth_scroll_delta);
            // A single scroll is 40, i.e. 1 / 0.025, pixels
            self.camera_zoom *= f32::powf(2.0, -0.003125 * scroll.y);
        } else {
            self.mouse_pos = None;
        }
    }

    pub fn check_hitbox(&self, object: &Object) -> bool {
        match self.mouse_pos {
            Some(mouse_pos) => {
                let transform = object.transform();
                match object.shape() {
                    Some(Shape::Path(path_shape)) => false,
                    Some(Shape::Polygon(polygon_shape)) => {
                        let vertices = self.vertices_to_viewport(polygon_shape.vertices(), transform);
                        self.pos_in_polygon(mouse_pos, &vertices)
                    },
                    Some(Shape::Ellipse(ellipse_shape)) => false,
                    None => {
                        let transform_pos = self.world_pos_to_viewport_pos(transform.pos);
                        transform_pos.distance_sq(mouse_pos) < 6.0 * 6.0
                    },
                }
            }
            None => false
        }
    }

    pub fn camera_zoom(&self) -> f32 {
        self.camera_zoom
    }

    pub fn _get_camera_pos(&self) -> egui::Pos2 {
        self.camera_pos
    }

    pub fn get_rect(&self) -> egui::Rect {
        self.rect
    }

    pub fn size(&self) -> egui::Vec2 {
        self.rect.size()
    }

    pub fn _get_mouse_pos(&self) -> Option<egui::Pos2> {
        self.mouse_pos
    }
    
    fn pos_in_polygon(&self, pos: egui::Pos2, vertices: &Vec<egui::Pos2>) -> bool {
        let mut num_intersections = 0;
        let n = vertices.len();
        for i in 0..n {
            if i == 0 {
                if intersect(vertices[0], vertices[n - 1], pos, pos + egui::Vec2::UP * self.rect.size().y) {
                num_intersections += 1;
                }
            }
            else {
                if intersect(vertices[i], vertices[i - 1], pos, pos + egui::Vec2::UP * self.rect.size().y) {
                    num_intersections += 1;
                }
            }
        }
        num_intersections % 2 == 1
    }

    pub fn triangle_contains_point(&self, vertices: &Vec<egui::Pos2>, i: usize) -> bool {
        let n = vertices.len();
        for j in 0..n {
            let diff = (n + j - i) % n;
            if diff > 1 && diff < n - 1 {
                if self.pos_in_polygon(vertices[j], &triangle_from_index(vertices, i))
                {
                    return true;
                }
            }
        }
        false
    }

    
    pub fn draw_polygon(&self, painter: &egui::Painter, vertices: &mut Vec<egui::Pos2>, color: egui::Color32) {
        while vertices.len() > 3 {
            let n = vertices.len();
            for i in 0..n {
                let triangle = triangle_from_index(vertices, i);
                if get_angle(&triangle) < f32::consts::PI && !self.triangle_contains_point(vertices, i) {
                    // i is an ear
                    painter.add(egui::Shape::convex_polygon(
                        triangle, 
                        color, 
                        egui::Stroke::new(1.0_f32, color)
                    ));
                    vertices.remove(i);
                    break;
                }
            }
        }
        painter.add(egui::Shape::convex_polygon(
            vertices.clone(),
            color, 
            egui::Stroke::new(1.0_f32, color)
        ));
    }

    pub fn draw_path(&self, painter: &egui::Painter, path: &PathShape, transform: &Transform) {
        for segment in path.segments() {
            match segment {
                Segment::Straight(line_segment) => {
                    let endpoints = self.vertices_to_viewport(
                        &vec![line_segment.start, line_segment.end],
                        transform
                    );
                    painter.add(egui::Shape::line_segment([endpoints[0], endpoints[1]], path.stroke()));
                },
                Segment::Curve(bezier_curve) => {

                }
            }
        }
    }
}

fn ccw(a: egui::Pos2, b: egui::Pos2, c: egui::Pos2) -> bool {
    (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
}

fn intersect(a: egui::Pos2, b: egui::Pos2, c: egui::Pos2, d: egui::Pos2) -> bool {
    ccw(a, c, d) != ccw(b, c, d) && ccw(a, b, c) != ccw(a, b, d)
}

fn get_angle(triangle: &Vec<egui::Pos2>) -> f32 {
    let v1 = triangle[1] - triangle[0];
    let v2 = triangle[2] - triangle[0];
    let mut angle = v1.angle() - v2.angle();
    while angle > f32::consts::TAU {
        angle -= f32::consts::TAU;
    }
    while angle < 0.0 {
        angle += f32::consts::TAU;
    }
    angle
}

pub fn triangle_from_index(vertices: &Vec<egui::Pos2>, i: usize) -> Vec<egui::Pos2> {
    let n = vertices.len();
    match i {
        0 => vec![vertices[n - 1], vertices[0], vertices[1]],
        i if i == n - 1 => vec![vertices[n - 2], vertices[n - 1], vertices[0]],
        i => vec![vertices[i - 1], vertices[i], vertices[i + 1]]
    }
}
