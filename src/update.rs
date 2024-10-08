use nannou::prelude::*;
use nannou_egui::{self, egui};

use crate::model::Model;

pub fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Alignment:");
        ui.add(egui::Slider::new(
            &mut settings.alignment_weight,
            0.0..=10.0,
        ));

        ui.label("Cohesion:");
        ui.add(egui::Slider::new(&mut settings.cohesion_weight, 0.0..=10.0));

        ui.label("Separation:");
        ui.add(egui::Slider::new(
            &mut settings.separation_weight,
            0.0..=100.0,
        ));
    });

    model.boids = model
        .boids
        .iter()
        .map(|boid| {
            boid.next(
                update.since_last.as_secs_f32(),
                &model.boids,
                &model.settings,
            )
        })
        .collect();
}
