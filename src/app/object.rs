use std::sync::atomic::{AtomicUsize, Ordering};

pub mod transform;
use transform::Transform;
pub mod geometry;
use geometry::*;

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

pub struct Object {
    id: usize,
    name: String,
    transform: Transform,
    shape: Option<Shape>,
    default_values: DefaultValues
}

impl Object {
    pub fn new(name: String) -> Self {
        Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            name: name,
            transform: Transform::ZERO,
            shape: None,
            default_values: DefaultValues::new()
        }
    }

    pub fn set_shape(&mut self, shape: Shape) {
        self.shape = Some(shape);
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn id(&self) -> usize {
        self.id
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn shape_mut(&mut self) -> Option<&mut Shape> {
        self.shape.as_mut()
    }

    pub fn shape(&self) -> Option<&Shape> {
        self.shape.as_ref()
    }

    pub fn default_values(&mut self) -> &mut DefaultValues {
        &mut self.default_values
    }
}

// #[derive(PartialEq, Debug)]
// enum DefaultShape {
//     Rectangle,
//     RegularPolygon,
//     Ellipse
// }

// impl DefaultShape {
//     fn to_string(&self) -> String {
//         match self {
//             DefaultShape::Rectangle => String::from("Rectangle"),
//             DefaultShape::RegularPolygon => String::from("Regular polygon"),
//             DefaultShape::Ellipse => String::from("Ellipse")
//         }
//     }
// }

// pub struct NewObject {
//     panel_enabled: bool,
//     name: String,
//     geometry: Option<Shape>,
//     fill_color: Option<egui::Color32>,
//     stroke: Option<egui::Stroke>,
//     selected: DefaultShape,
//     add_shape: bool
// }


// impl NewObject {
//     pub fn new() -> Self {
//         Self {
//             panel_enabled: false,
//             name: "Object".to_string(),
//             geometry: None,
//             fill_color: None,
//             stroke: None,
//             selected: DefaultShape::Rectangle,
//             add_shape: false
//         }
//     }

    // pub fn render_ui(&mut self, ctx: &egui::Context) {
    //     egui::Window::new("Create new object").show(ctx, |ui| {
    //         ui.horizontal(|ui| {
    //             ui.label("Object name: ");
    //             ui.text_edit_singleline(&mut self.name);
    //         });

    //         egui::CollapsingHeader::new("Shape").show(ui, |ui| {
    //             ui.label("iets");
    //         });

    //         if ui.button("Add shape").clicked() {
    //             self.add_shape = true;
    //         }
    //         if self.add_shape {
    //             egui::ComboBox::from_label("")
    //                 .selected_text(format!("{}", self.selected.to_string()))
    //                 .show_ui(ui, |ui| {
    //                     ui.selectable_value(&mut self.selected, DefaultShape::Rectangle, "Rectangle").clicked();
    //                     ui.selectable_value(&mut self.selected, DefaultShape::RegularPolygon, "Regular polygon").clicked();
    //                     ui.selectable_value(&mut self.selected, DefaultShape::Ellipse, "Ellipse").clicked();
    //                 });
    //         }

    //         // if ui.button("Add geometry").clicked() {
    //         //     self.geometry = Some(Shape::new(vec![]));
    //         // }
    //         // if let Some(geometry) = &mut self.geometry {
    //         //     ui.label("Vertices:");
    //         //     let mut vertices: Vec<egui::Pos2> = vec![];
    //         //     for vertex in geometry.get_vertices() {
    //         //         let mut add = true;
    //         //         let mut x = vertex.x;
    //         //         let mut y = vertex.y;
    //         //         ui.horizontal(|ui| {
    //         //             ui.add(egui::DragValue::new(&mut x).speed(0.01));
    //         //             ui.add(egui::DragValue::new(&mut y).speed(0.01));
    //         //             if ui.button("Remove").clicked() {
    //         //                 add = false;
    //         //             }
    //         //         });
    //         //         if add {
    //         //             vertices.push(egui::Pos2::new(x, y));
    //         //         }
    //         //     }
    //         //     if ui.button("Add vertex").clicked() {
    //         //         vertices.push(egui::Pos2::ZERO);
    //         //     }
    //         //     self.geometry = Some(Polygon::new(vertices));
    //         // }
    //         // ui.horizontal(|ui| {
    //         //     ui.label("Transform:");
    //         // });
    //         if ui.button("Cancel").clicked() {
    //             self.disable_panel();
    //         }
    //     });
    // }

//     pub fn enable_panel(&mut self) {
//         self.panel_enabled = true;
//     }

//     pub fn disable_panel(&mut self) {
//         self.panel_enabled = false;
//     }

//     pub fn is_panel_enabled(&self) -> bool {
//         self.panel_enabled
//     }
// }