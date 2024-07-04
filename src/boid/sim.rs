use std::{cell::RefCell, rc::Rc};

use super::Boid;

pub fn update_boids(boids: Rc<RefCell<Vec<Boid>>>, dt: f32) {
    let boid = boids.borrow_mut().pop().unwrap();
    let new_boid = Boid {
        position: boid.position + nalgebra::Vector2::new(8.0, 8.0) * dt,
        rotation: nalgebra::Rotation2::new(boid.rotation.angle() + std::f32::consts::PI * 9.0 * dt),
    };
    boids.borrow_mut().push(new_boid);
}
