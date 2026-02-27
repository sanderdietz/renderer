use eframe::egui;

const INIT_WIDTH: usize = 1200;
const INIT_HEIGHT: usize = 800;

mod app;
use app::App;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([INIT_WIDTH as f32, INIT_HEIGHT as f32])
        .with_position(egui::Pos2::new(300.0, 100.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Renderer",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}

// impl MyApp {
//     fn new() -> Self {
//         Self {
//             viewport: None
//         }
//         // let rect = egui::Rect::from_min_max(egui::Pos2::new(0.0, 0.0), egui::Pos2::new(0.1, 0.1));
//         // let color = egui::Color32::from_rgba_unmultiplied(0, 255, 100, 255);
//         // let object = Object::new("defaultrect".to_string()).set_geometry(rect).set_color(color);

//         // Self {
//         //     viewport_size: None,
//         //     mouse_pos: None,
//         //     world_center: egui::Vec2::new(0.0, 0.0),
//         //     zoom: 1.0,
//         //     objects: vec![object],
//         //     panel: Panel::None,
//         //     new_object_name: "New object".to_string()
//         // }
//     }

//     // fn viewport_pos_to_world_vec(&self, viewport_pos: &egui::Pos2) -> egui::Vec2 {
//     //     // TODO hier wordt expect() gebruikt, beter is een situatie waar viewport is een eigen struct waarin alles sowieso werkt
//     //     let size = self.viewport_size.expect("Viewport not initialized");
//     //     let normalized = 2.0 * self.zoom * viewport_pos.to_vec2() * egui::Vec2::new(1.0, -1.0) / size.x;
//     //     normalized - (self.world_center - egui::Vec2::new(-self.zoom, self.zoom * size.y / size.x))
//     // }
// }

// impl eframe::App for MyApp {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::SidePanel::left("left panel")
//         .show(ctx, |ui| {

//         });
//         egui::SidePanel::right("right panel")
//         .show(ctx, |ui| {
//             // ui.take_available_space();

//             // if let Some(size) = self.viewport_size {
//             //     ui.label(format!("width: {}", size.x));
//             //     ui.label(format!("height: {}", size.y));
//             // }
//             // if let Some(pos) = self.mouse_pos {
//             //     ui.label(format!("mouse_pos: {}", pos));
//             //     if self.viewport_size.is_some() {
//             //         ui.label(format!("world_vec: {}", self.viewport_pos_to_world_vec(&pos)));
//             //     }
//             // } else {
//             //     ui.label("No mouse");
//             // }
//             // // ui.label(format!("world_center: {}", self.world_center));

//             // egui::CollapsingHeader::new("Objects").show(ui, |ui| {
//             //     for obj in &self.objects {
//             //         egui::CollapsingHeader::new(obj.get_name()).show(ui, |ui| {
//             //             match obj.get_geometry() {
//             //                 Some(geometry) => {
//             //                     egui::CollapsingHeader::new("Geometry").show(ui, |ui| {
//             //                         ui.label("Vertices:");
//             //                         ui.horizontal(|ui| {
//             //                             ui.add(DragValue::new(&mut geometry.left_bottom().x));
//             //                             ui.add(DragValue::new(&mut geometry.left_bottom().y));

//             //                         });
//             //                     });
//             //                 },
//             //                 None => {
//             //                     ui.button("Add geometry");
//             //                 }
//             //             }
//             //             match obj.get_color() {
//             //                 Some(color) => {
//             //                     ui.label("Color");
//             //                 },
//             //                 None => {
//             //                     ui.button("Add color");
//             //                 }
//             //             }
//             //         });
//             //     }
//             //     match self.panel {
//             //         Panel::NewObject => {
//             //             ui.text_edit_singleline(&mut self.new_object_name);
//             //             if ui.button("Create object").clicked() {
//             //                 self.objects.push(Object::new(self.new_object_name.clone()));
//             //                 self.panel = Panel::None;
//             //             }
//             //         },
//             //         Panel::None => {}
//             //     }
//             //     if ui.button("Add object").clicked() {
//             //         self.panel = Panel::NewObject;
//             //     }
//             // });
            
//         });
//         egui::CentralPanel::default().show(ctx, |ui| {
//             // self.viewport_size = Some(ui.clip_rect().size());
//             // self.mouse_pos = ctx.pointer_hover_pos();
                    
//             // let painter = ui.painter();
//             // let mut x = 0.0;
//             // while x < 900.0 {
//             //     painter.line_segment(
//             //         [egui::Pos2::new(x, 0.0), egui::Pos2::new(x, 500.0)],
//             //         egui::Stroke::new(1.0, egui::Color32::WHITE)
//             //     );
//             //     x += 30.0;
//             // }
                
//             // } else {
//             //     // Rendering
//             //     self.update_pixels();
//             //     let image = ColorImage::from_rgba_unmultiplied(
//             //         [self.width, self.height],
//             //         &self.pixels
//             //     );
//             //     if let Some(texture) = &mut self.texture {
//             //         texture.set(image, egui::TextureOptions::NEAREST);
//             //     }
//             //     // Mouse clicking and positition
//             //     if let Some(texture) = &self.texture {
//             //         let response = ui.add(
//             //             egui::Image::new(texture)
//             //             .sense(egui::Sense::click_and_drag())
//             //         );
//             //         let translation = response.rect.left_top().to_vec2();
//             //         if let Some(temp) = response.hover_pos() {
//             //             let current_mouse_pos = temp - translation;
//             //             let current_mouse = self.viewport_pos_to_world_vec(&current_mouse_pos);
//             //             if let Some(old_mouse_pos) = &self.mouse_pos && response.dragged_by(egui::PointerButton::Secondary) {
//             //                 let old_mouse = self.viewport_pos_to_world_vec(&old_mouse_pos);
//             //                 let mouse_travelled = current_mouse - old_mouse;
//             //                 self.world_center = self.world_center - mouse_travelled;
//             //             }
//             //             self.mouse_pos = Some(current_mouse_pos);
//             //         } else {
//             //             self.mouse_pos = None;
//             //         }
//             //     }
//             //     let painter = ui.painter();
//             //     painter.add(Shape::convex_polygon(
//             //         vec![
//             //             egui::Pos2::new(0.0, 0.0),
//             //             egui::Pos2::new(150.0, 100.0),
//             //             egui::Pos2::new(100.0, 150.0),
//             //             egui::Pos2::new(250.0, 150.0)
//             //         ],
//             //         egui::Color32::RED,
//             //         egui::Stroke::new(1.0, egui::Color32::BLACK)
//             //     ));
//             //     // Mouse scrolling
//             //     let new_zoom = self.zoom - 0.001 * ui.input(|i| i.smooth_scroll_delta.y);
//             //     if new_zoom > 0.0 {
//             //         self.zoom = new_zoom
//             //     }
//             // }
//         });
//     }
// }
