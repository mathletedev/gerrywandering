use nannou::prelude::*;

use crate::{config::BOID_SIZE, model::Model};

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    model.boids.iter().for_each(|boid| {
        draw.tri().points(
            boid.position + boid.velocity.normalize() * BOID_SIZE,
            boid.position + boid.velocity.rotate(TAU * 0.4).normalize() * BOID_SIZE,
            boid.position + boid.velocity.rotate(TAU * 0.6).normalize() * BOID_SIZE,
        );
    });

    draw.to_frame(app, &frame).unwrap();
}
