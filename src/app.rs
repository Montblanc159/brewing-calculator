use eframe::*;
use egui::*;
use serde::*;

const DEFAULT_SPACING: f32 = 8.0;
const ERROR_COLOR: Color32 = Color32::from_rgb(255, 70, 70);

#[derive(Deserialize, Serialize, Default)]
struct Fermentecible {
    name: String,
    extract: f32,
    humidity: f32,
    ebc: u8,
    ratio: u8,
    weight: f32,
    mcu: f32,
}

#[derive(Deserialize, Serialize, Default)]
struct Ferment {
    name: String,
    attenuation: u8,
    pitch_rate: f32,
}

#[derive(Deserialize, Serialize, Default)]
struct Hop {
    name: String,
    alpha_acids: f32,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BrewingCalcApp {
    // #[serde(skip)] // This how you opt-out of serialization of a field
    name: String,
    style: String,
    abv: f32,
    ebc: u8,
    ibu: u8,
    original_gravity: f32,
    final_gravity: f32,
    efficiency: u8,
    batch_size: u16,
    mash_water_ratio: f32,
    evaporation_rate: f32,
    hops: Vec<Hop>,
    fermentecibles: Vec<Fermentecible>,
    ferments: Vec<Ferment>,
    mash_water_vol: f32,
    post_mash_water_vol: f32,
    sparge_water_vol: f32,
    pre_ebullition_water_vol: f32,
}

impl Default for BrewingCalcApp {
    fn default() -> Self {
        Self {
            name: String::from(""),
            style: String::from(""),
            abv: 5.0,
            ebc: 5,
            ibu: 10,
            original_gravity: 12.0,
            final_gravity: 2.0,
            efficiency: 80,
            mash_water_ratio: 3.0,
            evaporation_rate: 10.0,
            batch_size: 20,
            hops: vec![],
            fermentecibles: vec![],
            ferments: vec![],
            mash_water_vol: 0.0,
            post_mash_water_vol: 0.0,
            pre_ebullition_water_vol: 0.0,
            sparge_water_vol: 0.0,
        }
    }
}

impl BrewingCalcApp {
    /// Called once before the first frame.
    pub fn new(cc: &CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return get_value(storage, APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl App for BrewingCalcApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                        }
                    });
                    ui.add_space(DEFAULT_SPACING * 2.0);
                }

                widgets::global_theme_preference_buttons(ui);
            });
        });

        SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.label("Hello World!");
        });

        CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Recette");

            ui.separator();

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Nom : ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Style : ");
                ui.text_edit_singleline(&mut self.style);
            });

            ui.add_space(DEFAULT_SPACING);

            ui.label(format!("Alcool (%) : {}", self.abv));

            ui.add_space(DEFAULT_SPACING);

            ui.label(format!("EBC : {}", self.ebc));

            ui.add_space(DEFAULT_SPACING);

            ui.label(format!("IBU : {}", self.ibu));

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Densité initiale (°P) : ");
                ui.add(Slider::new(&mut self.original_gravity, 0.0..=25.0));
            });

            ui.add_space(DEFAULT_SPACING);

            ui.label(format!("Densité finale (°P): {}", self.final_gravity));

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Efficacité (%): ");
                ui.add(Slider::new(&mut self.efficiency, 0..=100));
            });

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Ratio d'eau à l'empâtage (L/kg): ");
                ui.add(Slider::new(&mut self.mash_water_ratio, 0.0..=10.0));
            });

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Taux d'évaporation (%): ");
                ui.add(Slider::new(&mut self.evaporation_rate, 0.0..=10.0));
            });

            ui.add_space(DEFAULT_SPACING);

            ui.label(format!(
                "Volume d'eau à l'empatage (L): {}",
                self.mash_water_vol
            ));

            ui.add_space(DEFAULT_SPACING);

            ui.label(format!(
                "Volume d'eau de rinçage (L): {}",
                self.sparge_water_vol
            ));

            ui.add_space(DEFAULT_SPACING);

            ui.label(format!(
                "Volume d'eau pré-ébullition (L): {}",
                self.pre_ebullition_water_vol
            ));

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Volume (L): ");
                ui.add(Slider::new(&mut self.batch_size, 0..=30000));
            });

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Ferments");
                if ui.button("+").clicked() {
                    self.ferments.push(Ferment {
                        ..Default::default()
                    });
                };

                if ui.button("-").clicked() {
                    self.ferments.pop();
                };
            });

            ui.add_space(DEFAULT_SPACING);

            for index in 0..self.ferments.len() {
                ferment_ui(ui, &mut self.ferments, index);
            }

            // This is an arbitrary choice to handle cofermentations, has to be enhanced
            let max_attenuation = self
                .ferments
                .iter()
                .map(|ferment| ferment.attenuation)
                .max()
                .unwrap_or(0);

            self.final_gravity =
                compute_final_gravity(self.original_gravity, max_attenuation as f32);

            self.abv = compute_abv(self.original_gravity, self.final_gravity);

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Fermentécibles");
                if ui.button("+").clicked() {
                    self.fermentecibles.push(Fermentecible {
                        ..Default::default()
                    })
                };

                if ui.button("-").clicked() {
                    self.fermentecibles.pop();
                };
            });

            ui.add_space(DEFAULT_SPACING);

            for index in 0..self.fermentecibles.len() {
                fermentecible_ui(ui, &mut self.fermentecibles, index);

                self.fermentecibles[index].weight = compute_grain_bill(
                    compute_total_extract(self.original_gravity),
                    self.efficiency,
                    self.batch_size,
                    &self.fermentecibles[index],
                );

                self.fermentecibles[index].mcu = compute_mcu(
                    self.fermentecibles[index].ebc,
                    self.fermentecibles[index].weight,
                    self.batch_size,
                );
            }

            let mut weights = vec![];
            let mut mcus = vec![];
            let mut ratios = vec![];
            for fermentecible in &self.fermentecibles {
                ratios.push(fermentecible.ratio);
                weights.push(fermentecible.weight);
                mcus.push(fermentecible.mcu);
            }

            self.ebc = compute_ebc(mcus.iter().sum());

            if self.fermentecibles.len() > 0 && check_fermentecible_ratio(ratios) {
                ui.colored_label(
                    ERROR_COLOR,
                    "Problème de ratios : leur somme doit être égal à 100",
                );
                ui.add_space(DEFAULT_SPACING);
            }

            self.mash_water_vol =
                compute_mash_water_vol(weights.iter().sum(), self.mash_water_ratio);

            self.post_mash_water_vol =
                compute_post_mash_water_vol(self.mash_water_vol, weights.iter().sum());

            self.sparge_water_vol = compute_sparge_water_vol(
                self.batch_size,
                self.evaporation_rate,
                self.post_mash_water_vol,
            );

            self.pre_ebullition_water_vol =
                compute_pre_ebullition_water_vol(self.sparge_water_vol, self.post_mash_water_vol);

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Houblons");
                if ui.button("+").clicked() {
                    self.hops.push(Hop {
                        ..Default::default()
                    })
                };

                if ui.button("-").clicked() {
                    self.hops.pop();
                };
            });

            ui.add_space(DEFAULT_SPACING);

            for index in 0..self.hops.len() {
                hop_ui(ui, &mut self.hops, index);
            }

            ui.separator();
            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

