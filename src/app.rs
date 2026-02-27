use eframe::egui;

pub mod viewport;
use viewport::Viewport;
pub mod object;
use object::{Object, NewObject};
use object::transform::Transform;
use object::geometry::Polygon;

pub struct App {
    viewport: Option<Viewport>,
    objects: Vec<Object>,
    selected_object_id: Option<usize>,
    draw_grid: bool,
    object_panel: NewObject
}

impl App {
    pub fn new() -> Self {
        let mut object1 = Object::new("obj1");
        let mut geometry = Polygon::new(vec![]);
        geometry.add(egui::Pos2::new(-0.2, -0.2));
        geometry.add(egui::Pos2::new(0.2, -0.2));
        geometry.add(egui::Pos2::new(0.2, 0.2));
        geometry.add(egui::Pos2::new(-0.2, 0.2));
        object1.set_geometry(geometry);

        let object2 = Object::new("obj2");

        let object3 = Object::new("obj3");

        let mut object4 = Object::new("obj4");
        object4.set_transform(Transform::new(egui::Pos2::new(0.5, 0.25), 0.0));
        
        Self {
            viewport: None,
            objects: vec![object1, object2, object3, object4],
            selected_object_id: None,
            draw_grid: true,
            object_panel: NewObject::new()
        }
    }

    fn get_selected_mut(&mut self) -> Option<&mut Object> {
        for object in &mut self.objects {
            match self.selected_object_id {
                Some(id) => {
                    if id == object.get_id() {
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
            Some(id) => id == object.get_id(),
            None => false
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.object_panel.is_panel_enabled() {
            self.object_panel.render_ui(ctx);
        }

        egui::SidePanel::left("left panel").show(ctx, |ui| {
            for object in self.objects.iter() {
                let selected = self.is_selected(object);
                let response =  ui.selectable_label(selected, format!("{}", object.get_name()));
                if response.clicked()
                {
                    self.selected_object_id = match self.selected_object_id {
                        Some(id) => {
                            if id == object.get_id() {
                                None
                            } else {
                                Some(object.get_id())
                            }
                        },
                        None => {
                            Some(object.get_id())
                        }
                    }
                }
            }
            if ui.button("Add object").clicked() {
                self.object_panel.enable_panel();
            }
        });

        egui::SidePanel::right("right panel").show(ctx, |ui| {
            ui.checkbox(&mut self.draw_grid, "Grid");

            if let Some(object) = self.get_selected_mut() {
                ui.label(format!("{}", object.get_name()));
                ui.label("Transform:");
                let mut pos = object.get_transform().get_pos();
                let mut angle = object.get_transform().get_angle();
                ui.horizontal(|ui| {
                    ui.add(egui::DragValue::new(&mut pos.x).speed(0.01));
                    ui.add(egui::DragValue::new(&mut pos.y).speed(0.01));
                    ui.add(egui::DragValue::new(&mut angle).speed(0.01));
                });
                let transform = Transform::new(pos, angle);
                object.set_transform(transform);

                if let Some(geometry) = object.get_geometry() {
                    ui.label("Geometry:");
                    let mut vertices: Vec<egui::Pos2> = vec![];
                    for vertex in geometry.get_vertices() {
                        let mut x = vertex.x;
                        let mut y = vertex.y;
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut x).speed(0.01));
                            ui.add(egui::DragValue::new(&mut y).speed(0.01));
                        });
                        vertices.push(egui::Pos2::new(x, y));
                    }
                    let new_geometry = Polygon::new(vertices);
                    object.set_geometry(new_geometry);
                }
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
                    viewport.draw_object(&painter, object, self.is_selected(object));
                }
                if self.draw_grid {
                    viewport.draw_grid(&painter);
                }
                if let Some(object) = self.get_selected() {
                    viewport.draw_gizmo(&painter, object.get_transform());
                }
            }
        });
    }
}