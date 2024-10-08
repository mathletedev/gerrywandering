use nannou::prelude::*;

use crate::model::Model;

pub fn update(_app: &App, model: &mut Model, update: Update) {
    model.boids = model.boids.iter().map(|boid| boid.next(update)).collect();
}
