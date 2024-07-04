mod renderer;
mod boid_renderer;
mod boid_sim;

use std::sync::Arc;

use boid_renderer::Boid;
use boid_sim::update_boids;
use nalgebra::Rotation2;
use pollster::FutureExt as _;
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, window::Window,
};

pub enum Handler<'a> {
    Loading,
    Running {
        renderer: renderer::Renderer<'a>,
        window: Arc<Window>,
        boids: Arc<Vec<Boid>>,
    },
}
impl<'a> ApplicationHandler for Handler<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Handler::Loading = self {
            let window = Arc::new(
                event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_title("woids")
                            .with_inner_size(LogicalSize::new(800, 600)),
                    )
                    .unwrap(),
            );
            let boids = Arc::new(vec![Boid { position: [0.5; 2].into(), rotation: Rotation2::identity() }]);
            let renderer = renderer::Renderer::new(window.clone(), boids.clone()).block_on().unwrap();
            *self = Handler::Running { renderer, window, boids };
        }
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Handler::Running { renderer, window, boids } = self {
            if window_id != window.id() {
                return;
            }

            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    update_boids(boids.clone());
                    renderer.render();
                }
                WindowEvent::Resized(size) => {
                    renderer.resize(size);
                }
                _ => {}
            }
        }
    }
}

pub fn srgb_to_linear(color: u32) -> wgpu::Color {
    fn channel(c: u32) -> f64 {
        (((c as f64) / 255.0 + 0.055) / 1.055).powf(2.4)
    }
    let r = (color >> 16) & 0xff;
    let g = (color >> 8) & 0xff;
    let b = color & 0xff;
    wgpu::Color {
        r: channel(r),
        g: channel(g),
        b: channel(b),
        a: 1.0,
    }
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    event_loop.run_app(&mut Handler::Loading).unwrap();
}
