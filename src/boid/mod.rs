use nalgebra::{Point2, Rotation2};

pub mod renderer;
pub mod sim;

pub const NUM_VERTS: u64 = 3;
pub const NUM_BOIDS: u64 = 2;

#[derive(Debug, Clone, Copy)]
pub struct Boid {
    pub position: Point2<f32>,
    pub rotation: Rotation2<f32>,
}
