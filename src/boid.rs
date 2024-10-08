use std::f32::consts::TAU;

use nannou::prelude::*;

use crate::config::{BOID_MAX_SPEED, BOID_MIN_SPEED, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Clone)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Default for Boid {
    fn default() -> Self {
        Self {
            position: Vec2::new(
                random_range(-(WINDOW_WIDTH as f32) / 2.0, WINDOW_WIDTH as f32 / 2.0) as f32,
                random_range(-(WINDOW_HEIGHT as f32) / 2.0, WINDOW_HEIGHT as f32 / 2.0) as f32,
            ),
            velocity: Vec2::X.rotate(random_range(0.0, TAU))
                * random_range(BOID_MIN_SPEED, BOID_MAX_SPEED),
            acceleration: Vec2::ZERO,
        }
    }
}

impl Boid {
    pub fn next(&self, dt: Update) -> Boid {
        let mut boid = self.clone();

        boid.position += boid.velocity * dt.since_last.as_secs_f32();

        boid
    }
}
