use crate::app::modules::math;
use crate::app::modules::ui_defaults::*;
use eframe::*;
use egui::*;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize, Default)]
pub struct WhirlpoolHops {
    hops: Vec<WhirlpoolHop>,
    pub total_ibu: f32,
    pub original_gravity: f32,
    pub batch_size: u16,
}

impl super::super::AppModule for WhirlpoolHops {
    fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn show(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.heading("Houblons au whirlpool");
            if ui.button("+").clicked() {
                self.hops.push(WhirlpoolHop {
                    addition_time: 0,
                    ..Default::default()
                })
            };

            if ui.button("-").clicked() {
                self.hops.pop();
            };
        });

        ui.add_space(DEFAULT_SPACING);

        let mut total_ibu = 0.;

        ScrollArea::horizontal()
            .id_salt("third_scroll")
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (index, hop) in &mut self.hops.iter_mut().enumerate() {
                        ui.vertical(|ui| {
                            whirlpool_hop_ui(ui, self.batch_size, index, hop);
                        });

                        hop.utilization =
                            math::compute_hop_utilization(self.original_gravity, hop.addition_time);

                        hop.ibu = math::compute_ibu(
                            hop.utilization,
                            self.batch_size,
                            hop.alpha_acids,
                            hop.weight,
                            self.original_gravity,
                            hop.addition_temp,
                        );

                        total_ibu += hop.ibu;
                    }
                });
            });

        self.total_ibu = total_ibu;
    }
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
