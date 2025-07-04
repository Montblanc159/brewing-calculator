mod math;

use eframe::*;
use egui::*;
use math::*;
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
    addition_time: u8,
    utilization: f32,
    weight: f32,
    ibu: f32,
    ratio: u8,
}

#[derive(Deserialize, Serialize, Default)]
struct WhirlpoolHop {
    name: String,
    alpha_acids: f32,
    weight: f32,
    utilization: f32,
    ibu: f32,
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
    ibu: f32,
    bugu: f32,
    original_gravity: f32,
    final_gravity: f32,
    efficiency: u8,
    batch_size: u16,
    mash_water_ratio: f32,
    evaporation_rate: f32,
    hops: Vec<Hop>,
    whirlpool_hops: Vec<WhirlpoolHop>,
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
            ibu: 10.0,
            bugu: 0.0,
            original_gravity: 12.0,
            final_gravity: 2.0,
            efficiency: 80,
            mash_water_ratio: 3.0,
            evaporation_rate: 10.0,
            batch_size: 20,
            hops: vec![],
            whirlpool_hops: vec![],
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

        // SidePanel::left("my_left_panel").show(ctx, |ui| {
        //     ui.label("Hello World!");
        // });

        // Add a lot of widgets here.
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::horizontal().show(ui, |ui| {
                // The central panel the region left after adding TopPanel's and SidePanel's
                ui.heading("Broutille");

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

                ui.horizontal(|ui| {
                    ui.label("IBU : ");
                    ui.add(Slider::new(&mut self.ibu, 0.0..=150.0));
                });

                ui.add_space(DEFAULT_SPACING);

                ui.label(format!("BUGU : {}", self.bugu));

                self.bugu = compute_bugu(self.ibu, self.original_gravity);

                ui.add_space(DEFAULT_SPACING);

                ui.label(format!(
                    "Required cell count : {}",
                    compute_cell_count(self.original_gravity, self.batch_size)
                ));

                ui.add_space(DEFAULT_SPACING);

                ui.horizontal(|ui| {
                    ui.label("Densité initiale (°P) : ");
                    ui.add(Slider::new(&mut self.original_gravity, 0.0..=25.0));
                    ui.label(format!("{} G", convert_plato_to_sg(self.original_gravity)));
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

                for ferment in &mut self.ferments {
                    ferment_ui(ui, ferment);
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

                let mut weights = vec![];
                let mut mcus = vec![];
                let mut ratios = vec![];

                for fermentecible in &mut self.fermentecibles {
                    let total_extract: f32 = compute_total_extract(self.original_gravity);

                    let fermentecible_extractable = compute_per_malt_extractable(
                        total_extract,
                        fermentecible.ratio,
                        self.efficiency,
                    );

                    fermentecible.weight = compute_grain_bill(
                        self.batch_size,
                        fermentecible_extractable,
                        fermentecible.humidity,
                        fermentecible.extract,
                    );

                    fermentecible.mcu =
                        compute_mcu(fermentecible.ebc, fermentecible.weight, self.batch_size);

                    ratios.push(fermentecible.ratio);
                    weights.push(fermentecible.weight);
                    mcus.push(fermentecible.mcu);
                }

                self.ebc = compute_ebc(mcus.iter().sum());

                for fermentecible in &mut self.fermentecibles {
                    fermentecible_ui(ui, fermentecible);
                }

                if !self.fermentecibles.is_empty() && check_ratios(ratios) {
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

                self.pre_ebullition_water_vol = compute_pre_ebullition_water_vol(
                    self.sparge_water_vol,
                    self.post_mash_water_vol,
                );

                ui.add_space(DEFAULT_SPACING);

                ui.horizontal(|ui| {
                    ui.label("Houblons au whirlpool");
                    if ui.button("+").clicked() {
                        self.whirlpool_hops.push(WhirlpoolHop {
                            utilization: 0.12,
                            ..Default::default()
                        })
                    };

                    if ui.button("-").clicked() {
                        self.whirlpool_hops.pop();
                    };
                });

                ui.add_space(DEFAULT_SPACING);

                for hop in &mut self.whirlpool_hops {
                    whirlpool_hop_ui(ui, self.batch_size, hop);

                    hop.ibu = compute_ibu(
                        hop.utilization,
                        self.batch_size,
                        hop.alpha_acids,
                        hop.weight,
                        self.original_gravity,
                    );
                }

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

                let mut hop_ratios = vec![];
                for hop in &mut self.hops {
                    hop_ui(ui, hop);

                    hop.utilization =
                        compute_hop_utilization(self.original_gravity, hop.addition_time);

                    hop.weight = compute_hop_weight(
                        hop.utilization,
                        self.batch_size,
                        hop.alpha_acids,
                        self.whirlpool_hops
                            .iter()
                            .map(|w_hop| w_hop.ibu)
                            .sum::<f32>()
                            * (hop.ratio as f32 / 100.0),
                        self.original_gravity,
                    );

                    hop.ibu = compute_ibu(
                        hop.utilization,
                        self.batch_size,
                        hop.alpha_acids,
                        hop.weight,
                        self.original_gravity,
                    );

                    hop_ratios.push(hop.ratio)
                }

                if !self.hops.is_empty() && check_ratios(hop_ratios) {
                    ui.colored_label(
                        ERROR_COLOR,
                        "Problème de ratios : leur somme doit être égal à 100",
                    );
                    ui.add_space(DEFAULT_SPACING);
                }

                ui.separator();
                ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                    powered_by_egui_and_eframe(ui);
                    warn_if_debug_build(ui);
                });
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

fn ferment_ui(ui: &mut Ui, ferment: &mut Ferment) {
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut ferment.name);
        ui.label("Atténuation (%)");
        ui.add(Slider::new(&mut ferment.attenuation, 0..=100));
        ui.label("Taux d'ensemencement (g/L)");
        ui.add(Slider::new(&mut ferment.pitch_rate, 0.0..=30.0));
    });
    ui.add_space(DEFAULT_SPACING);
}

fn fermentecible_ui(ui: &mut Ui, fermentecible: &mut Fermentecible) {
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut fermentecible.name);
        ui.label("Extrait (%)");
        ui.add(Slider::new(&mut fermentecible.extract, 0.0..=100.0));
        ui.label("Humidité (%)");
        ui.add(Slider::new(&mut fermentecible.humidity, 0.0..=100.0));
        ui.label("EBC");
        ui.add(Slider::new(&mut fermentecible.ebc, 0..=150));
        ui.label("Ratio (%)");
        ui.add(Slider::new(&mut fermentecible.ratio, 0..=100));
        ui.label(format!("Poid : {} g", fermentecible.weight));
    });
    ui.add_space(DEFAULT_SPACING);
}

