pub mod boid;
pub mod event;
pub mod gerrymander;
pub mod model;
pub mod settings;
pub mod update;
pub mod view;

use boid::Boid;
use model::Model;
use nannou::prelude::*;
use nannou_egui::Egui;
use settings::{Settings, NUM_BOIDS, WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() {
    nannou::app(model)
        .event(event::event)
        .update(update::update)
        .run();
}

fn model(app: &App) -> Model {
    let settings = Settings::default();

    let window_id = app
        .new_window()
        .resizable(false)
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Gerrywandering")
        .view(view::view)
        .raw_event(event::raw_window_event)
        .build()
        .unwrap();

    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    let boids = (0..NUM_BOIDS).map(|i| Boid::new(i, &settings)).collect();

    Model {
        egui,
        settings,
        boids,
        districts_tree: None,
    }
}
