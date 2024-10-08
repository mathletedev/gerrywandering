use nannou::prelude::*;

use crate::boid::Boid;

pub struct Model {
    pub window: WindowId,
    pub boids: Vec<Boid>,
}
