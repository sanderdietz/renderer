use eframe::egui;

pub mod viewport;
use viewport::Viewport;
pub mod object;
use object::Object;
use object::geometry::*;

pub struct App {
    viewport: Option<Viewport>,
    objects: Vec<Object>,
    selected_object_id: Option<usize>,
    hovered_object_id: Option<usize>,
    draw_grid: bool,
    new_object: bool,
    new_name: String
}

impl App {
    pub fn new() -> Self {
        Self {
            viewport: None,
            objects: Vec::new(),
            selected_object_id: None,
            hovered_object_id: None,
            draw_grid: true,
            new_object: false,
            new_name: String::from("new object")
        }
    }

    fn object_name_exists(&self, name: &String) -> bool {
        for object in &self.objects {
            if object.name() == name {
                return true;
            }
        }
        false
    }

    fn get_selected_mut(&mut self) -> Option<&mut Object> {
        for object in &mut self.objects {
            match self.selected_object_id {
                Some(id) => {
                    if id == object.id() {
                        return Some(object);
                    }
                }
                None => {}
            }
        }
        None
    }

    fn get_selected(&self) -> Option<&Object> {
        for object in self.objects.iter() {
            if self.is_selected(object) {
                return Some(object);
            }
        }
        None
    }

    fn is_selected(&self, object: &Object) -> bool {
        match self.selected_object_id {
            Some(id) => id == object.id(),
            None => false
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left panel").show(ctx, |ui| {

            // Object list
            ui.label("Objects");
            for object in self.objects.iter() {
                let selected = self.is_selected(object);
                let response =  ui.selectable_label(selected, format!("{}", object.name()));
                if response.clicked()
                {
                    self.selected_object_id = match self.selected_object_id {
                        Some(id) => {
                            if id == object.id() {
                                None
                            } else {
                                Some(object.id())
                            }
                        },
                        None => {
                            Some(object.id())
                        }
                    }
                }
            }

            // Add object
            if self.new_object {
                ui.text_edit_singleline(&mut self.new_name);
                let name_available = !self.object_name_exists(&self.new_name);
                ui.horizontal(|ui| {
                    if ui.add_enabled(
                        name_available,
                        egui::Button::new("Create object")
                    ).clicked() {
                        let object = Object::new(self.new_name.clone());
                        let id = object.id();
                        self.objects.push(object);
                        self.selected_object_id = Some(id);
                        self.new_name = String::from("new object");
                        self.new_object = false;
                    }
                    if !name_available {
                        ui.label("Name already exists");
                    } 
                });
                if ui.button("Cancel").clicked() {
                    self.new_name = String::from("new object");
                    self.new_object = false;
                }
            } else {
                if ui.button("Add object").clicked() {
                    self.new_object = true;
                }
            }
        });

