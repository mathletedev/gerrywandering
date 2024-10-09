use nannou_egui::Egui;

use crate::{boid::Boid, gerrymander::Node, settings::Settings};

pub struct Model {
    pub egui: Egui,
    pub settings: Settings,
    pub boids: Vec<Boid>,
    pub districts_tree: Option<Box<Node>>,
}
