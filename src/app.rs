mod modules;

use eframe::*;
use egui::*;
use modules::ui_defaults::*;
use modules::*;
use serde::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BrewingCalcApp {
    // #[serde(skip)] // This how you opt-out of serialization of a field
    base: base::Base,
    bjcp_indexer: bjcp_style_index::BJCPStyleIndex,
    ingredients_indexer: ingredients_index::IngredientsIndex,
    equilibrium_pressure: equilibrium_pressure::EquilibriumPressure,
    temperature_after_mix: temperature_after_mix::TemperatureAfterMix,
    yeast: yeast::Yeast,
    fermentecibles: fermentecibles::Fermentecibles,
    water: water::Water,
    hops: hops::Hops,
}

impl Default for BrewingCalcApp {
    fn default() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let beer_styles =
            modules::JsonParser::<bjcp_style_index::BeerStyle>::new(bjcp_style_index::JSON_PATH)
                .parse_json();

        #[cfg(target_arch = "wasm32")]
        let beer_styles = modules::WasmJsonParser::<bjcp_style_index::BeerStyle>::new(
            bjcp_style_index::BJCPStyleIndex::parse_file(),
        )
        .parse_json();

        Self {
            base: base::Base::new(),
            bjcp_indexer: bjcp_style_index::BJCPStyleIndex::new(beer_styles),
            ingredients_indexer: ingredients_index::IngredientsIndex::new(),
            equilibrium_pressure: equilibrium_pressure::EquilibriumPressure::new(),
            temperature_after_mix: temperature_after_mix::TemperatureAfterMix::new(),
            yeast: yeast::Yeast::new(),
            fermentecibles: fermentecibles::Fermentecibles::new(),
            water: water::Water::new(),
            hops: hops::Hops::new(),
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
            self.ingredients_indexer.show(ui)
        });

        // Add a lot of widgets here.
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                // The central panel the region left after adding TopPanel's and SidePanel's
                ui.heading("Broutille - la calculette du brasseur");

                ui.separator();

                ui.add_space(DEFAULT_SPACING);

                self.base.ebc = self.fermentecibles.ebc;
                self.base.yeast_attenuation = self.yeast.max_attenuation;

                self.base.show(ui);

                ui.add_space(DEFAULT_SPACING);

                self.water.batch_size = self.base.batch_size;
                self.water.grain_weight = self.fermentecibles.total_weight;

                self.water.show(ui);

                ui.add_space(DEFAULT_SPACING);

                self.yeast.original_gravity = self.base.original_gravity;
                self.yeast.batch_size = self.base.batch_size;

                self.yeast.show(ui);

                ui.add_space(DEFAULT_SPACING);

                self.fermentecibles.batch_size = self.base.batch_size;
                self.fermentecibles.original_gravity = self.base.original_gravity;
                self.fermentecibles.efficiency = self.base.efficiency;

                self.fermentecibles.show(ui);

                ui.add_space(DEFAULT_SPACING);

                self.hops.original_gravity = self.base.original_gravity;
                self.hops.batch_size = self.base.batch_size;
                self.hops.ibu = self.base.ibu;
                self.hops.show(ui);
            });
        });
    }
}