        egui::SidePanel::right("right panel").show(ctx, |ui| {
            ui.checkbox(&mut self.draw_grid, "Grid");

            if let Some(object) = self.get_selected_mut() {
                ui.label(format!("{}", object.name()));
                
                // Display transform
                egui::CollapsingHeader::new("Transform")
                    .default_open(true)
                    .show(ui, |ui| {
                        let transform = object.transform_mut();
                        ui.label("Position");
                        ui.horizontal(|ui| {
                            ui.label("x:");
                            ui.add(egui::DragValue::new(&mut transform.pos.x).speed(0.01));
                            ui.label("y:");
                            ui.add(egui::DragValue::new(&mut transform.pos.y).speed(0.01));
                        });
                        ui.label("Scale");
                        ui.horizontal(|ui| {
                            ui.label("x:");
                            ui.add(egui::DragValue::new(&mut transform.scale.x).speed(0.01));
                            ui.label("y:");
                            ui.add(egui::DragValue::new(&mut transform.scale.y).speed(0.01));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Angle");
                            ui.add(egui::DragValue::new(&mut transform.angle).speed(0.01));
                        });
                    });
                
                // Display shape
                egui::CollapsingHeader::new("Shape")
                    .default_open(object.shape().is_some())
                    .show(ui, |ui| {
                        if let Some(shape) = object.shape_mut() {
                            match shape {
                                // Path
                                Shape::Path(path_shape) => {
                                    let mut stroke = path_shape.stroke();
                                    let segments = path_shape.segments_mut();
                                    let n = segments.len();
                                    for (i, segment) in segments.iter_mut().enumerate() {
                                        match segment {
                                            Segment::Straight(line_segment) => {
                                                ui.horizontal(|ui| {
                                                    ui.label("x:");
                                                    ui.add(egui::DragValue::new(&mut line_segment.start.x).speed(0.01));
                                                    ui.label("y:");
                                                    ui.add(egui::DragValue::new(&mut line_segment.start.y).speed(0.01));
                                                });
                                                if i == n - 1 {
                                                    ui.horizontal(|ui| {
                                                        ui.label("x:");
                                                        ui.add(egui::DragValue::new(&mut line_segment.end.x).speed(0.01));
                                                        ui.label("y:");
                                                        ui.add(egui::DragValue::new(&mut line_segment.end.y).speed(0.01));
                                                    });
                                                }
                                            },
                                            Segment::Curve(bezier_curve) => {
                                                let mut start = bezier_curve.start;
                                            }
                                        }
                                    }
                                    ui.label("Stroke:");
                                    ui.horizontal(|ui| {
                                        ui.label("width:");
                                        ui.add(egui::DragValue::new(&mut stroke.width).speed(0.01).range(0.0..=f32::INFINITY));
                                        ui.label("color:");
                                        egui::color_picker::color_edit_button_srgba(ui, &mut stroke.color, egui::color_picker::Alpha::Opaque);
                                    });
                                    path_shape.set_stroke(stroke);
                                },
                                // Polygon
                                Shape::Polygon(polygon_shape) => {
                                    let mut color = polygon_shape.color();
                                    let mut stroke = polygon_shape.stroke();
                                    let vertices = polygon_shape.vertices_mut();
                                    for vertex in vertices.iter_mut() {
                                        ui.horizontal(|ui| {
                                            ui.label("x:");
                                            ui.add(egui::DragValue::new(&mut vertex.x).speed(0.01));
                                            ui.label("y:");
                                            ui.add(egui::DragValue::new(&mut vertex.y).speed(0.01));
                                        });
                                    }
                                    ui.label("Color:");
                                    egui::color_picker::color_edit_button_srgba(ui, &mut color, egui::color_picker::Alpha::Opaque);
                                    ui.label("Stroke:");
                                    ui.horizontal(|ui| {
                                        ui.label("width:");
                                        ui.add(egui::DragValue::new(&mut stroke.width).speed(0.01).range(0.0..=f32::INFINITY));
                                        ui.label("color:");
                                        egui::color_picker::color_edit_button_srgba(ui, &mut stroke.color, egui::color_picker::Alpha::Opaque);
                                    });
                                    
                                    polygon_shape.set_color(color);
                                    polygon_shape.set_stroke(stroke);
                                },
                                // Ellipse
                                Shape::Ellipse(ellipse_shape) => {
                                    let mut width = ellipse_shape.width();
                                    let mut height = ellipse_shape.height();
                                    let mut color = ellipse_shape.color();
                                    let mut stroke = ellipse_shape.stroke();

                                    ui.horizontal(|ui| {
                                        ui.label("Width:");
                                        ui.add(egui::DragValue::new(&mut width).speed(0.01).range(0.0..=f32::INFINITY));
                                        ui.label("Height:");
                                        ui.add(egui::DragValue::new(&mut height).speed(0.01).range(0.0..=f32::INFINITY));
                                    });
                                    ui.label("Color:");
                                    egui::color_picker::color_edit_button_srgba(ui, &mut color, egui::color_picker::Alpha::Opaque);
                                    ui.label("Stroke:");
                                    ui.horizontal(|ui| {
                                        ui.label("width:");
                                        ui.add(egui::DragValue::new(&mut stroke.width).speed(0.01).range(0.0..=f32::INFINITY));
                                        ui.label("color:");
                                        egui::color_picker::color_edit_button_srgba(ui, &mut stroke.color, egui::color_picker::Alpha::Opaque);
                                    });
                                    
                                    ellipse_shape.set_width(width);
                                    ellipse_shape.set_height(height);
                                    ellipse_shape.set_color(color);
                                    ellipse_shape.set_stroke(stroke);
                                }
                            }
                        } else {
                            ui.label("No shape");
                            let default_values = object.default_values();
                            let current_shape = default_values.shape();
                            egui::ComboBox::from_label(format!("{}", current_shape))
                                .selected_text(format!("{}", current_shape))
                                .show_ui(ui, |ui| {
                                    for default_shape in [
                                        DefaultShape::LineSegment,
                                        DefaultShape::Rectangle,
                                        DefaultShape::RegularPolygon,
                                        DefaultShape::Circle
                                    ] {
                                        if ui.selectable_value(
                                            &mut default_values.shape(),
                                            default_shape,
                                            format!("{}", default_shape)
                                        ).clicked() {
                                            default_values.set_shape(default_shape);
                                        }
                                    }
                                });
                            
                            match default_values.shape() {
                                DefaultShape::LineSegment => {
                                    let mut start = default_values.start();
                                    let mut end = default_values.end();
                                    let mut stroke = default_values.stroke();

                                    ui.label("Start");
                                    ui.horizontal(|ui| {
                                        ui.label("x:");
                                        ui.add(egui::DragValue::new(&mut start.x).speed(0.01));
                                        ui.label("y:");
                                        ui.add(egui::DragValue::new(&mut start.y).speed(0.01));
                                    });
                                    ui.label("End");
                                    ui.horizontal(|ui| {
                                        ui.label("x:");
                                        ui.add(egui::DragValue::new(&mut end.x).speed(0.01));
                                        ui.label("y:");
                                        ui.add(egui::DragValue::new(&mut end.y).speed(0.01));
                                    });
                                    ui.label("Stroke:");
                                    ui.horizontal(|ui| {
                                        ui.label("width:");
                                        ui.add(egui::DragValue::new(&mut stroke.width).speed(0.01).range(0.0..=f32::INFINITY));
                                        ui.label("color:");
                                        egui::color_picker::color_edit_button_srgba(ui, &mut stroke.color, egui::color_picker::Alpha::Opaque);
                                    });

                                    default_values.set_start(start);
                                    default_values.set_end(end);
                                    default_values.set_stroke(stroke);
                                },
                                DefaultShape::Rectangle => {
                                    let mut width = default_values.width();
                                    let mut height = default_values.height();
                                    let mut stroke = default_values.stroke();
                                    let mut color = default_values.color();

                                    ui.horizontal(|ui| {
                                        ui.label("width:");
                                        ui.add(egui::DragValue::new(&mut width).speed(0.01).range(0.0..=f32::INFINITY));
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label("height:");
                                        ui.add(egui::DragValue::new(&mut height).speed(0.01).range(0.0..=f32::INFINITY));
                                    });
                                    ui.label("Stroke:");
                                    ui.horizontal(|ui| {
                                        ui.label("width:");
                                        ui.add(egui::DragValue::new(&mut stroke.width).speed(0.01).range(0.0..=f32::INFINITY));
                                        ui.label("color:");
                                        egui::color_picker::color_edit_button_srgba(ui, &mut stroke.color, egui::color_picker::Alpha::Opaque);
                                    });
                                    ui.label("Color");
                                    egui::color_picker::color_edit_button_srgba(ui, &mut color, egui::color_picker::Alpha::Opaque);

                                    default_values.set_width(width);
                                    default_values.set_height(height);
                                    default_values.set_stroke(stroke);
                                    default_values.set_color(color);
                                },
                                DefaultShape::RegularPolygon => {
                                    let mut num_edges = default_values.num_edges();
                                    let mut radius = default_values.radius();
                                    let mut stroke = default_values.stroke();
                                    let mut color = default_values.color();

                                    ui.horizontal(|ui| {
                                        ui.label("number of edges:");
                                        ui.add(egui::DragValue::new(&mut num_edges).range(3..=u32::MAX));
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label("radius:");
                                        ui.add(egui::DragValue::new(&mut radius).speed(0.01).range(0.0..=f32::INFINITY));
                                    });
                                    ui.label("Stroke:");
                                    ui.horizontal(|ui| {
                                        ui.label("width:");
                                        ui.add(egui::DragValue::new(&mut stroke.width).speed(0.01).range(0.0..=f32::INFINITY));
                                        ui.label("color:");
                                        egui::color_picker::color_edit_button_srgba(ui, &mut stroke.color, egui::color_picker::Alpha::Opaque);
                                    });
                                    ui.label("Color");
                                    egui::color_picker::color_edit_button_srgba(ui, &mut color, egui::color_picker::Alpha::Opaque);

                                    default_values.set_num_edges(num_edges);
                                    default_values.set_radius(radius);
                                    default_values.set_stroke(stroke);
                                    default_values.set_color(color);
                                },
                                DefaultShape::Circle => {
                                    let mut radius = default_values.radius();
                                    let mut stroke = default_values.stroke();
                                    let mut color = default_values.color();

                                    ui.horizontal(|ui| {
                                        ui.label("radius:");
                                        ui.add(egui::DragValue::new(&mut radius).speed(0.01).range(0.0..=f32::INFINITY));
                                    });
                                    ui.label("Stroke:");
                                    ui.horizontal(|ui| {
                                        ui.label("width:");
                                        ui.add(egui::DragValue::new(&mut stroke.width).speed(0.01).range(0.0..=f32::INFINITY));
                                        ui.label("color:");
                                        egui::color_picker::color_edit_button_srgba(ui, &mut stroke.color, egui::color_picker::Alpha::Opaque);
                                    });
                                    ui.label("Color");
                                    egui::color_picker::color_edit_button_srgba(ui, &mut color, egui::color_picker::Alpha::Opaque);

                                    default_values.set_radius(radius);
                                    default_values.set_stroke(stroke);
                                    default_values.set_color(color);
                                },
                            }
                            if ui.button("Add shape").clicked() {
                                let shape = Shape::new(object.default_values());
                                object.set_shape(shape);
                            }
                        }
                    });
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(viewport) = &mut self.viewport {
                viewport.update(ui);
            } else {
                let rect = ui.clip_rect();
                self.viewport = Some(Viewport::new(&rect));
            }

            if let Some(viewport) = &self.viewport {
                let painter = ui.painter_at(viewport.get_rect());
                if self.draw_grid {
                    viewport.draw_grid(&painter);
                }
                self.hovered_object_id = None;
                for object in self.objects.iter() {
                    viewport.draw_object(&painter, object, self.is_selected(object));
                    if viewport.check_hitbox(object) {
                        self.hovered_object_id = Some(object.id());
                    }
                }
                if let Some(object) = self.get_selected() {
                    viewport.draw_gizmo(&painter, object.transform());
                }
            }
        });
    }
}