fn hop_ui(ui: &mut Ui, hop: &mut Hop) {
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut hop.name);
        ui.label("Acide alpha (%)");
        ui.add(Slider::new(&mut hop.alpha_acids, 0.0..=100.0));
        ui.label("Temps d'addition");
        ui.add(Slider::new(&mut hop.addition_time, 0..=60));
        ui.label("Ratio");
        ui.add(Slider::new(&mut hop.ratio, 0..=100));
        ui.label(format!("Poids (g) : {}", hop.weight));
        ui.label(format!("Utilisation : {}", hop.utilization));
        ui.label(format!("Contribution IBU : {}", hop.ibu));
    });
    ui.add_space(DEFAULT_SPACING);
}

fn whirlpool_hop_ui(ui: &mut Ui, batch_size: u16, hop: &mut WhirlpoolHop) {
    ui.horizontal(|ui| {
        ui.text_edit_singleline(&mut hop.name);
        ui.label("Acide alpha (%)");
        ui.add(Slider::new(&mut hop.alpha_acids, 0.0..=100.0));
        ui.label("Poids (g)");
        ui.add(Slider::new(&mut hop.weight, 0.0..=10000.0));
        ui.label(format!("{} g/l", hop.weight / batch_size as f32));
        ui.label("Temps d'addition: Whirlpool");
        ui.label(format!("Utilisation : {}", hop.utilization));
        ui.label(format!("Contribution IBU : {}", hop.ibu));
    });
    ui.add_space(DEFAULT_SPACING);
}
