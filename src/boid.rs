use std::f32::consts::TAU;

use nannou::prelude::*;

use crate::config::{
    ALIGNMENT_WEIGHT, BOID_MAX_SPEED, BOID_MIN_SPEED, BOID_STEER_FORCE, BOID_VIEW_RADIUS,
    WINDOW_HEIGHT, WINDOW_WIDTH,
};

#[derive(Clone)]
pub struct Boid {
    pub id: u32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl Boid {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            position: Vec2::new(
                random_range(-(WINDOW_WIDTH as f32 / 2.0), WINDOW_WIDTH as f32 / 2.0) as f32,
                random_range(-(WINDOW_HEIGHT as f32 / 2.0), WINDOW_HEIGHT as f32 / 2.0) as f32,
            ),
            velocity: Vec2::X.rotate(random_range(0.0, TAU))
                * random_range(BOID_MIN_SPEED, BOID_MAX_SPEED),
            acceleration: Vec2::ZERO,
        }
    }

    pub fn next(&self, dt: Update, boids: &[Boid]) -> Boid {
        let mut boid = self.clone();

        let mut avg_heading = Vec2::ZERO;

        boids.iter().for_each(|other| {
            if self.position.distance_squared(other.position) > BOID_VIEW_RADIUS.pow(2) {
                return;
            }

            avg_heading += other.velocity;
        });

        boid.acceleration = Vec2::ZERO;

        let alignment_force = self.steer_towards(avg_heading) * ALIGNMENT_WEIGHT;

        boid.acceleration += alignment_force;

        boid.velocity += boid.acceleration * dt.since_last.as_secs_f32();
        boid.velocity = boid.velocity.clamp_length(BOID_MIN_SPEED, BOID_MAX_SPEED);

        boid.position += boid.velocity * dt.since_last.as_secs_f32();

        if boid.position.x < -(WINDOW_WIDTH as f32 / 2.0) {
            boid.position.x += WINDOW_WIDTH as f32;
        }
        if boid.position.x > WINDOW_WIDTH as f32 / 2.0 {
            boid.position.x -= WINDOW_WIDTH as f32;
        }
        if boid.position.y < -(WINDOW_HEIGHT as f32 / 2.0) {
            boid.position.y += WINDOW_HEIGHT as f32;
        }
        if boid.position.y > WINDOW_HEIGHT as f32 / 2.0 {
            boid.position.y -= WINDOW_HEIGHT as f32;
        }

        boid
    }

    fn steer_towards(&self, target: Vec2) -> Vec2 {
        (target.normalize() * BOID_MAX_SPEED - self.velocity).clamp_length_max(BOID_STEER_FORCE)
    }
}
