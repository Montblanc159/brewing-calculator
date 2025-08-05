use crate::app::modules::ui_defaults::*;
use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
pub const JSON_PATH: &str = "src/app/modules/ingredients_index/assets/yeasts.json";

#[cfg(target_arch = "wasm32")]
pub const JSON_FILE: &[u8; 6089] = include_bytes!("assets/yeasts.json");

#[derive(Deserialize, Serialize, Default)]
pub struct YeastIng {
    #[serde(skip)]
    opened: bool,
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub temp_min: Option<u32>,
    pub temp_max: Option<u32>,
    pub lab: Option<String>,
    pub attenuation_min: Option<u32>,
    pub attenuation_max: Option<u32>,
    pub form: Option<String>,
    pub flocculation: Option<String>,
    pub styles: Option<String>,
    pub alcohol_tolerance: Option<f32>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct YeastsIndex {
    pub yeasts: Vec<YeastIng>,
    opened: bool,
}

impl YeastsIndex {
    #[cfg(target_arch = "wasm32")]
    pub fn parse_file() -> String {
        String::from_utf8_lossy(&JSON_FILE[..]).to_string()
    }

    pub fn new(yeasts: Vec<YeastIng>) -> Self {
        Self {
            yeasts,
            ..Default::default()
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        if ui.button("Liste de ferments").clicked() {
            self.opened = !self.opened;
        }

        Window::new("Liste de ferments")
            .default_size([400., 400.])
            .open(&mut self.opened)
            .show(ui.ctx(), |ui| {
                ScrollArea::vertical()
                    .id_salt("liste-yeasts")
                    .show(ui, |ui| {
                        for yeast in &mut self.yeasts {
                            if ui.button(&yeast.name).clicked() {
                                yeast.opened = !yeast.opened;
                            }

                            Window::new(&yeast.name)
                                .default_size([400., 400.])
                                .open(&mut yeast.opened)
                                .show(ui.ctx(), |ui| {
                                    ScrollArea::vertical().id_salt(&yeast.name).show(ui, |ui| {
                                        if let Some(description) = &yeast.description {
                                            ui.label(description);
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(styles) = &yeast.styles {
                                            ui.label(format!("Styles: {}", styles));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(alcohol_tolerance) = &yeast.alcohol_tolerance {
                                            ui.label(format!(
                                                "Alcohol tolerance: {}",
                                                alcohol_tolerance
                                            ));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(flocculation) = &yeast.flocculation {
                                            ui.label(format!("Flocculation: {}", flocculation));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        ui.horizontal(|ui| {
                                            ui.label("Attenuation");

                                            if let Some(attenuation_min) = &yeast.attenuation_min {
                                                ui.label(format!("{}", attenuation_min));
                                            };

                                            ui.label("-");

                                            if let Some(attenuation_max) = &yeast.attenuation_max {
                                                ui.label(format!("{}", attenuation_max));
                                            };
                                        });

                                        ui.horizontal(|ui| {
                                            ui.label("Temperature");

                                            if let Some(temp_min) = &yeast.temp_min {
                                                ui.label(format!("{}", temp_min));
                                            };

                                            ui.label("-");

                                            if let Some(temp_max) = &yeast.temp_max {
                                                ui.label(format!("{}", temp_max));
                                            };
                                        });

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(lab) = &yeast.lab {
                                            ui.label(format!("Lab: {}", lab));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(form) = &yeast.form {
                                            ui.label(format!("Form: {}", form));
                                        };
                                    });
                                });
                        }
                    });
            });
    }
}
