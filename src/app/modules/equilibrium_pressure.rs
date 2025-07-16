use crate::app::modules::math::compute_equilibrium_pressure;
use crate::app::modules::ui_defaults::*;
use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct EquilibriumPressure {
    opened: bool,
    beer_temp: f32,
    saturation_target: f32,
    equilibrium_pressure: f32,
}

impl Default for EquilibriumPressure {
    fn default() -> Self {
        Self {
            opened: false,
            beer_temp: 0.,
            saturation_target: 0.,
            equilibrium_pressure: 0.,
        }
    }
}

impl EquilibriumPressure {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        if ui.button("Pression d'équilibre").clicked() {
            self.opened = !self.opened;
        };

        egui::Window::new("Pression d'équilibre")
            .open(&mut self.opened)
            .default_size([250., 250.])
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Température de la bière (°C): ");
                    ui.add(Slider::new(&mut self.beer_temp, 0.0..=45.));
                });

                ui.add_space(DEFAULT_SPACING);

                ui.horizontal(|ui| {
                    ui.label("Saturation cible (g/l): ");
                    ui.add(Slider::new(&mut self.saturation_target, 0.0..=10.0));
                });

                self.equilibrium_pressure =
                    compute_equilibrium_pressure(self.beer_temp, self.saturation_target);

                ui.add_space(DEFAULT_SPACING);

                ui.label(format!(
                    "Pression d'équilibre (bar): {}",
                    self.equilibrium_pressure
                ))
            });
    }
}
