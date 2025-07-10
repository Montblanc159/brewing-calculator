use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[cfg(not(target_arch = "wasm32"))]
use std::fs;
use std::process::exit;

const DEFAULT_SPACING: f32 = 8.0;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct BJCPStyleIndex {
    beer_styles: BeerStyles,
    prompt: String,
    result: Vec<BeerStyle>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BeerStyles {
    styles: Vec<BeerStyle>,
}

#[derive(Deserialize, Serialize, Clone)]
struct BeerStyle {
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
    opened: bool,
}

impl Default for BJCPStyleIndex {
    fn default() -> Self {
        Self {
            beer_styles: BeerStyles { styles: vec![] },
            prompt: "".into(),
            result: vec![],
        }
    }
}

impl BJCPStyleIndex {
    pub fn new(beer_styles: BeerStyles) -> Self {
        Self {
            beer_styles,
            ..Default::default()
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        egui::Frame::new().show(ui, |ui| {
            ui.label("Recherche par style (eng)");

            if ui.text_edit_singleline(&mut self.prompt).changed() {
                self.result = search_styles(&self.prompt, &self.beer_styles.styles)
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
                    for style in &mut self.beer_styles.styles {
                        style_ui(style, ui.ctx());
                    }

                    for style in self.beer_styles.styles.iter_mut() {
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
        .default_size([250., 400.])
        .open(&mut style.opened)
        .show(ctx, |ui| {
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
                };

                ui.label("-");

                if let Some(og_max) = &style.ogmax {
                    ui.label(og_max);
                };
            });

            ui.horizontal(|ui| {
                ui.label("Final gravity");

                if let Some(fg_min) = &style.fgmin {
                    ui.label(fg_min);
                };

                ui.label("-");

                if let Some(fg_max) = &style.fgmax {
                    ui.label(fg_max);
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
                };

                ui.label("-");

                if let Some(srm_max) = &style.srmmax {
                    ui.label(srm_max);
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
}

pub fn parse_json() -> BeerStyles {
    // Use a `match` block to return the
    // file `contents` as a `Data struct: Ok(d)`
    // or handle any `errors: Err(_)`.
    let data: BeerStyles = match serde_json::from_str(&fetch_json()) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `beer_styles.json`");
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    data
}

#[cfg(not(target_arch = "wasm32"))]
pub fn fetch_json() -> String {
    // Variable that holds the filename as a `&str`.
    let filename = "src/app/modules/bjcp_style_index/assets/beer_styles.json";

    // Read the contents of the file using a `match` block
    // to return the `data: Ok(c)` as a `String`
    // or handle any `errors: Err(_)`.
    match fs::read_to_string(filename) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(e) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{filename}`: {e}");
            // Exit the program with exit code `1`.
            exit(1);
        }
    }
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
pub fn fetch_json() -> String {
    String::from_utf8_lossy(&include_bytes!("assets/beer_styles.json")[..]).to_string()
}
