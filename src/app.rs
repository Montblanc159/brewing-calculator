mod modules;

use eframe::*;
use egui::*;
use modules::bjcp_style_index;
use modules::equilibrium_pressure;
use modules::fermentecibles;
use modules::math;
use modules::temperature_after_mix;
use modules::ui_defaults::*;
use modules::yeast;
use serde::*;

use crate::app::modules::AppModule;

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
    cells_per_gram: u32,
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
    addition_temp: f32,
}

#[derive(Deserialize, Serialize, Default)]
struct WhirlpoolHop {
    name: String,
    alpha_acids: f32,
    addition_time: u8,
    weight: f32,
    utilization: f32,
    ibu: f32,
    addition_temp: f32,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BrewingCalcApp {
    // #[serde(skip)] // This how you opt-out of serialization of a field
    name: String,
    style: String,
    abv: f32,
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
    mash_water_vol: f32,
    post_mash_water_vol: f32,
    sparge_water_vol: f32,
    pre_ebullition_water_vol: f32,
    bjcp_indexer: bjcp_style_index::BJCPStyleIndex,
    equilibrium_pressure: equilibrium_pressure::EquilibriumPressure,
    temperature_after_mix: temperature_after_mix::TemperatureAfterMix,
    yeast: yeast::Yeast,
    fermentecibles: fermentecibles::Fermentecibles,
}

impl Default for BrewingCalcApp {
    fn default() -> Self {
        Self {
            name: String::from(""),
            style: String::from(""),
            abv: 5.0,
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
            mash_water_vol: 0.0,
            post_mash_water_vol: 0.0,
            pre_ebullition_water_vol: 0.0,
            sparge_water_vol: 0.0,
            bjcp_indexer: bjcp_style_index::BJCPStyleIndex::new(bjcp_style_index::parse_json()),
            equilibrium_pressure: equilibrium_pressure::EquilibriumPressure::new(),
            temperature_after_mix: temperature_after_mix::TemperatureAfterMix::new(),
            yeast: yeast::Yeast::new(),
            fermentecibles: fermentecibles::Fermentecibles::new(),
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

        SidePanel::right("right_panel").show(ctx, |ui| self.bjcp_indexer.show(ui));
        SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Outils");

            self.equilibrium_pressure.show(ui);
            self.temperature_after_mix.show(ui);
        });

        // Add a lot of widgets here.
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                // The central panel the region left after adding TopPanel's and SidePanel's
                ui.heading("Broutille - la calculette du brasseur");

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

                ui.label(format!("Alcool (%) : {:.1}", self.abv));

                ui.add_space(DEFAULT_SPACING);

                ui.label(format!(
                    "EBC : {} ({:.1} SRM)",
                    self.fermentecibles.ebc,
                    math::convert_ebc_to_srm(self.fermentecibles.ebc)
                ));

                ui.add_space(DEFAULT_SPACING);

                ui.horizontal(|ui| {
                    ui.label("IBU : ");
                    ui.add(Slider::new(&mut self.ibu, 0.0..=150.0));
                });

                ui.add_space(DEFAULT_SPACING);

                ui.label(format!("BUGU : {:.2}", self.bugu));

                self.bugu = math::compute_bugu(self.ibu, self.original_gravity);

                ui.add_space(DEFAULT_SPACING);

                ui.horizontal(|ui| {
                    ui.label("Densité initiale (°P) : ");
                    ui.add(Slider::new(&mut self.original_gravity, 0.0..=25.0));
                    ui.label(format!(
                        "{:.3} SG",
                        math::convert_plato_to_sg(self.original_gravity)
                    ));
                });

                ui.add_space(DEFAULT_SPACING);

                ui.label(format!(
                    "Densité finale (°P): {:.1} ({:.3} SG)",
                    self.final_gravity,
                    math::convert_plato_to_sg(self.final_gravity)
                ));

                ui.add_space(DEFAULT_SPACING);

                ui.horizontal(|ui| {
                    ui.label("Efficacité (%): ");
                    ui.add(Slider::new(&mut self.fermentecibles.efficiency, 0..=100));
                });

