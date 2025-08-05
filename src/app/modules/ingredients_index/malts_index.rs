use crate::app::modules::ui_defaults::*;
use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
pub const JSON_PATH: &str = "src/app/modules/ingredients_index/assets/malts.json";

#[cfg(target_arch = "wasm32")]
pub const JSON_FILE: &[u8; 44049] = include_bytes!("assets/malts.json");

#[derive(Deserialize, Serialize, Default)]
pub struct MaltIng {
    #[serde(skip)]
    opened: bool,
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub ebc_min: Option<f32>,
    pub ebc_max: Option<f32>,
    pub maltster: Option<String>,
    pub ratio: Option<u8>,
    pub grain_yield: Option<f32>,
    pub moisture: Option<f32>,
    pub diastatic_power: Option<u32>,
    pub kolbach_index: Option<u8>,
    pub total_nitrogen: Option<f32>,
    pub total_protein: Option<f32>,
    pub snr: Option<u8>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct MaltsIndex {
    pub malts: Vec<MaltIng>,
    opened: bool,
}

impl MaltsIndex {
    #[cfg(target_arch = "wasm32")]
    pub fn parse_file() -> String {
        String::from_utf8_lossy(&JSON_FILE[..]).to_string()
    }

    pub fn new(malts: Vec<MaltIng>) -> Self {
        Self {
            malts,
            ..Default::default()
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        if ui.button("Liste de malts").clicked() {
            self.opened = !self.opened;
        }

        Window::new("Liste de malts")
            .default_size([400., 400.])
            .open(&mut self.opened)
            .show(ui.ctx(), |ui| {
                ScrollArea::vertical()
                    .id_salt("liste-malts")
                    .show(ui, |ui| {
                        for malt in &mut self.malts {
                            if ui.button(&malt.name).clicked() {
                                malt.opened = !malt.opened;
                            }

                            Window::new(&malt.name)
                                .default_size([400., 400.])
                                .open(&mut malt.opened)
                                .show(ui.ctx(), |ui| {
                                    ScrollArea::vertical().id_salt(&malt.name).show(ui, |ui| {
                                        if let Some(description) = &malt.description {
                                            ui.label(description);
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(maltster) = &malt.maltster {
                                            ui.label(format!("Maltster: {}", maltster));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(ratio) = &malt.ratio {
                                            ui.label(format!("Ratio: {}", ratio));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(grain_yield) = &malt.grain_yield {
                                            ui.label(format!("Grain yield: {}", grain_yield));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(moisture) = &malt.moisture {
                                            ui.label(format!("Moisture: {}", moisture));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(diastatic_power) = &malt.diastatic_power {
                                            ui.label(format!(
                                                "Diastatic power: {}",
                                                diastatic_power
                                            ));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(kolbach_index) = &malt.kolbach_index {
                                            ui.label(format!("Kolbach index: {}", kolbach_index));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(total_nitrogen) = &malt.total_nitrogen {
                                            ui.label(format!("Total nitrogen: {}", total_nitrogen));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(total_protein) = &malt.total_protein {
                                            ui.label(format!("Total protein: {}", total_protein));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(snr) = &malt.snr {
                                            ui.label(format!("SNR: {}", snr));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        ui.horizontal(|ui| {
                                            ui.label("EBC");

                                            if let Some(ebc_min) = &malt.ebc_min {
                                                ui.label(format!("{}", ebc_min));
                                            };

                                            ui.label("-");

                                            if let Some(ebc_max) = &malt.ebc_max {
                                                ui.label(format!("{}", ebc_max));
                                            };
                                        });
                                    });
                                });
                        }
                    });
            });
    }
}
