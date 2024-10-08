use nannou_egui::Egui;

use crate::{boid::Boid, settings::Settings};

pub struct Model {
    pub egui: Egui,
    pub settings: Settings,
    pub boids: Vec<Boid>,
}