                ui.add_space(DEFAULT_SPACING);
                ui.heading("Eau");
                ui.add_space(DEFAULT_SPACING);

                egui::Frame::new()
                    .fill(LIGHTER_COLOR)
                    .inner_margin(DEFAULT_PADDING)
                    .corner_radius(DEFAULT_CORNER_RADIUS)
                    .show(ui, |ui| {
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
                            "Volume d'eau à l'empatage (L): {:.3}",
                            self.mash_water_vol
                        ));

                        ui.add_space(DEFAULT_SPACING);

                        ui.label(format!(
                            "Volume d'eau de rinçage (L): {:.3}",
                            self.sparge_water_vol
                        ));

                        ui.add_space(DEFAULT_SPACING);

                        ui.label(format!(
                            "Volume d'eau pré-ébullition (L): {:.3}",
                            self.pre_ebullition_water_vol
                        ));

                        ui.add_space(DEFAULT_SPACING);

                        ui.horizontal(|ui| {
                            ui.label("Volume (L): ");
                            ui.add(Slider::new(&mut self.batch_size, 0..=30000));
                        });
                    });
                ui.add_space(DEFAULT_SPACING);

                self.yeast.cell_count =
                    math::compute_cell_count(self.original_gravity, self.batch_size) as u64;

                self.yeast.show(ui);

                self.final_gravity = math::compute_final_gravity(
                    self.original_gravity,
                    self.yeast.max_attenuation as f32,
                );

                self.abv = math::compute_abv(self.original_gravity, self.final_gravity);

                self.fermentecibles.batch_size = self.batch_size;
                self.fermentecibles.original_gravity = self.original_gravity;

                self.fermentecibles.show(ui);

                self.mash_water_vol = math::compute_mash_water_vol(
                    self.fermentecibles.total_weight,
                    self.mash_water_ratio,
                );

                self.post_mash_water_vol = math::compute_post_mash_water_vol(
                    self.mash_water_vol,
                    self.fermentecibles.total_weight,
                );

                self.sparge_water_vol = math::compute_sparge_water_vol(
                    self.batch_size,
                    self.evaporation_rate,
                    self.post_mash_water_vol,
                );

                self.pre_ebullition_water_vol = math::compute_pre_ebullition_water_vol(
                    self.sparge_water_vol,
                    self.post_mash_water_vol,
                );

                ui.add_space(DEFAULT_SPACING);

                ui.horizontal(|ui| {
                    ui.heading("Houblons au whirlpool");
                    if ui.button("+").clicked() {
                        self.whirlpool_hops.push(WhirlpoolHop {
                            addition_time: 0,
                            ..Default::default()
                        })
                    };

                    if ui.button("-").clicked() {
                        self.whirlpool_hops.pop();
                    };
                });

                ui.add_space(DEFAULT_SPACING);

                ScrollArea::horizontal()
                    .id_salt("third_scroll")
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            for (index, hop) in &mut self.whirlpool_hops.iter_mut().enumerate() {
                                ui.vertical(|ui| {
                                    whirlpool_hop_ui(ui, self.batch_size, index, hop);
                                });

                                hop.utilization = math::compute_hop_utilization(
                                    self.original_gravity,
                                    hop.addition_time,
                                );

                                hop.ibu = math::compute_ibu(
                                    hop.utilization,
                                    self.batch_size,
                                    hop.alpha_acids,
                                    hop.weight,
                                    self.original_gravity,
                                    hop.addition_temp,
                                );
                            }
                        });
                    });

                ui.add_space(DEFAULT_SPACING);

                ui.horizontal(|ui| {
                    ui.heading("Houblons");
                    if ui.button("+").clicked() {
                        self.hops.push(Hop {
                            addition_temp: 100.0,
                            ..Default::default()
                        })
                    };

                    if ui.button("-").clicked() {
                        self.hops.pop();
                    };
                });

                ui.add_space(DEFAULT_SPACING);

                let mut hop_ratios = vec![];
                ScrollArea::horizontal()
                    .id_salt("fourth_scroll")
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            for (index, hop) in &mut self.hops.iter_mut().enumerate() {
                                ui.vertical(|ui| {
                                    hop_ui(ui, index, hop);
                                });

                                hop.utilization = math::compute_hop_utilization(
                                    self.original_gravity,
                                    hop.addition_time,
                                );

                                let ibu_left = self.ibu
                                    - self
                                        .whirlpool_hops
                                        .iter()
                                        .map(|w_hop| w_hop.ibu)
                                        .sum::<f32>();

                                hop.weight = math::compute_hop_weight(
                                    hop.utilization,
                                    self.batch_size,
                                    hop.alpha_acids,
                                    ibu_left * (hop.ratio as f32 / 100.0),
                                    self.original_gravity,
                                    hop.addition_temp,
                                );

                                hop.ibu = math::compute_ibu(
                                    hop.utilization,
                                    self.batch_size,
                                    hop.alpha_acids,
                                    hop.weight,
                                    self.original_gravity,
                                    hop.addition_temp,
                                );

                                hop_ratios.push(hop.ratio)
                            }
                        });
                    });

                if !self.hops.is_empty() && math::check_ratios(hop_ratios) {
                    ui.colored_label(
                        ERROR_COLOR,
                        "Problème de ratios : leur somme doit être égal à 100",
                    );
                    ui.add_space(DEFAULT_SPACING);
                }
            });
        });
    }
}

