use nannou::prelude::*;
use nannou_egui::{self, egui};

use crate::{
    boid::Party,
    gerrymander::{count_districts, count_parties, gerrymander, Bounds},
    model::Model,
    settings::{NUM_PARTIES, WINDOW_HEIGHT, WINDOW_WIDTH},
};

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

        ui.separator();

        let mut num_red = 0;
        let mut num_blue = 0;
        let mut num_none = 0;

        model.boids.iter().for_each(|boid| match boid.party {
            Some(Party::RED) => num_red += 1,
            Some(Party::BLUE) => num_blue += 1,
            None => num_none += 1,
        });

        ui.label(format!("# red: {}", num_red));
        ui.label(format!("# blue: {}", num_blue));
        ui.label(format!("# none: {}", num_none));

        let mut num_districts = [0; NUM_PARTIES];
        if settings.paused {
            num_districts = count_districts(&model.districts_tree);
        }

        ui.label(format!("# red districts: {}", num_districts[0]));
        ui.label(format!("# blue districts: {}", num_districts[1]));

        ui.separator();

        ui.label("Favour:");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut settings.favour, Party::RED, "Red");
            ui.selectable_value(&mut settings.favour, Party::BLUE, "Blue");
        });

        let clicked = ui
            .button(if settings.paused {
                "Resume"
            } else {
                "Gerrymander!"
            })
            .clicked();

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
