pub mod boid;
pub mod event;
pub mod model;
pub mod settings;
pub mod update;
pub mod view;

use boid::Boid;
use model::Model;
use nannou::prelude::*;
use settings::{Settings, NUM_BOIDS, WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() {
    nannou::app(model)
        .event(event::event)
        .update(update::update)
        .run();
}

fn model(app: &App) -> Model {
    let settings = Settings::default();

    let window = app
        .new_window()
        .resizable(false)
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Gerrywandering")
        .view(view::view)
        .build()
        .unwrap();

    let boids = (0..NUM_BOIDS).map(|i| Boid::new(i, &settings)).collect();

    Model {
        window,
        settings,
        boids,
    }
}