fn hop_ui(ui: &mut Ui, index: usize, hop: &mut Hop) {
    Window::new(format!("Houblon {}", index + 1))
        .default_size([250., 250.])
        .show(ui.ctx(), |ui| {
            egui::Frame::new()
                .fill(LIGHTER_COLOR)
                .inner_margin(DEFAULT_PADDING)
                .corner_radius(DEFAULT_CORNER_RADIUS)
                .show(ui, |ui| {
                    ui.text_edit_singleline(&mut hop.name);
                    ui.add_space(DEFAULT_SPACING);
                    ui.label("Acide alpha (%)");
                    ui.add(Slider::new(&mut hop.alpha_acids, 0.0..=100.0));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label("Temps d'addition");
                    ui.add(Slider::new(&mut hop.addition_time, 0..=60));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label(format!(
                        "Température d'addition (°C) : {}",
                        hop.addition_temp
                    ));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label("Ratio");
                    ui.add(Slider::new(&mut hop.ratio, 0..=100));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label(format!("Poids (g) : {:.2}", hop.weight));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label(format!("Utilisation : {:.2}", hop.utilization));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label(format!("Contribution IBU : {:.2}", hop.ibu));
                });
        });
}

fn whirlpool_hop_ui(ui: &mut Ui, batch_size: u16, index: usize, hop: &mut WhirlpoolHop) {
    Window::new(format!("Houblon au W {}", index + 1))
        .default_size([250., 250.])
        .show(ui.ctx(), |ui| {
            egui::Frame::new()
                .fill(LIGHTER_COLOR)
                .inner_margin(DEFAULT_PADDING)
                .corner_radius(DEFAULT_CORNER_RADIUS)
                .show(ui, |ui| {
                    ui.text_edit_singleline(&mut hop.name);
                    ui.add_space(DEFAULT_SPACING);
                    ui.label("Acide alpha (%)");
                    ui.add(Slider::new(&mut hop.alpha_acids, 0.0..=100.0));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label("Poids (g)");
                    ui.add(Slider::new(&mut hop.weight, 0.0..=10000.0));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label(format!("{:.2} g/l", hop.weight / batch_size as f32));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label("Temps d'addition: Whirlpool");
                    ui.add_space(DEFAULT_SPACING);
                    ui.label("Température d'addition (°C)");
                    ui.add(Slider::new(&mut hop.addition_temp, 0.0..=100.0));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label(format!("Utilisation : {:.2}", hop.utilization));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label(format!("Contribution IBU : {:.2}", hop.ibu));
                });
        });
}
