use egui::{Color32, RichText, Vec2, ComboBox};
use egui_i18n::tr;
use std::path::PathBuf;
use super::state::CopaibaApp;

impl CopaibaApp {
    pub fn show_voicebank_header(&mut self, ctx: &egui::Context) {
        let tab_idx = self.current_tab;
        
        // Ensure character texture is loaded
        {
            let tab = &mut self.tabs[tab_idx];
            if tab.character_texture.is_none() {
                if let Some(path) = &tab.character_image_path {
                    if let Ok(data) = std::fs::read(path) {
                        if let Ok(image) = image::load_from_memory(&data) {
                            let size = [image.width() as usize, image.height() as usize];
                            let image_buffer = image.to_rgba8();
                            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                                size,
                                image_buffer.as_flat_samples().as_slice(),
                            );
                            tab.character_texture = Some(ctx.load_texture("char_img", color_image, Default::default()));
                        }
                    }
                }
            }
        }

        egui::TopBottomPanel::top("voicebank_header").show(ctx, |ui| {
            let mut save_readme_btn = false;
            let mut cancel_readme_btn = false;
            
            let (char_tex, char_name, oto_dir, mut readme, original_readme, license, readme_path, root_path) = {
                let tab = &self.tabs[tab_idx];
                (tab.character_texture.as_ref().map(|t| t.id()), tab.character_name.clone(), tab.oto_dir.clone(), tab.readme_text.clone(), tab.original_readme_text.clone(), tab.license_text.clone(), tab.readme_path.clone(), tab.root_path.clone())
            };

            ui.add_space(4.0);
            crate::app::layout::horizontal(ui, self.is_rtl(), |ui| {
                // Character Image (60x60)
                let (rect, _resp) = ui.allocate_at_least(Vec2::new(60.0, 60.0), egui::Sense::hover());
                if let Some(tex_id) = char_tex {
                    ui.painter().image(tex_id, rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), Color32::WHITE);
                } else {
                    ui.painter().rect_filled(rect, 8.0, Color32::from_rgb(30, 30, 46));
                    ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "👤", egui::FontId::proportional(28.0), Color32::GRAY);
                }

                ui.add_space(8.0);

                ui.vertical(|ui| {
                    let name = if char_name.is_empty() {
                        oto_dir.as_ref()
                            .and_then(|p: &PathBuf| p.file_name())
                            .map(|s: &std::ffi::OsStr| s.to_string_lossy().to_string())
                            .unwrap_or_else(|| "Voicebank".to_string())
                    } else { 
                        char_name
                    };
                    ui.label(RichText::new(name).strong().size(16.0));
                    ui.label(RichText::new(oto_dir.as_ref().map(|p: &PathBuf| p.to_string_lossy().to_string()).unwrap_or_default()).color(ui.visuals().weak_text_color()).size(9.0));
                });

                ui.add_space(12.0);
 
                 // Readme / License buttons
                 ui.horizontal(|ui| {
                     if readme_path.is_some() {
                         ui.vertical(|ui| {
                             ui.label(RichText::new("📄 Readme.txt").size(10.0).color(Color32::from_rgb(180, 180, 200)));
                             egui::ScrollArea::vertical().id_salt("readme_scroll").max_height(48.0).show(ui, |ui| {
                                 ui.add(egui::TextEdit::multiline(&mut readme).desired_width(550.0).font(egui::TextStyle::Small).margin(egui::Margin::same(2)));
                             });
                             
                             if readme != original_readme {
                                 ui.horizontal(|ui| {
                                     if ui.button(RichText::new("💾 Salvar").color(Color32::from_rgb(100, 255, 100)).size(10.0)).clicked() {
                                         save_readme_btn = true;
                                     }
                                     if ui.button(RichText::new("✖ Cancelar").color(Color32::from_rgb(255, 100, 100)).size(10.0)).clicked() {
                                         cancel_readme_btn = true;
                                     }
                                 });
                             }
                         });
                    } else if let Some(root) = &root_path {
                         ui.vertical(|ui| {
                             ui.add_space(16.0);
                             let btn = egui::Button::new(RichText::new("➕ Criar readme.txt").size(11.0).color(Color32::from_rgb(170, 170, 220)))
                                 .fill(Color32::TRANSPARENT)
                                 .stroke(egui::Stroke::new(1.0, Color32::from_rgba_premultiplied(100, 100, 150, 50)));
                             
                             if ui.add(btn).clicked() {
                                 let new_path = root.join("readme.txt");
                                 let initial_text = "Insira os detalhes do voicebank aqui...\n";
                                 if std::fs::write(&new_path, initial_text).is_ok() {
                                     let tab = &mut self.tabs[tab_idx];
                                     tab.readme_path = Some(new_path);
                                     tab.readme_text = initial_text.to_string();
                                     tab.original_readme_text = initial_text.to_string();
                                     self.ui.toast_manager.success("readme.txt criado com sucesso!");
                                 } else {
                                     self.ui.toast_manager.error("Falha ao criar o arquivo readme.txt");
                                 }
                             }
                         });
                    }
                    if !license.is_empty() {
                        ui.vertical(|ui| {
                            ui.add_space(8.0);
                            if ui.button(RichText::new("⚖ License").size(11.0).color(Color32::from_rgb(150, 200, 150))).clicked() {
                               self.ui.show_license = true;
                            }
                        });
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(8.0);
                    
                    // Action Buttons (Right to Left)
                    let has_resampler = self.config.resampler_path.is_some();
                    let mut btn = ui.button(RichText::new(format!("🧪 {}", tr!("header.resampler.test"))).strong());
                    if !has_resampler {
                        btn = btn.on_hover_text(tr!("header.resampler.hover"));
                    }
                    if btn.clicked() {
                        if has_resampler {
                            self.resample_current();
                        } else {
                            self.ui.status = tr!("header.resampler.status").to_string();
                        }
                    }
                    
                    ui.add(egui::DragValue::new(&mut self.config.test_duration_ms).suffix(" ms").range(50.0..=2000.0).prefix("ms "));
                    
                    ui.separator();

                    // Pitch Selection
                    ComboBox::from_id_salt("pitch_select")
                        .selected_text(RichText::new(&self.config.test_pitch).color(Color32::from_rgb(137, 180, 250)).strong())
                        .width(60.0)
                        .show_ui(ui, |ui| {
                            egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                                for p in &[
                                    "C1","C#1","D1","D#1","E1","F1","F#1","G1","G#1","A1","A#1","B1",
                                    "C2","C#2","D2","D#2","E2","F2","F#2","G2","G#2","A2","A#2","B2",
                                    "C3","C#3","D3","D#3","E3","F3","F#3","G3","G#3","A3","A#3","B3",
                                    "C4","C#4","D4","D#4","E4","F4","F#4","G4","G#4","A4","A#4","B4",
                                    "C5","C#5","D5","D#5","E5","F5","F#5","G5","G#5","A5","A#5","B5",
                                ] {
                                    ui.selectable_value(&mut self.config.test_pitch, p.to_string(), *p);
                                }
                            });
                        });
                    ui.label(tr!("header.pitch.label"));
                    
                    ui.separator();

                    // Resampler Selection
                    if let Some(res) = &self.config.resampler_path {
                        ui.label(RichText::new(res.file_name().unwrap_or_default().to_string_lossy()).size(10.0).color(ui.visuals().weak_text_color()));
                    } else {
                        ui.label(RichText::new(tr!("header.resampler.none")).size(10.0).color(Color32::from_rgb(243, 139, 168)));
                    }
                    if ui.button(RichText::new(format!("⚙ {}", tr!("header.resampler.select"))).strong()).clicked() {
                        #[cfg(not(target_arch = "wasm32"))]
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.config.resampler_path = Some(path);
                        }
                    }
                });
            });
            ui.add_space(4.0);
            
            // Post-UI Update Logic for Readme
            if save_readme_btn || cancel_readme_btn || readme != self.tabs[tab_idx].readme_text {
                let tab = &mut self.tabs[tab_idx];
                tab.readme_text = readme.clone();
            }

            if cancel_readme_btn {
                let tab = &mut self.tabs[tab_idx];
                tab.readme_text = tab.original_readme_text.clone();
            }

            if save_readme_btn {
                if let Some(path) = readme_path {
                    let text = readme.clone();
                    // Detect encoding based on original_readme_text ? For simplicity, try writing as bytes if Japanese encoding, or UTF-8
                    // In Copaiba NEO we map encoding, but saving as UTF-8 is usually safer unless strictly handled.
                    // We'll just write as UTF-8 bytes to the specific path.
                    if std::fs::write(&path, &text).is_ok() {
                        let tab = &mut self.tabs[tab_idx];
                        tab.original_readme_text = text;
                        self.ui.toast_manager.success("Readme salvo com sucesso!");
                    } else {
                        self.ui.toast_manager.error(format!("Erro ao salvar {}", path.display()));
                    }
                }
            }
        });
    }
}
