use core::f32;

use eframe::egui;

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
            
        let x_axis = egui::Vec2::new(f32::cos(angle), f32::sin(angle));
        let y_axis = egui::Vec2::new(-f32::sin(angle), f32::cos(angle));
        
        let origin = self.world_pos_to_viewport_pos(pos);
        let x_axis = self.world_pos_to_viewport_pos(pos + x_axis * 0.2);
        let y_axis = self.world_pos_to_viewport_pos(pos + y_axis * 0.2);

        painter.line_segment([origin, x_axis], egui::Stroke::new(3.0, egui::Color32::GREEN));
        painter.line_segment([origin, y_axis], egui::Stroke::new(3.0, egui::Color32::RED));
        painter.circle(
            origin,
            5.0,
            egui::Color32::LIGHT_BLUE,
            egui::Stroke::new(2.0, egui::Color32::BLUE)
        );
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
                    for segment in path_shape.segments() {
                        match segment {
                            Segment::Straight(line_segment) => {
                                let viewport_start = self.world_pos_to_viewport_pos(
                                    transform.pos + self.rotate(line_segment.start.to_vec2(), transform.angle)
                                );
                                let viewport_end = self.world_pos_to_viewport_pos(
                                    transform.pos + self.rotate(line_segment.end.to_vec2(), transform.angle)
                                );
                                painter.add(egui::Shape::line_segment([viewport_start, viewport_end], path_shape.stroke()));
                            },
                            Segment::Curve(bezier_curve) => {

                            }
                        }
                    }
                },
                Shape::Polygon(polygon_shape) => {
                    let mut vertices: Vec<egui::Pos2> = Vec::new();
                    for segment in polygon_shape.segments() {
                        match segment {
                            Segment::Straight(line_segment) => {
                                let viewport_vertex = self.world_pos_to_viewport_pos(
                                    transform.pos + self.rotate(line_segment.start.to_vec2(), transform.angle)
                                );
                                vertices.push(viewport_vertex);
                            },
                            Segment::Curve(bezier_curve) => {
                                let viewport_vertex = self.world_pos_to_viewport_pos(
                                    transform.pos + self.rotate(bezier_curve.start.to_vec2(), transform.angle)
                                );
                                vertices.push(viewport_vertex);
                            }
                        }
                    }
                    painter.add(egui::Shape::convex_polygon(vertices, polygon_shape.color(), polygon_shape.stroke()));
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

        let grid_spacing = 0.125 * (x_max - x_min);

        let left_line_index = (x_min / grid_spacing) as i32;
        let right_line_index = (x_max / grid_spacing) as i32;
        let bottom_line_index = (y_min / grid_spacing) as i32;
        let top_line_index = (y_max / grid_spacing) as i32;
        
        for j in left_line_index..=right_line_index {
            let x = j as f32 * grid_spacing;
            let top_pos = self.world_pos_to_viewport_pos(egui::Pos2::new(x, y_max));
            let bottom_pos = self.world_pos_to_viewport_pos(egui::Pos2::new(x, y_min));
            let stroke_width = match j {
                0 => 4.0,
                _ => 1.0
            };
            painter.line_segment([top_pos, bottom_pos], egui::Stroke::new(stroke_width, egui::Color32::LIGHT_GRAY));
        }
        
        for i in bottom_line_index..=top_line_index {
            let y = i as f32 * grid_spacing;
            let left_pos = self.world_pos_to_viewport_pos(egui::Pos2::new(x_min, y));
            let right_pos = self.world_pos_to_viewport_pos(egui::Pos2::new(x_max, y));
            let stroke_width = match i {
                0 => 4.0,
                _ => 1.0
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

    pub fn update(&mut self, ui: &mut egui::Ui) {
        self.rect = ui.clip_rect();
        let response = ui.allocate_response(self.rect.size(), egui::Sense::click_and_drag());

        if let Some(cur_mouse_pos) = ui.ctx().pointer_hover_pos() && self.rect.contains(cur_mouse_pos) {
            if let Some(old_mouse_pos) = self.mouse_pos && response.dragged_by(egui::PointerButton::Secondary) {
                let old_mouse_pos_world = self.viewport_pos_to_world_pos(old_mouse_pos);
                let cur_mouse_pos_world = self.viewport_pos_to_world_pos(cur_mouse_pos);
                self.camera_pos += old_mouse_pos_world - cur_mouse_pos_world;
            }
            self.mouse_pos = Some(cur_mouse_pos);
            let scroll = ui.ctx().input(|i| i.smooth_scroll_delta);
            //TODO scrollen
            self.camera_zoom -= scroll.y * 0.05;
        } else {
            self.mouse_pos = None;
        }
    }

     pub fn _get_camera_zoom(&self) -> f32 {
        self.camera_zoom
    }

    pub fn _get_camera_pos(&self) -> egui::Pos2 {
        self.camera_pos
    }

    pub fn get_rect(&self) -> egui::Rect {
        self.rect
    }

    pub fn _get_size(&self) -> egui::Vec2 {
        self.rect.size()
    }

    pub fn _get_mouse_pos(&self) -> Option<egui::Pos2> {
        self.mouse_pos
    }
}