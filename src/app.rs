use eframe::egui;

pub mod viewport;
use viewport::Viewport;
pub mod object;
use object::Object;
use object::transform::Transform;
use object::geometry::*;

pub struct App {
    viewport: Option<Viewport>,
    objects: Vec<Object>,
    selected_object_id: Option<usize>,
    draw_grid: bool,
    new_object: bool,
    new_name: String,
    new_default_shape: DefaultShape
}

impl App {
    pub fn new() -> Self {
        // let mut object1 = Object::new("obj1");
        // // let mut geometry = Polygon::new(vec![]);
        // // geometry.add(egui::Pos2::new(-0.2, -0.2));
        // // geometry.add(egui::Pos2::new(0.2, -0.2));
        // // geometry.add(egui::Pos2::new(0.2, 0.2));
        // // geometry.add(egui::Pos2::new(-0.2, 0.2));
        // // object1.set_geometry(geometry);

        // let object2 = Object::new("obj2");

        // let object3 = Object::new("obj3");

        // let mut object4 = Object::new("obj4");
        // object4.set_transform(Transform::new(egui::Pos2::new(0.5, 0.25), 0.0));
        
        Self {
            viewport: None,
            objects: Vec::new(),//vec![object1, object2, object3, object4],
            selected_object_id: None,
            draw_grid: true,
            new_object: false,
            new_name: String::from("new object"),
            new_default_shape: DefaultShape::Rectangle
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
                        let transform = object.transform();
                        let mut pos = transform.pos();
                        let mut scale = transform.scale();
                        let mut angle = transform.angle();
                        ui.label("Position");
                        ui.horizontal(|ui| {
                            ui.label("x:");
                            ui.add(egui::DragValue::new(&mut pos.x).speed(0.01));
                            ui.label("y:");
                            ui.add(egui::DragValue::new(&mut pos.y).speed(0.01));
                        });
                        ui.label("Scale");
                        ui.horizontal(|ui| {
                            ui.label("x:");
                            ui.add(egui::DragValue::new(&mut scale.x).speed(0.01));
                            ui.label("y:");
                            ui.add(egui::DragValue::new(&mut scale.y).speed(0.01));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Angle");
                            ui.add(egui::DragValue::new(&mut angle).speed(0.01));
                        });
                        let transform = Transform::new(pos, scale, angle);
                        object.set_transform(transform);
                    });
                
                // Display shape
                egui::CollapsingHeader::new("Shape")
                    .default_open(object.shape().is_some())
                    .show(ui, |ui| {
                        if let Some(shape) = object.shape() {
                            match shape {
                                Shape::Path(path) => ui.label("path"),
                                Shape::Polygon(polygon) => ui.label("polygon"),
                                Shape::Ellipse(ellipse) => ui.label("ellipse")
                            };
                        } else {
                            ui.label("No shape");
                            // let mut default_shape = DefaultShape::Rectangle;
                            egui::ComboBox::from_label("")
                                .selected_text(format!("{}", object.default_shape()))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value( 
                                        object.default_shape(), 
                                        DefaultShape::LineSegment, 
                                        format!("{}", DefaultShape::LineSegment)
                                    );
                                    ui.selectable_value( 
                                        object.default_shape(), 
                                        DefaultShape::Rectangle, 
                                        format!("{}", DefaultShape::Rectangle)
                                    );
                                    ui.selectable_value( 
                                        object.default_shape(), 
                                        DefaultShape::RegularPolygon, 
                                        format!("{}", DefaultShape::RegularPolygon)
                                    );
                                    ui.selectable_value( 
                                        object.default_shape(), 
                                        DefaultShape::Circle, 
                                        format!("{}", DefaultShape::Circle)
                                    );
                                });
                            match object.default_shape() {
                                DefaultShape::LineSegment => {
                                    let mut start = object.default_values().start();
                                    let mut end = object.default_values().end();
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
                                    object.default_values();
                                },
                                DefaultShape::Rectangle => {},
                                DefaultShape::RegularPolygon => {},
                                DefaultShape::Circle => {},
                            }
                            if ui.button("Add shape").clicked() {

                            }
                        }
                    });
                // ui.label("Transform:");
                // let mut pos = object.transform().pos();
                // let mut angle = object.transform().angle();
                // ui.horizontal(|ui| {
                //     ui.add(egui::DragValue::new(&mut angle).speed(0.01));
                // });
                // let transform = Transform::new(pos, angle);
                // object.set_transform(transform);

                // if let Some(geometry) = object.get_geometry() {
                //     ui.label("Geometry:");
                //     let mut vertices: Vec<egui::Pos2> = vec![];
                //     for vertex in geometry.get_vertices() {
                //         let mut x = vertex.x;
                //         let mut y = vertex.y;
                //         ui.horizontal(|ui| {
                //             ui.add(egui::DragValue::new(&mut x).speed(0.01));
                //             ui.add(egui::DragValue::new(&mut y).speed(0.01));
                //         });
                //         vertices.push(egui::Pos2::new(x, y));
                //     }
                //     let new_geometry = Polygon::new(vertices);
                //     object.set_geometry(new_geometry);
                // }
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
                for object in self.objects.iter() {
                    // viewport.draw_object(&painter, object, self.is_selected(object));
                }
                if self.draw_grid {
                    viewport.draw_grid(&painter);
                }
                if let Some(object) = self.get_selected() {
                    viewport.draw_gizmo(&painter, object.transform());
                }
            }
        });
    }
}