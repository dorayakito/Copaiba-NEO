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
            let mut save_pmap_btn = false;
            let mut cancel_pmap_btn = false;
            
            let (char_tex, char_name, oto_dir, mut readme, original_readme, license, readme_path, root_path, pmap, original_pmap) = {
                let tab = &self.tabs[tab_idx];
                (tab.character_texture.as_ref().map(|t| t.id()), tab.character_name.clone(), tab.oto_dir.clone(), tab.readme_text.clone(), tab.original_readme_text.clone(), tab.license_text.clone(), tab.readme_path.clone(), tab.root_path.clone(), tab.prefix_map.clone(), tab.original_prefix_map.clone())
            };

            // ── Compact Header Row ───────────────────────────────────────────
            ui.add_space(2.0);
            crate::app::layout::horizontal(ui, self.is_rtl(), |ui| {
                // Character Image (36x36 compact)
                let (rect, _resp) = ui.allocate_at_least(Vec2::new(36.0, 36.0), egui::Sense::hover());
                if let Some(tex_id) = char_tex {
                    ui.painter().image(tex_id, rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), Color32::WHITE);
                } else {
                    ui.painter().rect_filled(rect, 6.0, Color32::from_rgb(30, 30, 46));
                    ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "👤", egui::FontId::proportional(16.0), Color32::GRAY);
                }

                ui.add_space(6.0);

                // Name + Path (vertical, compact)
                ui.vertical(|ui| {
                    let name = if char_name.is_empty() {
                        oto_dir.as_ref()
                            .and_then(|p: &PathBuf| p.file_name())
                            .map(|s: &std::ffi::OsStr| s.to_string_lossy().to_string())
                            .unwrap_or_else(|| "Voicebank".to_string())
                    } else { 
                        char_name
                    };
                    ui.label(RichText::new(name).strong().size(13.0));
                    ui.label(RichText::new(oto_dir.as_ref().map(|p: &PathBuf| p.to_string_lossy().to_string()).unwrap_or_default()).color(ui.visuals().weak_text_color()).size(8.0));
                });

                ui.add_space(8.0);

                // Toggle buttons for readme / prefix.map / license
                ui.horizontal(|ui| {
                    if readme_path.is_some() {
                        let label = if self.ui.show_readme_panel { "📄 ▾" } else { "📄 ▸" };
                        if ui.add(egui::Button::new(RichText::new(label).size(10.0)).small().fill(Color32::TRANSPARENT)).on_hover_text("Readme.txt").clicked() {
                            self.ui.show_readme_panel = !self.ui.show_readme_panel;
                        }
                    } else if root_path.is_some() {
                        if ui.add(egui::Button::new(RichText::new("➕📄").size(10.0)).small().fill(Color32::TRANSPARENT)).on_hover_text("Criar readme.txt").clicked() {
                            let root = root_path.as_ref().unwrap();
                            let new_path = root.join("readme.txt");
                            let initial_text = "Insira os detalhes do voicebank aqui...\n";
                            if std::fs::write(&new_path, initial_text).is_ok() {
                                let tab = &mut self.tabs[tab_idx];
                                tab.readme_path = Some(new_path);
                                tab.readme_text = initial_text.to_string();
                                tab.original_readme_text = initial_text.to_string();
                                self.ui.toast_manager.success("readme.txt criado!");
                            }
                        }
                    }

                    if !pmap.is_empty() {
                        let pmap_dirty = pmap != original_pmap;
                        let label = if self.ui.show_pmap_panel { "prefix.map ▾" } else { "prefix.map ▸" };
                        let color = if pmap_dirty { Color32::from_rgb(255, 200, 100) } else { Color32::from_rgb(160, 160, 180) };
                        if ui.add(egui::Button::new(RichText::new(label).size(10.0).color(color)).small().fill(Color32::TRANSPARENT)).clicked() {
                            self.ui.show_pmap_panel = !self.ui.show_pmap_panel;
                        }
                    }

                    if !license.is_empty() {
                        if ui.add(egui::Button::new(RichText::new("⚖").size(10.0)).small().fill(Color32::TRANSPARENT)).on_hover_text("License").clicked() {
                            self.ui.show_license = true;
                        }
                    }
                });

                // ── Right side: Resampler controls ───────────────────────────
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(8.0);
                    
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
            ui.add_space(2.0);

            // ── Collapsible Readme Panel ─────────────────────────────────────
            if self.ui.show_readme_panel && readme_path.is_some() {
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(RichText::new("📄 Readme.txt").size(10.0).color(Color32::from_rgb(180, 180, 200)));
                    if readme != original_readme {
                        if ui.button(RichText::new("💾 Salvar").color(Color32::from_rgb(100, 255, 100)).size(10.0)).clicked() {
                            save_readme_btn = true;
                        }
                        if ui.button(RichText::new("✖ Cancelar").color(Color32::from_rgb(255, 100, 100)).size(10.0)).clicked() {
                            cancel_readme_btn = true;
                        }
                    }
                });
                egui::ScrollArea::vertical().id_salt("readme_scroll").max_height(60.0).show(ui, |ui| {
                    ui.add(egui::TextEdit::multiline(&mut readme).desired_width(f32::INFINITY).font(egui::TextStyle::Small).margin(egui::Margin::same(2)));
                });
            }

            // ── Collapsible Prefix Map Panel ─────────────────────────────────
            if self.ui.show_pmap_panel && !pmap.is_empty() {
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label(RichText::new("prefix.map").size(10.0).color(Color32::from_rgb(180, 180, 200)));
                    ui.add_space(4.0);
                    if pmap != original_pmap {
                        if ui.button(RichText::new("💾").size(10.0).color(Color32::from_rgb(100, 255, 100))).on_hover_text("Salvar prefix.map").clicked() {
                            save_pmap_btn = true;
                        }
                        if ui.button(RichText::new("✖").size(10.0).color(Color32::from_rgb(255, 100, 100))).on_hover_text("Cancelar alterações").clicked() {
                            cancel_pmap_btn = true;
                        }
                    }
                    ui.add_space(8.0);
                    ui.add(egui::TextEdit::singleline(&mut self.pmap_batch_pre).hint_text("Pre").desired_width(32.0).font(egui::FontId::monospace(9.0)));
                    ui.add(egui::TextEdit::singleline(&mut self.pmap_batch_suf).hint_text("Suf").desired_width(32.0).font(egui::FontId::monospace(9.0)));
                    if ui.add(egui::Button::new(RichText::new("⚡").size(10.0)).small()).on_hover_text("Aplicar em lote").clicked() {
                        let b_pre = self.pmap_batch_pre.clone();
                        let b_suf = self.pmap_batch_suf.clone();
                        let tab = self.cur_mut();
                        let has_selection = tab.prefix_map.iter().any(|e| e.selected);
                        for e in &mut tab.prefix_map {
                            if !has_selection || e.selected {
                                if !b_pre.is_empty() { e.prefix = b_pre.clone(); }
                                if !b_suf.is_empty() { e.suffix = b_suf.clone(); }
                            }
                        }
                    }
                });

                let pmap_len = pmap.len();
                let table = egui_extras::TableBuilder::new(ui)
                    .striped(true)
                    .max_scroll_height(60.0)
                    .column(egui_extras::Column::auto().at_least(16.0))   // ✔
                    .column(egui_extras::Column::initial(36.0))            // Pitch
                    .column(egui_extras::Column::initial(50.0))            // Prefix
                    .column(egui_extras::Column::initial(50.0));           // Suffix

                table.header(14.0, |mut header| {
                    header.col(|ui| { ui.label(RichText::new("✔").size(9.0).strong()); });
                    header.col(|ui| { ui.label(RichText::new("Ton").size(9.0).strong()); });
                    header.col(|ui| { ui.label(RichText::new("Pre").size(9.0).strong()); });
                    header.col(|ui| { ui.label(RichText::new("Suf").size(9.0).strong()); });
                })
                .body(|body| {
                    body.rows(18.0, pmap_len, |mut row| {
                        let idx = row.index();
                        let tab = self.cur_mut();
                        if let Some(entry) = tab.prefix_map.get_mut(idx) {
                            row.col(|ui| { ui.checkbox(&mut entry.selected, ""); });
                            row.col(|ui| { ui.label(RichText::new(&entry.pitch).size(9.0)); });
                            row.col(|ui| { ui.add(egui::TextEdit::singleline(&mut entry.prefix).font(egui::FontId::monospace(9.0)).margin(egui::Margin::ZERO).frame(false)); });
                            row.col(|ui| { ui.add(egui::TextEdit::singleline(&mut entry.suffix).font(egui::FontId::monospace(9.0)).margin(egui::Margin::ZERO).frame(false)); });
                        }
                    });
                });
            }
            
            // ── Post-UI Update Logic ─────────────────────────────────────────
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
                    if std::fs::write(&path, &text).is_ok() {
                        let tab = &mut self.tabs[tab_idx];
                        tab.original_readme_text = text;
                        self.ui.toast_manager.success("Readme salvo com sucesso!");
                    } else {
                        self.ui.toast_manager.error(format!("Erro ao salvar {}", path.display()));
                    }
                }
            }

            if save_pmap_btn { self.save_prefix_map(); }
            if cancel_pmap_btn { self.cancel_prefix_map(); }
        });
    }
}
