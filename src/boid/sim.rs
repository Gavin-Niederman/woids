use std::{cell::RefCell, rc::Rc};

use nalgebra::Vector2;

const COHESION: f32 = 2.3;
const SEPARATION: f32 = 1.6;
const ALIGNMENT: f32 = 0.1;

use super::Boid;

pub fn update_boids(boids: Rc<RefCell<Vec<Boid>>>, dt: f32) {
    let mut boids = boids.borrow_mut();

    let center_of_mass: nalgebra::Vector2<f32> = boids
        .iter()
        .map(|b| b.position.coords)
        .sum::<Vector2<f32>>()
        / boids.len() as f32;

    let boids_clone = boids.clone();
    for boid in boids.iter_mut() {
        let mut new_velocity = nalgebra::Vector2::new(0.0, 0.0);
        new_velocity += cohesion(*boid, center_of_mass) * COHESION;
        new_velocity += separation(*boid, &boids_clone) * SEPARATION;
        new_velocity += alignment(*boid, &boids_clone) * ALIGNMENT;
        boid.velocity += new_velocity;
        boid.position += boid.velocity * dt * 1.0;

        fn torus(x: f32) -> f32 {
            if x.abs() > 1.0 {
                -x
            } else {
                x
            }
        }

        boid.position = nalgebra::Point2::new(torus(boid.position.x), torus(boid.position.y));
        boid.rotation = nalgebra::Rotation2::rotation_between(
            &nalgebra::Vector2::new(0.0, 1.0),
            &boid.velocity,
        );
    }
}

pub fn cohesion(boid: Boid, center_of_mass: Vector2<f32>) -> nalgebra::Vector2<f32> {
    center_of_mass - boid.position.coords
}

pub fn separation(boid: Boid, boids: &[Boid]) -> nalgebra::Vector2<f32> {
    let mut separation = nalgebra::Vector2::new(0.0, 0.0);
    for other_boid in boids {
        if nalgebra::distance(&boid.position, &other_boid.position) < 0.4 {
            separation -= other_boid.position.coords - boid.position.coords;
        }
    }
    separation
}

pub fn alignment(boid: Boid, boids: &[Boid]) -> nalgebra::Vector2<f32> {
    let average_velocity =
        boids.iter().map(|b| b.velocity).sum::<Vector2<f32>>() / boids.len() as f32;
    average_velocity - boid.velocity
}
