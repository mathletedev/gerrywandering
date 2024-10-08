use std::f32::consts::TAU;

use nannou::prelude::*;

use crate::config::{
    ALIGNMENT_WEIGHT, AVOIDANCE_MULTIPLIER, BOID_AVOID_RADIUS, BOID_MAX_SPEED, BOID_MIN_SPEED,
    BOID_STEER_FORCE, BOID_VIEW_RADIUS, BORDER_WEIGHT, COHESION_WEIGHT, MUTATION_RATE,
    PREFERENCE_MULTIPLIER, SEPARATION_WEIGHT, WINDOW_HEIGHT, WINDOW_WIDTH,
};

#[derive(Clone, PartialEq)]
pub enum Party {
    RED,
    BLUE,
}

impl Party {
    pub fn random() -> Option<Self> {
        if random_f32() < 0.1 {
            None
        } else if random_f32() < 0.5 {
            Some(Party::RED)
        } else {
            Some(Party::BLUE)
        }
    }
}

#[derive(Clone)]
pub struct Boid {
    pub id: u32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub party: Option<Party>,
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
            party: Party::random(),
        }
    }

    pub fn next(&self, dt: Update, boids: &[Boid]) -> Boid {
        let mut boid = self.clone();

        let mut alignment_heading = Vec2::ZERO;
        let mut cohesion_heading = Vec2::ZERO;
        let mut separation_heading = Vec2::ZERO;

        let mut count = 0;

        boids.iter().for_each(|other| {
            if self.id == other.id || self.position == other.position {
                return;
            }

            let distance_squared = self.position.distance_squared(other.position);
            if distance_squared > BOID_VIEW_RADIUS.pow(2) {
                return;
            }

            count += 1;

            let mut alignment_add = other.velocity;

            if self.party.is_some() && self.party == other.party {
                alignment_add *= PREFERENCE_MULTIPLIER;
            }

            alignment_heading += other.velocity;
            cohesion_heading += other.position - self.position;

            if self.party.is_some() && other.party.is_some() && self.party != other.party {
                separation_heading -=
                    (other.position - self.position) / distance_squared * AVOIDANCE_MULTIPLIER;
            }

            if distance_squared > BOID_AVOID_RADIUS.pow(2) {
                return;
            }

            separation_heading -= (other.position - self.position) / distance_squared;
        });

        boid.acceleration = Vec2::ZERO;

        if alignment_heading != Vec2::ZERO {
            boid.steer_towards(alignment_heading, ALIGNMENT_WEIGHT);
        }
        if cohesion_heading != Vec2::ZERO && count > 0 {
            cohesion_heading /= count as f32;
            boid.steer_towards(cohesion_heading, COHESION_WEIGHT);
        }
        if separation_heading != Vec2::ZERO {
            boid.steer_towards(separation_heading, SEPARATION_WEIGHT);
        }

        // avoid borders
        if boid.position.x < -(WINDOW_WIDTH as f32 / 2.0) + BOID_AVOID_RADIUS {
            boid.steer_towards(Vec2::X, BORDER_WEIGHT);
        }
        if boid.position.x > WINDOW_WIDTH as f32 / 2.0 - BOID_AVOID_RADIUS {
            boid.steer_towards(-Vec2::X, BORDER_WEIGHT);
        }
        if boid.position.y < -(WINDOW_HEIGHT as f32 / 2.0) + BOID_AVOID_RADIUS {
            boid.steer_towards(Vec2::Y, BORDER_WEIGHT);
        }
        if boid.position.y > WINDOW_HEIGHT as f32 / 2.0 - BOID_AVOID_RADIUS {
            boid.steer_towards(-Vec2::Y, BORDER_WEIGHT);
        }

        boid.velocity += boid.acceleration * dt.since_last.as_secs_f32();
        boid.velocity = boid.velocity.clamp_length(BOID_MIN_SPEED, BOID_MAX_SPEED);

        boid.position += boid.velocity * dt.since_last.as_secs_f32();

        if random_f32() < MUTATION_RATE * dt.since_last.as_secs_f32() {
            boid.party = Party::random();
        }

        boid
    }

    fn steer_towards(&mut self, direction: Vec2, weight: f32) {
        self.acceleration += (direction.normalize() * BOID_MAX_SPEED - self.velocity)
            .clamp_length_max(BOID_STEER_FORCE)
            * weight;
    }
}
