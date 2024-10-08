pub mod boid;
pub mod config;
pub mod event;
pub mod model;
pub mod update;
pub mod view;

use boid::Boid;
use config::{NUM_BOIDS, WINDOW_HEIGHT, WINDOW_WIDTH};
use model::Model;
use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .event(event::event)
        .update(update::update)
        .run();
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .resizable(false)
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Gerrywandering")
        .view(view::view)
        .build()
        .unwrap();

    let boids = (0..NUM_BOIDS).map(Boid::new).collect();

    Model { window, boids }
}
