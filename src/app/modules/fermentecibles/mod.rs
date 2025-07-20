use crate::app::modules::math;
use crate::app::modules::ui_defaults::*;
use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};

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
pub struct Fermentecibles {
    pub total_weight: f32,
    pub original_gravity: f32,
    pub efficiency: u8,
    pub batch_size: u16,
    pub ebc: u8,
    fermentecibles: Vec<Fermentecible>,
}

impl super::AppModule for Fermentecibles {
    fn new() -> Self {
        Self {
            efficiency: 80,
            ..Default::default()
        }
    }

    fn show(&mut self, ui: &mut Ui) {
        ui.add_space(DEFAULT_SPACING);

        ui.horizontal(|ui| {
            ui.heading("Fermentescibles");
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
            let total_extract: f32 = math::compute_total_extract(self.original_gravity);

            let fermentecible_extractable = math::compute_per_malt_extractable(
                total_extract,
                fermentecible.ratio,
                self.efficiency,
            );

            fermentecible.weight = math::compute_grain_bill(
                self.batch_size,
                fermentecible_extractable,
                fermentecible.humidity,
                fermentecible.extract,
            );

            fermentecible.mcu =
                math::compute_mcu(fermentecible.ebc, fermentecible.weight, self.batch_size);

            ratios.push(fermentecible.ratio);
            weights.push(fermentecible.weight);
            mcus.push(fermentecible.mcu);
        }

        self.ebc = math::compute_ebc(mcus.iter().sum());

        self.total_weight = weights.iter().sum();

        ui.horizontal(|ui| {
            for (index, fermentecible) in &mut self.fermentecibles.iter_mut().enumerate() {
                ui.vertical(|ui| {
                    fermentecible_ui(ui, index, fermentecible);
                });
            }
        });

        if !self.fermentecibles.is_empty() && math::check_ratios(ratios) {
            ui.colored_label(
                ERROR_COLOR,
                "Problème de ratios : leur somme doit être égal à 100",
            );
            ui.add_space(DEFAULT_SPACING);
        }
    }
}

fn fermentecible_ui(ui: &mut Ui, index: usize, fermentecible: &mut Fermentecible) {
    Window::new(format!("Fermentescible {}", index + 1))
        .default_size([250., 250.])
        .show(ui.ctx(), |ui| {
            egui::Frame::new()
                .fill(LIGHTER_COLOR)
                .inner_margin(DEFAULT_PADDING)
                .corner_radius(DEFAULT_CORNER_RADIUS)
                .show(ui, |ui| {
                    ui.text_edit_singleline(&mut fermentecible.name);
                    ui.add_space(DEFAULT_SPACING);
                    ui.label("Extrait (%)");
                    ui.add(Slider::new(&mut fermentecible.extract, 0.0..=100.0));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label("Humidité (%)");
                    ui.add(Slider::new(&mut fermentecible.humidity, 0.0..=100.0));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label("EBC");
                    ui.add(Slider::new(&mut fermentecible.ebc, 0..=150));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label("Ratio (%)");
                    ui.add(Slider::new(&mut fermentecible.ratio, 0..=100));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label(format!("Poid : {:.0} g", fermentecible.weight));
                });
        });
}