fn ferment_ui(ui: &mut Ui, ferments: &mut Vec<Ferment>, index: usize) {
    ui.horizontal(|ui| {
        let ferment = &mut ferments[index];

        ui.text_edit_singleline(&mut ferment.name);
        ui.label("Atténuation (%)");
        ui.add(Slider::new(&mut ferment.attenuation, 0..=100));
        ui.label("Taux d'ensemencement (g/L)");
        ui.add(Slider::new(&mut ferment.pitch_rate, 0.0..=30.0));
    });
    ui.add_space(DEFAULT_SPACING);
}

fn fermentecible_ui(ui: &mut Ui, fermentecibles: &mut Vec<Fermentecible>, index: usize) {
    ui.horizontal(|ui| {
        let fermentecible = &mut fermentecibles[index];

        ui.text_edit_singleline(&mut fermentecible.name);
        ui.label("Extrait (%)");
        ui.add(Slider::new(&mut fermentecible.extract, 0.0..=100.0));
        ui.label("Humidité (g/L)");
        ui.add(Slider::new(&mut fermentecible.humidity, 0.0..=100.0));
        ui.label("EBC");
        ui.add(Slider::new(&mut fermentecible.ebc, 0..=150));
        ui.label("Ratio (%)");
        ui.add(Slider::new(&mut fermentecible.ratio, 0..=100));
        ui.label(format!("Poid : {} g", fermentecible.weight));
    });
    ui.add_space(DEFAULT_SPACING);
}

