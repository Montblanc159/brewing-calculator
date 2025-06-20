const DEFAULT_SPACING: f32 = 8.0;

struct Malt {
    name: String,
    extract: u8,
    humidity: u8,
    ebc: u8,
}

struct Yeast {
    name: String,
    attenuation: u8,
    pitch_rate: u8,
}

struct Hop {
    name: String,
    alpha_acids: u8,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BrewingCalcApp {
    name: String,
    style: String,

    // #[serde(skip)] // This how you opt-out of serialization of a field
    abv: f32,
    ebc: u8,
    ibu: u8,
    original_gravity: f32,
    final_gravity: f32,
    efficiency: u8,
    batch_size: u16,
}

impl Default for BrewingCalcApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            name: String::from(""),
            style: String::from(""),
            abv: 5.0,
            ebc: 5,
            ibu: 10,
            original_gravity: 12.0,
            final_gravity: 2.0,
            efficiency: 80,
            batch_size: 20,
        }
    }
}

impl BrewingCalcApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for BrewingCalcApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(DEFAULT_SPACING * 2.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::SidePanel::left("my_left_panel").show(ctx, |ui| {
            ui.label("Hello World!");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
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

            ui.horizontal(|ui| {
                ui.label("Alcool (%) : ");
                ui.add(egui::Slider::new(&mut self.abv, 0.0..=15.0));
            });

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("EBC : ");
                ui.add(egui::Slider::new(&mut self.ebc, 0..=140));
            });

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("IBU : ");
                ui.add(egui::Slider::new(&mut self.ibu, 0..=70));
            });

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Densité initiale (°P) : ");
                ui.add(egui::Slider::new(&mut self.original_gravity, 0.0..=25.0));
            });

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Densité finale (°P): ");
                ui.add(egui::Slider::new(&mut self.final_gravity, 0.0..=25.0));
            });

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Efficacité (%): ");
                ui.add(egui::Slider::new(&mut self.efficiency, 0..=100));
            });

            ui.add_space(DEFAULT_SPACING);

            ui.horizontal(|ui| {
                ui.label("Volume (L): ");
                ui.add(egui::Slider::new(&mut self.batch_size, 0..=30000));
            });

            ui.add_space(DEFAULT_SPACING);

            ui.separator();
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
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
