use crate::app::modules::math;
use crate::app::modules::ui_defaults::*;
use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
struct Ferment {
    name: String,
    attenuation: u8,
    cells_per_gram: u32,
    pitch_weight: f64,
    ratio: u8,
}

#[derive(Deserialize, Serialize, Default)]
pub struct Yeast {
    pub batch_size: u16,
    pub original_gravity: f32,
    cell_count: u64,
    ferments: Vec<Ferment>,
    pub max_attenuation: u8,
}

impl super::AppModule for Yeast {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn show(&mut self, ui: &mut Ui) {
        self.cell_count = math::compute_cell_count(self.original_gravity, self.batch_size) as u64;

        ui.horizontal(|ui| {
            ui.heading("Ferments");

            ui.add_space(DEFAULT_SPACING);

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

        let mut ratios = vec![];

        for (index, ferment) in self.ferments.iter_mut().enumerate() {
            ferment_ui(ui, index, ferment);

            ferment.pitch_weight =
                math::compute_pitch_weight(self.cell_count, ferment.cells_per_gram, ferment.ratio);

            ratios.push(ferment.ratio);
        }

        // This is an arbitrary choice to handle cofermentations, has to be enhanced
        self.max_attenuation = self
            .ferments
            .iter()
            .map(|ferment| ferment.attenuation)
            .max()
            .unwrap_or(0);

        if !self.ferments.is_empty() && math::check_ratios(ratios) {
            ui.colored_label(
                ERROR_COLOR,
                "Problème de ratios : leur somme doit être égal à 100",
            );
            ui.add_space(DEFAULT_SPACING);
        }
    }
}

fn ferment_ui(ui: &mut Ui, index: usize, ferment: &mut Ferment) {
    Window::new(format!("Ferment {}", index + 1))
        .default_size([250., 250.])
        .show(ui.ctx(), |ui| {
            egui::Frame::new()
                .fill(LIGHTER_COLOR)
                .inner_margin(DEFAULT_PADDING)
                .corner_radius(DEFAULT_CORNER_RADIUS)
                .show(ui, |ui| {
                    ui.text_edit_singleline(&mut ferment.name);
                    ui.add_space(DEFAULT_SPACING);
                    ui.label("Atténuation (%)");
                    ui.add(Slider::new(&mut ferment.attenuation, 0..=100));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label("Cellules par gramme (millions)");
                    ui.add(Slider::new(&mut ferment.cells_per_gram, 0..=10_000));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label("Ratio (%)");
                    ui.add(Slider::new(&mut ferment.ratio, 0..=100));
                    ui.add_space(DEFAULT_SPACING);
                    ui.label(format!("Poids (g) {:.1}", ferment.pitch_weight));
                });
        });
}
