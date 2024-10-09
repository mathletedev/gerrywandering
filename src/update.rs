use nannou::prelude::*;
use nannou_egui::{self, egui};

use crate::{
    gerrymander::{count_parties, gerrymander, Bounds},
    model::Model,
    settings::{WINDOW_HEIGHT, WINDOW_WIDTH},
};

pub fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Alignment");
        ui.add(egui::Slider::new(
            &mut settings.alignment_weight,
            0.0..=10.0,
        ));

        ui.label("Cohesion");
        ui.add(egui::Slider::new(&mut settings.cohesion_weight, 0.0..=10.0));

        ui.label("Separation");
        ui.add(egui::Slider::new(
            &mut settings.separation_weight,
            0.0..=100.0,
        ));

        let clicked = ui.button("Gerrywander").clicked();

        if clicked {
            settings.paused = !settings.paused;

            if settings.paused && model.districts_tree.is_none() {
                model.districts_tree = Some(Box::default());

                count_parties(
                    &mut model.districts_tree,
                    Bounds {
                        left: -(WINDOW_WIDTH as f32 / 2.0),
                        bottom: -(WINDOW_HEIGHT as f32 / 2.0),
                        width: WINDOW_WIDTH as f32,
                        height: WINDOW_HEIGHT as f32,
                    },
                    &model.boids,
                );

                gerrymander(&mut model.districts_tree, settings.favour);
            } else if !settings.paused && model.districts_tree.is_some() {
                model.districts_tree = None;
            }
        }
    });

    if settings.paused {
        return;
    }

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
