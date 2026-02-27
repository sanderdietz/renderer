use eframe::egui::{self, ComboBox};
use std::sync::atomic::{AtomicUsize, Ordering};

pub mod transform;
use transform::Transform;
pub mod geometry;
use geometry::Polygon;

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

pub struct Object {
    id: usize,
    name: String,
    transform: Transform,
    geometry: Option<Polygon>,
    fill_color: Option<egui::Color32>,
    stroke: Option<egui::Stroke>
}

impl Object {
    pub fn new(name: &str) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name: name.to_string(),
            transform: Transform::ZERO,
            geometry: None,
            fill_color: None,
            stroke: None
        }
    }

    pub fn set_geometry(&mut self, geometry: Polygon) {
        self.geometry = Some(geometry);
        self.fill_color = Some(egui::Color32::WHITE);
        self.stroke = Some(egui::Stroke::NONE);
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    pub fn get_transform(&self) -> &Transform {
        &self.transform
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
    
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_geometry(&self) -> &Option<Polygon> {
        &self.geometry
    }

    pub fn get_fill_color(&self) -> Option<egui::Color32> {
        self.fill_color
    }

    pub fn get_stroke(&self) -> Option<egui::Stroke> {
        self.stroke
    }
}


pub struct NewObject {
    panel_enabled: bool,
    name: String,
    geometry: Option<Polygon>,
    fill_color: Option<egui::Color32>,
    stroke: Option<egui::Stroke>
}

impl NewObject {
    pub fn new() -> Self {
        Self {
            panel_enabled: false,
            name: "Object".to_string(),
            geometry: None,
            fill_color: None,
            stroke: None
        }
    }

    pub fn render_ui(&mut self, ctx: &egui::Context) {
        egui::Window::new("Create new object").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Object name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            egui::ComboBox::from_label("Select")
                .selected_text(format!("iets"))
                .show_ui(ui, |ui| {
                    ui.selectable_label(false, "iets");
                    ui.selectable_label(true, "iets anders");
                });
            if ui.button("Add geometry").clicked() {
                self.geometry = Some(Polygon::new(vec![]));
            }
            if let Some(geometry) = &mut self.geometry {
                ui.label("Vertices:");
                let mut vertices: Vec<egui::Pos2> = vec![];
                for vertex in geometry.get_vertices() {
                    let mut add = true;
                    let mut x = vertex.x;
                    let mut y = vertex.y;
                    ui.horizontal(|ui| {
                        ui.add(egui::DragValue::new(&mut x).speed(0.01));
                        ui.add(egui::DragValue::new(&mut y).speed(0.01));
                        if ui.button("Remove").clicked() {
                            add = false;
                        }
                    });
                    if add {
                        vertices.push(egui::Pos2::new(x, y));
                    }
                }
                if ui.button("Add vertex").clicked() {
                    vertices.push(egui::Pos2::ZERO);
                }
                self.geometry = Some(Polygon::new(vertices));
            }
            // ui.horizontal(|ui| {
            //     ui.label("Transform:");
            // });
            if ui.button("Cancel").clicked() {
                self.disable_panel();
            }
        });
    }

    pub fn enable_panel(&mut self) {
        self.panel_enabled = true;
    }

    pub fn disable_panel(&mut self) {
        self.panel_enabled = false;
    }

    pub fn is_panel_enabled(&self) -> bool {
        self.panel_enabled
    }
}