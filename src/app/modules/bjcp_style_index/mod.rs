use crate::app::modules::math::{convert_sg_to_plato, convert_srm_to_ebc};
use crate::app::modules::ui_defaults::*;

use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
pub const JSON_PATH: &str = "src/app/modules/bjcp_style_index/assets/beer_styles.json";

#[cfg(target_arch = "wasm32")]
pub const JSON_FILE: &[u8; 395720] = include_bytes!("assets/beer_styles.json");

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct BJCPStyleIndex {
    beer_styles: Vec<BeerStyle>,
    prompt: String,
    result: Vec<BeerStyle>,
}

#[derive(Deserialize, Serialize, Default, Clone)]
pub struct BeerStyle {
    name: String,
    number: Option<String>,
    category: Option<String>,
    categorynumber: Option<String>,
    overallimpression: Option<String>,
    aroma: Option<String>,
    appearance: Option<String>,
    flavor: Option<String>,
    mouthfeel: Option<String>,
    comments: Option<String>,
    history: Option<String>,
    characteristicingredients: Option<String>,
    stylecomparison: Option<String>,
    strengthclassifications: Option<String>,
    currentlydefinedtypes: Option<String>,
    entryinstructions: Option<String>,
    ibumin: Option<String>,
    ibumax: Option<String>,
    ogmin: Option<String>,
    ogmax: Option<String>,
    fgmin: Option<String>,
    fgmax: Option<String>,
    abvmin: Option<String>,
    abvmax: Option<String>,
    srmmin: Option<String>,
    srmmax: Option<String>,
    commercialexamples: Option<String>,
    tags: Option<String>,
    #[serde(skip)]
    opened: bool,
}

impl Default for BJCPStyleIndex {
    fn default() -> Self {
        Self {
            beer_styles: vec![],
            prompt: "".into(),
            result: vec![],
        }
    }
}

impl BJCPStyleIndex {
    #[cfg(target_arch = "wasm32")]
    pub fn parse_file() -> String {
        String::from_utf8_lossy(&JSON_FILE[..]).to_string()
    }

