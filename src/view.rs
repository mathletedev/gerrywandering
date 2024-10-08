use nannou::prelude::*;

use crate::{boid::Party, model::Model, settings::BOID_SIZE};

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    model.boids.iter().for_each(|boid| {
        let mut color = WHITE;

        if let Some(party) = &boid.party {
            color = match party {
                Party::RED => RED,
                Party::BLUE => BLUE,
            }
        }

        draw.tri()
            .points(
                boid.position + boid.velocity.normalize() * BOID_SIZE,
                boid.position + boid.velocity.rotate(TAU * 0.4).normalize() * BOID_SIZE,
                boid.position + boid.velocity.rotate(TAU * 0.6).normalize() * BOID_SIZE,
            )
            .color(color);
    });

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
