use eframe::egui;
use crate::core::document::Document;
use crate::tools::{Tool, brush::draw_line, bucket::flood_fill};
use crate::filters::basic::{grayscale, invert, gaussian_blur};

pub struct RustyPsApp {
    document: Document,
    // Add texture for rendering the canvas
    canvas_texture: Option<egui::TextureHandle>,
    active_tool: Tool,
    brush_color: [f32; 4],
    brush_size: f32,
    brush_hardness: f32,
    last_mouse_pos: Option<(f32, f32)>,
    zoom: f32,
    pan: egui::Vec2,
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
            brush_hardness: 1.0,
            last_mouse_pos: None,
            zoom: 1.0,
            pan: egui::vec2(0.0, 0.0),
        }
    }

    fn update_canvas_texture(&mut self, ctx: &egui::Context) {
        if !self.document.is_dirty && self.canvas_texture.is_some() {
            return;
        }

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

        self.document.is_dirty = false;
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
                    if ui.button("Gaussian Blur").clicked() {
                        self.document.save_state();
                        if let Some(active_layer) = self.document.get_active_layer_mut() {
                            gaussian_blur(&mut active_layer.data, 3.0);
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
                    if ui.selectable_label(self.active_tool == Tool::Eraser, "🧽 Eraser").clicked() {
                        self.active_tool = Tool::Eraser;
                    }
                    if ui.selectable_label(self.active_tool == Tool::Picker, "💉 Picker").clicked() {
                        self.active_tool = Tool::Picker;
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
                let mut layer_blend_mode_to_change = None;
                let mut layer_opacity_to_change = None;

                for (i, layer) in self.document.layers.iter_mut().enumerate().rev() {
                    let is_active = self.document.active_layer_index == i;

                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            let mut visible = layer.visible;
                            if ui.checkbox(&mut visible, "").changed() {
                                layer_to_toggle = Some((i, visible));
                            }

                            if ui.selectable_label(is_active, &layer.name).clicked() {
                                self.document.active_layer_index = i;
                            }
                        });

                        if is_active {
                            let mut opacity = layer.opacity as f32 / 255.0;
                            if ui.add(egui::Slider::new(&mut opacity, 0.0..=1.0).text("Opacity")).changed() {
                                layer_opacity_to_change = Some((i, (opacity * 255.0) as u8));
                            }

                            let mut current_blend_mode = layer.blend_mode;
                            egui::ComboBox::from_id_source(format!("blend_mode_{}", i))
                                .selected_text(format!("{:?}", current_blend_mode))
                                .show_ui(ui, |ui| {
                                    use crate::core::layer::BlendMode;
                                    ui.selectable_value(&mut current_blend_mode, BlendMode::Normal, "Normal");
                                    ui.selectable_value(&mut current_blend_mode, BlendMode::Multiply, "Multiply");
                                    ui.selectable_value(&mut current_blend_mode, BlendMode::Screen, "Screen");
                                });
                            if current_blend_mode != layer.blend_mode {
                                layer_blend_mode_to_change = Some((i, current_blend_mode));
                            }
                        }
                    });
                }

                if let Some((i, visible)) = layer_to_toggle {
                    self.document.layers[i].visible = visible;
                    self.document.is_dirty = true;
                }
                if let Some((i, mode)) = layer_blend_mode_to_change {
                    self.document.layers[i].blend_mode = mode;
                    self.document.is_dirty = true;
                }
                if let Some((i, opacity)) = layer_opacity_to_change {
                    self.document.layers[i].opacity = opacity;
                    self.document.is_dirty = true;
                }

                ui.separator();
                if ui.button("+ New Layer").clicked() {
                    let new_layer_idx = self.document.layers.len() + 1;
                    self.document.add_layer(format!("Layer {}", new_layer_idx));
                }

                ui.separator();
                ui.heading("Tool Properties");
                if self.active_tool == Tool::Brush || self.active_tool == Tool::Eraser {
                    ui.add(egui::Slider::new(&mut self.brush_size, 1.0..=100.0).text("Size"));
                    ui.add(egui::Slider::new(&mut self.brush_hardness, 0.0..=1.0).text("Hardness"));
                }
                ui.color_edit_button_rgba_unmultiplied(&mut self.brush_color);
            });

        // Central Canvas
        egui::CentralPanel::default().show(ctx, |ui| {
            // Handle Zoom via scroll
            if ui.rect_contains_pointer(ui.max_rect()) {
                let scroll_delta = ctx.input(|i| i.smooth_scroll_delta.y);
                if scroll_delta != 0.0 {
                    let old_zoom = self.zoom;
                    self.zoom *= 1.0 + (scroll_delta * 0.005);
                    self.zoom = self.zoom.clamp(0.1, 10.0);

                    if let Some(mouse_pos) = ctx.pointer_latest_pos() {
                        // Adjust pan to zoom towards mouse position
                        let offset_from_center = mouse_pos.to_vec2() - ui.max_rect().center().to_vec2();
                        self.pan -= offset_from_center * (self.zoom / old_zoom - 1.0);
                    }
                }
            }

            // Update the texture
            self.update_canvas_texture(ctx);

            if let Some(texture) = &self.canvas_texture {
                let texture_size = texture.size_vec2();
                let scaled_size = texture_size * self.zoom;

                // Position image based on pan and center
                let center = ui.max_rect().center() + self.pan;
                let rect = egui::Rect::from_center_size(center, scaled_size);

                // Allocate interaction area for the whole central panel to capture panning everywhere
                let panel_rect = ui.max_rect();
                let response = ui.allocate_rect(panel_rect, egui::Sense::click_and_drag());

                // Handle canvas panning using Middle Mouse Button or Space + Drag
                let panning = response.dragged_by(egui::PointerButton::Middle)
                    || (ctx.input(|i| i.modifiers.command) && response.dragged());

                if panning {
                    self.pan += response.drag_delta();
                    self.last_mouse_pos = None;
                }

                ui.painter().image(
                    texture.id(),
                    rect,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );

                // Handle tool interaction (only if left clicking and hovering the image)
                if !panning && self.document.get_active_layer_mut().is_some() {
                    if response.dragged_by(egui::PointerButton::Primary) || response.clicked_by(egui::PointerButton::Primary) {
                        if let Some(pos) = response.interact_pointer_pos() {
                            // Only draw if pointer is within the canvas rect
                            if rect.contains(pos) {
                                // Map screen coordinate to image coordinate
                                let img_x = (pos.x - rect.min.x) / self.zoom;
                                let img_y = (pos.y - rect.min.y) / self.zoom;

                            let rgba = image::Rgba([
                                (self.brush_color[0] * 255.0) as u8,
                                (self.brush_color[1] * 255.0) as u8,
                                (self.brush_color[2] * 255.0) as u8,
                                (self.brush_color[3] * 255.0) as u8,
                            ]);

                            match self.active_tool {
                                Tool::Brush | Tool::Eraser => {
                                    if self.last_mouse_pos.is_none() {
                                        // Starting a new stroke
                                        self.document.save_state();
                                    }
                                    let is_eraser = self.active_tool == Tool::Eraser;
                                    let layer = self.document.get_active_layer_mut().unwrap(); // Safe as we are in if let
                                    if let Some(last_pos) = self.last_mouse_pos {
                                        draw_line(
                                            &mut layer.data,
                                            last_pos,
                                            (img_x, img_y),
                                            rgba,
                                            self.brush_size,
                                            self.brush_hardness,
                                            is_eraser,
                                        );
                                    } else {
                                        draw_line(
                                            &mut layer.data,
                                            (img_x, img_y),
                                            (img_x, img_y),
                                            rgba,
                                            self.brush_size,
                                            self.brush_hardness,
                                            is_eraser,
                                        );
                                    }
                                    self.last_mouse_pos = Some((img_x, img_y));
                                }
                                Tool::Picker => {
                                    if response.clicked() || response.dragged() {
                                        let ix = img_x as u32;
                                        let iy = img_y as u32;
                                        let layer = self.document.get_active_layer_mut().unwrap();
                                        if ix < layer.data.width() && iy < layer.data.height() {
                                            let pixel = layer.data.get_pixel(ix, iy);
                                            self.brush_color = [
                                                pixel[0] as f32 / 255.0,
                                                pixel[1] as f32 / 255.0,
                                                pixel[2] as f32 / 255.0,
                                                pixel[3] as f32 / 255.0,
                                            ];
                                        }
                                    }
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
                            } // End of rect.contains(pos) check
                        }
                    } else {
                        self.last_mouse_pos = None;
                    }
                }
            }
        });
    }
}