    pub fn new(beer_styles: Vec<BeerStyle>) -> Self {
        Self {
            beer_styles,
            ..Default::default()
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        egui::Frame::new().show(ui, |ui| {
            ui.label("Recherche par style (eng)");

            if ui.text_edit_singleline(&mut self.prompt).changed() {
                self.result = search_styles(&self.prompt, &self.beer_styles)
            };

            ui.add_space(DEFAULT_SPACING);

            ScrollArea::vertical().show(ui, |ui| {
                if !self.prompt.is_empty() {
                    for style in &mut self.result {
                        style_ui(style, ui.ctx());
                    }

                    for style in self.result.iter_mut() {
                        if ui.button(&style.name).clicked() {
                            style.opened = !style.opened;
                        }
                    }
                } else {
                    for style in &mut self.beer_styles {
                        style_ui(style, ui.ctx());
                    }

                    for style in self.beer_styles.iter_mut() {
                        if ui.button(&style.name).clicked() {
                            style.opened = !style.opened;
                        }
                    }
                }
            });
        });
    }
}

fn search_styles(prompt: &str, styles: &[BeerStyle]) -> Vec<BeerStyle> {
    styles
        .iter()
        .filter(|&style| style.name.to_lowercase().contains(&prompt.to_lowercase()))
        .cloned()
        .collect()
}

fn style_ui(style: &mut BeerStyle, ctx: &Context) {
    Window::new(&style.name)
        .default_size([400., 400.])
        .open(&mut style.opened)
        .show(ctx, |ui| {
            ScrollArea::vertical().id_salt(&style.name).show(ui, |ui| {
                if let Some(number) = &style.number {
                    ui.label(format!("BJCP Number: {number}"));
                };

                ui.add_space(DEFAULT_SPACING);

                if let Some(category) = &style.category {
                    ui.label(format!("Category: {category}"));
                };

                ui.add_space(DEFAULT_SPACING);

                ui.horizontal(|ui| {
                    ui.label("IBU");

                    if let Some(ibu_min) = &style.ibumin {
                        ui.label(ibu_min);
                    };

                    ui.label("-");

                    if let Some(ibu_max) = &style.ibumax {
                        ui.label(ibu_max);
                    };
                });

                ui.horizontal(|ui| {
                    ui.label("Original gravity");

                    if let Some(og_min) = &style.ogmin {
                        ui.label(og_min);

                        if let Ok(og_min) = og_min.parse::<f32>() {
                            ui.label(format!("({:.1} °P)", convert_sg_to_plato(og_min)));
                        }
                    };

                    ui.label("-");

                    if let Some(og_max) = &style.ogmax {
                        ui.label(og_max);

                        if let Ok(og_max) = og_max.parse::<f32>() {
                            ui.label(format!("({:.1} °P)", convert_sg_to_plato(og_max)));
                        }
                    };
                });

                ui.horizontal(|ui| {
                    ui.label("Final gravity");

                    if let Some(fg_min) = &style.fgmin {
                        ui.label(fg_min);

                        if let Ok(fg_min) = fg_min.parse::<f32>() {
                            ui.label(format!("({:.1} °P)", convert_sg_to_plato(fg_min)));
                        }
                    };

                    ui.label("-");

                    if let Some(fg_max) = &style.fgmax {
                        ui.label(fg_max);

                        if let Ok(fg_max) = fg_max.parse::<f32>() {
                            ui.label(format!("({:.1} °P)", convert_sg_to_plato(fg_max)));
                        }
                    };
                });

                ui.horizontal(|ui| {
                    ui.label("ABV");

                    if let Some(abv_min) = &style.abvmin {
                        ui.label(abv_min);
                    };

                    ui.label("-");

                    if let Some(abv_max) = &style.abvmax {
                        ui.label(abv_max);
                    };
                });

                ui.horizontal(|ui| {
                    ui.label("SRM");

                    if let Some(srm_min) = &style.srmmin {
                        ui.label(srm_min);

                        if let Ok(srm_min) = srm_min.parse::<f32>() {
                            ui.label(format!("({:.1} EBC)", convert_srm_to_ebc(srm_min)));
                        }
                    };

                    ui.label("-");

                    if let Some(srm_max) = &style.srmmax {
                        ui.label(srm_max);

                        if let Ok(srm_max) = srm_max.parse::<f32>() {
                            ui.label(format!("({:.1} EBC)", convert_srm_to_ebc(srm_max)));
                        }
                    };
                });

                ui.add_space(DEFAULT_SPACING);

                if let Some(overall_impression) = &style.overallimpression {
                    ui.heading("Overall impression:");
                    ui.label(overall_impression);
                };

                ui.add_space(DEFAULT_SPACING);

                if let Some(appearance) = &style.appearance {
                    ui.heading("Appearance:");
                    ui.label(appearance);
                };

                ui.add_space(DEFAULT_SPACING);

                if let Some(aroma) = &style.aroma {
                    ui.heading("Aroma:");
                    ui.label(aroma);
                };

                ui.add_space(DEFAULT_SPACING);

                if let Some(flavor) = &style.flavor {
                    ui.heading("Flavor:");
                    ui.label(flavor);
                };

                ui.add_space(DEFAULT_SPACING);

                if let Some(mouthfeel) = &style.mouthfeel {
                    ui.heading("Mouthfeel:");
                    ui.label(mouthfeel);
                };

                ui.add_space(DEFAULT_SPACING);

                if let Some(history) = &style.history {
                    ui.heading("History:");
                    ui.label(history);
                };

                ui.add_space(DEFAULT_SPACING);

                if let Some(characteristic_ingredients) = &style.characteristicingredients {
                    ui.heading("Characteristic ingredients:");
                    ui.label(characteristic_ingredients);
                };

                ui.add_space(DEFAULT_SPACING);

                if let Some(style_comparison) = &style.stylecomparison {
                    ui.heading("Style comparison:");
                    ui.label(style_comparison);
                };

                ui.add_space(DEFAULT_SPACING);

                if let Some(commercial_examples) = &style.commercialexamples {
                    ui.heading("Commercial examples:");
                    ui.label(commercial_examples);
                };
            });
        });
}
