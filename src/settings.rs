pub const WINDOW_WIDTH: u32 = 1024;
pub const WINDOW_HEIGHT: u32 = 1024;

pub const NUM_BOIDS: u32 = 100;

pub const BOID_SIZE: f32 = 10.0;

pub struct Settings {
    pub boid_max_speed: f32,
    pub boid_min_speed: f32,
    pub boid_steer_force: f32,

    pub boid_view_radius: f32,
    pub boid_avoid_radius: f32,

    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub separation_weight: f32,
    pub border_weight: f32, // avoidance of border

    pub preference_multiplier: f32, // alignment & cohesion to same party
    pub avoidance_multiplier: f32,  // separation from opposite party

    pub mutation_rate: f32, // probability of switching to another party (multiplied by dt)
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            boid_max_speed: 200.0,
            boid_min_speed: 100.0,
            boid_steer_force: 200.0,

            boid_view_radius: 100.0,
            boid_avoid_radius: 50.0,

            alignment_weight: 1.0,
            cohesion_weight: 0.5,
            separation_weight: 5.0,
            border_weight: 10.0,

            preference_multiplier: 2.0,
            avoidance_multiplier: 10.0,

            mutation_rate: 0.1,
        }
    }
}
