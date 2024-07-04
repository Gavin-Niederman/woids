mod boid;
mod renderer;

use std::{cell::RefCell, rc::Rc, sync::Arc, time::Instant};

use boid::{initialize_boids, sim::update_boids, Boid};
use pollster::FutureExt as _;
use winit::{
    application::ApplicationHandler, dpi::LogicalSize, event::WindowEvent, event_loop::ControlFlow,
    window::Window,
};

pub enum Handler<'a> {
    Loading,
    Running {
        renderer: renderer::Renderer<'a>,
        window: Arc<Window>,
        boids: Rc<RefCell<Vec<Boid>>>,
        last_time: Instant,
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
            let boids = Rc::new(RefCell::new(initialize_boids()));
            let renderer = renderer::Renderer::new(window.clone(), boids.clone())
                .block_on()
                .unwrap();
            let last_time = Instant::now();
            *self = Handler::Running {
                renderer,
                window,
                boids,
                last_time,
            };
        }
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Handler::Running {
            renderer,
            window,
            boids,
            last_time,
        } = self
        {
            if window_id != window.id() {
                return;
            }

            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::RedrawRequested => {
                    update_boids(boids.clone(), last_time.elapsed().as_secs_f32());
                    renderer.render();
                    *last_time = Instant::now();
                }
                WindowEvent::Resized(size) => {
                    renderer.resize(size);
                }
                _ => {}
            }
        }
    }
    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
        if let Self::Running {
            renderer: _,
            window,
            boids: _,
            last_time: _,
        } = self
        {
            window.request_redraw();
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
