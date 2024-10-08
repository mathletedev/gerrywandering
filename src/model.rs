use nannou::prelude::*;

use crate::{boid::Boid, settings::Settings};

pub struct Model {
    pub window: WindowId,
    pub settings: Settings,
    pub boids: Vec<Boid>,
}
