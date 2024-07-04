use nalgebra::{Point2, Rotation2, Vector2};

pub mod renderer;
pub mod sim;

pub const NUM_VERTS: u64 = 3;
pub const NUM_BOIDS: u64 = 20;

#[derive(Debug, Clone, Copy)]
pub struct Boid {
    pub position: Point2<f32>,
    pub rotation: Rotation2<f32>,
    pub velocity: Vector2<f32>,
}

pub fn initialize_boids() -> Vec<Boid> {
    let mut boids = Vec::new();
    for _ in 0..NUM_BOIDS {
        let (x, y): (f32, f32) = rand::random();
        let (x, y) = ((x - 0.5) * 2.0, (y - 0.5) * 2.0);

        let boid = Boid {
            position: Point2::new(x, y),
            rotation: Rotation2::identity(),
            velocity: Vector2::new(10.0, 1.0),
        };
        boids.push(boid);
    }

    boids
}
