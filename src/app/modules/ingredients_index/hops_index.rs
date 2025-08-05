use crate::app::modules::ui_defaults::*;
use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
pub const JSON_PATH: &str = "src/app/modules/ingredients_index/assets/hops.json";

#[cfg(target_arch = "wasm32")]
pub const JSON_FILE: &[u8; 66023] = include_bytes!("assets/hops.json");

#[derive(Deserialize, Serialize, Default)]
pub struct HopIng {
    #[serde(skip)]
    opened: bool,
    id: Option<i64>,
    brewing_usage: Option<String>,
    name: String,
    aroma: Option<String>,
    pedigree: Option<String>,
    alpha_min: Option<f32>,
    alpha_max: Option<f32>,
    beta_min: Option<f32>,
    beta_max: Option<f32>,
    cohumulone_min: Option<f32>,
    cohumulone_max: Option<f32>,
    info: Option<String>,
    styles: Option<String>,
    total_oil_min: Option<f32>,
    total_oil_max: Option<f32>,
    trade: Option<String>,
}

#[derive(Deserialize, Serialize, Default)]
pub struct HopsIndex {
    pub hops: Vec<HopIng>,
    opened: bool,
}

impl HopsIndex {
    #[cfg(target_arch = "wasm32")]
    pub fn parse_file() -> String {
        String::from_utf8_lossy(&JSON_FILE[..]).to_string()
    }

    pub fn new(hops: Vec<HopIng>) -> Self {
        Self {
            hops,
            ..Default::default()
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        if ui.button("Liste de houblons").clicked() {
            self.opened = !self.opened;
        }

        Window::new("Liste de houblons")
            .default_size([400., 400.])
            .open(&mut self.opened)
            .show(ui.ctx(), |ui| {
                ScrollArea::vertical()
                    .id_salt("liste-houblons")
                    .show(ui, |ui| {
                        for hop in &mut self.hops {
                            if ui.button(&hop.name).clicked() {
                                hop.opened = !hop.opened;
                            }

                            Window::new(&hop.name)
                                .default_size([400., 400.])
                                .open(&mut hop.opened)
                                .show(ui.ctx(), |ui| {
                                    ScrollArea::vertical().id_salt(&hop.name).show(ui, |ui| {
                                        if let Some(info) = &hop.info {
                                            ui.label(info);
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(brewing_usage) = &hop.brewing_usage {
                                            ui.label(format!("Brewing usage: {brewing_usage}"));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(aroma) = &hop.aroma {
                                            ui.label(format!("Aroma: {aroma}"));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(pedigree) = &hop.pedigree {
                                            ui.label(format!("Pedigree: {pedigree}"));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        ui.horizontal(|ui| {
                                            ui.label("Alpha");

                                            if let Some(alpha_min) = &hop.alpha_min {
                                                ui.label(format!("{alpha_min}"));
                                            };

                                            ui.label("-");

                                            if let Some(alpha_max) = &hop.alpha_max {
                                                ui.label(format!("{alpha_max}"));
                                            };
                                        });

                                        ui.add_space(DEFAULT_SPACING);

                                        ui.horizontal(|ui| {
                                            ui.label("Beta");

                                            if let Some(beta_min) = &hop.beta_min {
                                                ui.label(format!("{beta_min}"));
                                            };

                                            ui.label("-");

                                            if let Some(beta_max) = &hop.beta_max {
                                                ui.label(format!("{beta_max}"));
                                            };
                                        });

                                        ui.add_space(DEFAULT_SPACING);

                                        ui.horizontal(|ui| {
                                            ui.label("Cohumulone");

                                            if let Some(cohumulone_min) = &hop.cohumulone_min {
                                                ui.label(format!("{cohumulone_min}"));
                                            };

                                            ui.label("-");

                                            if let Some(cohumulone_max) = &hop.cohumulone_max {
                                                ui.label(format!("{cohumulone_max}"));
                                            };
                                        });

                                        ui.add_space(DEFAULT_SPACING);

                                        ui.horizontal(|ui| {
                                            ui.label("Total oil");

                                            if let Some(total_oil_min) = &hop.total_oil_min {
                                                ui.label(format!("{total_oil_min}"));
                                            };

                                            ui.label("-");

                                            if let Some(total_oil_max) = &hop.total_oil_max {
                                                ui.label(format!("{total_oil_max}"));
                                            };
                                        });

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(styles) = &hop.styles {
                                            ui.label(format!("Styles: {styles}"));
                                        };

                                        ui.add_space(DEFAULT_SPACING);

                                        if let Some(trade) = &hop.trade {
                                            ui.label(format!("Trade: {trade}"));
                                        };
                                    });
                                });
                        }
                    });
            });
    }
}
