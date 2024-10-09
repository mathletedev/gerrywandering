use std::f32::consts::TAU;

use nannou::prelude::*;

use crate::settings::{Settings, WINDOW_HEIGHT, WINDOW_WIDTH};

#[derive(Clone, Copy, PartialEq)]
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
    pub fn new(id: u32, settings: &Settings) -> Self {
        Self {
            id,
            position: Vec2::new(
                random_range(-(WINDOW_WIDTH as f32 / 2.0), WINDOW_WIDTH as f32 / 2.0) as f32,
                random_range(-(WINDOW_HEIGHT as f32 / 2.0), WINDOW_HEIGHT as f32 / 2.0) as f32,
            ),
            velocity: Vec2::X.rotate(random_range(0.0, TAU))
                * random_range(settings.boid_min_speed, settings.boid_max_speed),
            acceleration: Vec2::ZERO,
            party: Party::random(),
        }
    }

    pub fn next(&self, dt: f32, boids: &[Boid], settings: &Settings) -> Boid {
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
            if distance_squared > settings.boid_view_radius.pow(2) {
                return;
            }

            count += 1;

            let mut alignment_add = other.velocity;
            let mut cohesion_add = other.position - self.position;

            if self.party.is_some() && self.party == other.party {
                alignment_add *= settings.preference_multiplier;
                cohesion_add *= settings.preference_multiplier;
            }

            alignment_heading += other.velocity;
            cohesion_heading += other.position - self.position;

            if self.party.is_some() && other.party.is_some() && self.party != other.party {
                separation_heading -= (other.position - self.position) / distance_squared
                    * settings.avoidance_multiplier;
            }

            if distance_squared > settings.boid_avoid_radius.pow(2) {
                return;
            }

            separation_heading -= (other.position - self.position) / distance_squared;
        });

        boid.acceleration = Vec2::ZERO;

        if alignment_heading != Vec2::ZERO {
            boid.steer_towards(alignment_heading, settings.alignment_weight, settings);
        }
        if cohesion_heading != Vec2::ZERO && count > 0 {
            cohesion_heading /= count as f32;
            boid.steer_towards(cohesion_heading, settings.cohesion_weight, settings);
        }
        if separation_heading != Vec2::ZERO {
            boid.steer_towards(separation_heading, settings.separation_weight, settings);
        }

        // avoid borders
        if boid.position.x < -(WINDOW_WIDTH as f32 / 2.0) + settings.boid_avoid_radius {
            boid.steer_towards(Vec2::X, settings.border_weight, settings);
        }
        if boid.position.x > WINDOW_WIDTH as f32 / 2.0 - settings.boid_avoid_radius {
            boid.steer_towards(-Vec2::X, settings.border_weight, settings);
        }
        if boid.position.y < -(WINDOW_HEIGHT as f32 / 2.0) + settings.boid_avoid_radius {
            boid.steer_towards(Vec2::Y, settings.border_weight, settings);
        }
        if boid.position.y > WINDOW_HEIGHT as f32 / 2.0 - settings.boid_avoid_radius {
            boid.steer_towards(-Vec2::Y, settings.border_weight, settings);
        }

        boid.velocity += boid.acceleration * dt;
        boid.velocity = boid
            .velocity
            .clamp_length(settings.boid_min_speed, settings.boid_max_speed);

        boid.position += boid.velocity * dt;

        if random_f32() < settings.mutation_rate * dt {
            boid.party = Party::random();
        }

        boid
    }

    fn steer_towards(&mut self, direction: Vec2, weight: f32, settings: &Settings) {
        self.acceleration += (direction.normalize() * settings.boid_max_speed - self.velocity)
            .clamp_length_max(settings.boid_steer_force)
            * weight;
    }
}
