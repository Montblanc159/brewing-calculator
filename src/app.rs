use eframe::*;
use egui::*;
use serde::*;

const DEFAULT_SPACING: f32 = 8.0;

#[derive(Deserialize, Serialize, Default)]
struct Fermentecible {
    name: String,
    extract: f32,
    humidity: f32,
    ebc: u8,
    ratio: u8,
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
    hops: Vec<Hop>,
    fermentecibles: Vec<Fermentecible>,
    ferments: Vec<Ferment>,
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
            batch_size: 20,
            hops: vec![],
            fermentecibles: vec![],
            ferments: vec![],
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
                if ui
                    .add(Slider::new(&mut self.original_gravity, 0.0..=25.0))
                    .changed()
                {
                    self.abv = compute_abv(self.original_gravity, self.final_gravity);
                };
            });

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Densité finale (°P): ");
                if ui
                    .add(Slider::new(&mut self.final_gravity, 0.0..=25.0))
                    .changed()
                {
                    self.abv = compute_abv(self.original_gravity, self.final_gravity);
                };
            });

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Efficacité (%): ");
                ui.add(Slider::new(&mut self.efficiency, 0..=100));
            });

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
                    })
                };
            });

            ui.add_space(DEFAULT_SPACING);

            for index in 0..self.ferments.len() {
                ferment_ui(ui, &mut self.ferments, index);
            }

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Fermentécibles");
                if ui.button("+").clicked() {
                    self.fermentecibles.push(Fermentecible {
                        ..Default::default()
                    })
                };
            });

            ui.add_space(DEFAULT_SPACING);

            let mut ratios = vec![];
            for fermentecible in &self.fermentecibles {
                ratios.push(fermentecible.ratio)
            }

            if self.fermentecibles.len() > 0 && check_fermentecible_ratio(ratios) {
                ui.colored_label(
                    Color32::from_rgb(255, 70, 70),
                    "Problème de ratios : leur somme doit être égal à 100",
                );
                ui.add_space(DEFAULT_SPACING);
            }

            for index in 0..self.fermentecibles.len() {
                fermentecible_ui(ui, &mut self.fermentecibles, index);
            }

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Houblons");
                if ui.button("+").clicked() {
                    self.hops.push(Hop {
                        ..Default::default()
                    })
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
        if ui.button("Supprimer").clicked() {
            ferments.remove(index);
        }
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
        ui.label("EBC (g/L)");
        ui.add(Slider::new(&mut fermentecible.ebc, 0..=150));
        ui.label("Ratio (%)");
        ui.add(Slider::new(&mut fermentecible.ratio, 0..=100));
        if ui.button("Supprimer").clicked() {
            fermentecibles.remove(index);
        }
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
