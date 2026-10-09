use eframe::egui;
use crate::core::document::Document;
use crate::tools::{Tool, brush::draw_line, bucket::flood_fill};
use crate::filters::basic::{grayscale, invert};

pub struct RustyPsApp {
    document: Document,
    // Add texture for rendering the canvas
    canvas_texture: Option<egui::TextureHandle>,
    active_tool: Tool,
    brush_color: [f32; 4],
    brush_size: f32,
    last_mouse_pos: Option<(f32, f32)>,
}

impl RustyPsApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Initialize an empty document
        let doc = Document::new(800, 600);

        Self {
            document: doc,
            canvas_texture: None,
            active_tool: Tool::Brush,
            brush_color: [0.0, 0.0, 0.0, 1.0], // Black default
            brush_size: 10.0,
            last_mouse_pos: None,
        }
    }

    fn update_canvas_texture(&mut self, ctx: &egui::Context) {
        let rendered_image = self.document.render();
        let size = [rendered_image.width() as usize, rendered_image.height() as usize];
        let pixels = rendered_image.into_flat_samples();

        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            size,
            pixels.as_slice(),
        );

        if let Some(texture) = &mut self.canvas_texture {
            texture.set(color_image, egui::TextureOptions::LINEAR);
        } else {
            self.canvas_texture = Some(ctx.load_texture(
                "canvas",
                color_image,
                egui::TextureOptions::LINEAR,
            ));
        }
    }
}

impl eframe::App for RustyPsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top Menu Bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Undo").clicked() {
                        self.document.undo();
                    }
                    if ui.button("Redo").clicked() {
                        self.document.redo();
                    }
                });
                ui.menu_button("Filters", |ui| {
                    if ui.button("Grayscale").clicked() {
                        self.document.save_state();
                        if let Some(active_layer) = self.document.get_active_layer_mut() {
                            grayscale(&mut active_layer.data);
                        }
                    }
                    if ui.button("Invert").clicked() {
                        self.document.save_state();
                        if let Some(active_layer) = self.document.get_active_layer_mut() {
                            invert(&mut active_layer.data);
                        }
                    }
                });
            });
        });

        // Left Toolbar
        egui::SidePanel::left("left_panel")
            .resizable(false)
            .default_width(50.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Tools");
                    ui.separator();
                    if ui.selectable_label(self.active_tool == Tool::Brush, "🖌 Brush").clicked() {
                        self.active_tool = Tool::Brush;
                    }
                    if ui.selectable_label(self.active_tool == Tool::Fill, "🪣 Fill").clicked() {
                        self.active_tool = Tool::Fill;
                    }
                });
            });

        // Right Panels (Layers, Properties)
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Layers");
                ui.separator();

                let mut layer_to_toggle = None;
                for (i, layer) in self.document.layers.iter_mut().enumerate().rev() {
                    ui.horizontal(|ui| {
                        let mut visible = layer.visible;
                        if ui.checkbox(&mut visible, "").changed() {
                            layer_to_toggle = Some((i, visible));
                        }

                        let is_active = self.document.active_layer_index == i;
                        if ui.selectable_label(is_active, &layer.name).clicked() {
                            self.document.active_layer_index = i;
                        }
                    });
                }

                if let Some((i, visible)) = layer_to_toggle {
                    self.document.layers[i].visible = visible;
                }

                ui.separator();
                if ui.button("+ New Layer").clicked() {
                    let new_layer_idx = self.document.layers.len() + 1;
                    self.document.add_layer(format!("Layer {}", new_layer_idx));
                }

                ui.separator();
                ui.heading("Tool Properties");
                if self.active_tool == Tool::Brush {
                    ui.add(egui::Slider::new(&mut self.brush_size, 1.0..=100.0).text("Size"));
                }
                ui.color_edit_button_rgba_unmultiplied(&mut self.brush_color);
            });

        // Central Canvas
        egui::CentralPanel::default().show(ctx, |ui| {
            // Update the texture
            self.update_canvas_texture(ctx);

            if let Some(texture) = &self.canvas_texture {
                let available_size = ui.available_size();
                let texture_size = texture.size_vec2();

                // Calculate scale to fit the window while maintaining aspect ratio
                let scale = (available_size.x / texture_size.x)
                    .min(available_size.y / texture_size.y)
                    .min(1.0); // Don't scale up past 1:1

                let scaled_size = texture_size * scale;

                // Center the image
                let rect = egui::Rect::from_center_size(
                    ui.max_rect().center(),
                    scaled_size,
                );

                let response = ui.allocate_rect(rect, egui::Sense::click_and_drag());

                ui.painter().image(
                    texture.id(),
                    rect,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );

                // Handle tool interaction
                if self.document.get_active_layer_mut().is_some() {
                    if response.dragged() || response.clicked() {
                        if let Some(pos) = response.interact_pointer_pos() {
                            // Map screen coordinate to image coordinate
                            let img_x = (pos.x - rect.min.x) / scale;
                            let img_y = (pos.y - rect.min.y) / scale;

                            let rgba = image::Rgba([
                                (self.brush_color[0] * 255.0) as u8,
                                (self.brush_color[1] * 255.0) as u8,
                                (self.brush_color[2] * 255.0) as u8,
                                (self.brush_color[3] * 255.0) as u8,
                            ]);

                            match self.active_tool {
                                Tool::Brush => {
                                    if self.last_mouse_pos.is_none() {
                                        // Starting a new stroke
                                        self.document.save_state();
                                    }
                                    let layer = self.document.get_active_layer_mut().unwrap(); // Safe as we are in if let
                                    if let Some(last_pos) = self.last_mouse_pos {
                                        draw_line(
                                            &mut layer.data,
                                            last_pos,
                                            (img_x, img_y),
                                            rgba,
                                            self.brush_size,
                                        );
                                    } else {
                                        draw_line(
                                            &mut layer.data,
                                            (img_x, img_y),
                                            (img_x, img_y),
                                            rgba,
                                            self.brush_size,
                                        );
                                    }
                                    self.last_mouse_pos = Some((img_x, img_y));
                                }
                                Tool::Fill => {
                                    if response.clicked() {
                                        self.document.save_state();
                                        let layer = self.document.get_active_layer_mut().unwrap();
                                        flood_fill(
                                            &mut layer.data,
                                            img_x as u32,
                                            img_y as u32,
                                            rgba,
                                        );
                                    }
                                }
                            }
                        }
                    } else {
                        self.last_mouse_pos = None;
                    }
                }
            }
        });
    }
}