fn hop_ui(ui: &mut Ui, hops: &mut Vec<Hop>, index: usize) {
    ui.horizontal(|ui| {
        let hop = &mut hops[index];

        ui.text_edit_singleline(&mut hop.name);
        ui.label("Acide alpha (%)");
        ui.add(Slider::new(&mut hop.alpha_acids, 0.0..=100.0));
        if ui.button("Supprimer").clicked() {
            hops.remove(index);
        }
    });
    ui.add_space(DEFAULT_SPACING);
}

fn compute_abv(og: f32, fg: f32) -> f32 {
    (og - fg) * 0.5
}

fn check_fermentecible_ratio(ratios: Vec<u8>) -> bool {
    ratios.iter().sum::<u8>() != 100
}

// g/L
fn compute_total_extract(og: f32) -> f32 {
    (0.9974 / ((1.0 / og) - 0.00382) + 0.01) * 10.0
}

fn compute_grain_bill(
    total_extract: f32,
    efficiency: u8,
    batch_size: u16,
    fermentecible: &Fermentecible,
) -> f32 {
    let f_extract =
        (total_extract * (fermentecible.ratio as f32 / 100.0)) / (efficiency as f32 / 100.0);

    (f_extract / (fermentecible.extract / 100.0)) / (1.0 - (fermentecible.humidity / 100.0))
        * batch_size as f32
}

fn compute_final_gravity(og: f32, attenuation: f32) -> f32 {
    og - og * (attenuation / 100.0)
}

fn compute_mash_water_vol(grain_weight: f32, water_ratio: f32) -> f32 {
    grain_weight / 1000.0 * water_ratio
}

fn compute_post_mash_water_vol(mash_water_vol: f32, grain_weight: f32) -> f32 {
    mash_water_vol - ((grain_weight / 1000.0) * 0.8)
}

fn compute_sparge_water_vol(
    batch_size: u16,
    evaporation_rate: f32,
    post_mash_water_vol: f32,
) -> f32 {
    (batch_size as f32 + (batch_size as f32 * (evaporation_rate as f32 / 100.0)))
        - post_mash_water_vol
}

fn compute_pre_ebullition_water_vol(sparge_water_vol: f32, post_mash_water_vol: f32) -> f32 {
    sparge_water_vol + post_mash_water_vol
}

fn compute_cell_count(og: f32, batch_size: u16) -> f32 {
    1_000_000.0 * (batch_size as f32 * 1000.0) * og
}

fn compute_mcu(ebc: u8, grain_weight: f32, batch_size: u16) -> f32 {
    (4.23 * (ebc as f32) * (grain_weight / 1000.0)) / batch_size as f32
}

fn compute_ebc(total_mcu: f32) -> u8 {
    (2.939 * total_mcu.powf(0.6859)) as u8
}

fn compute_bugu(ibu: u8, og: f32) {
    ibu as f32 / convert_plato_to_sg(og);
}

fn convert_plato_to_sg(plato: f32) -> f32 {
    1.0 + (plato / (258.6 - ((plato / 258.2) * 227.1)))
}

fn convert_sg_to_plato(sg: f32) -> f32 {
    (-1.0 * 616.868) + (1111.14 * sg) - (630.272 * sg.powf(2.0)) + (135.997 * sg.powf(3.0))
}
