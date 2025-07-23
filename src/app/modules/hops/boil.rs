use crate::app::modules::math;
use crate::app::modules::ui_defaults::*;
use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default)]
struct BoilHop {
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
pub struct BoilHops {
    hops: Vec<BoilHop>,
    pub target_ibu: f32,
    pub batch_size: u16,
    pub original_gravity: f32,
}

impl super::super::AppModule for BoilHops {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Houblons");
            if ui.button("+").clicked() {
                self.hops.push(BoilHop {
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
                            boil_hop_ui(ui, index, hop);
                        });

                        hop.utilization =
                            math::compute_hop_utilization(self.original_gravity, hop.addition_time);

                        hop.weight = math::compute_hop_weight(
                            hop.utilization,
                            self.batch_size,
                            hop.alpha_acids,
                            self.target_ibu * (hop.ratio as f32 / 100.0),
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
    }
}

fn boil_hop_ui(ui: &mut Ui, index: usize, hop: &mut BoilHop) {
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
