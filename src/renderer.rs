use std::sync::Arc;

use crate::boid_renderer::{Boid, BoidRenderer};

pub struct Renderer<'a> {
    surface: wgpu::Surface<'a>,
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    boid_renderer: BoidRenderer,
}
impl<'a> Renderer<'a> {
    pub async fn new(
        window: Arc<winit::window::Window>,
        boids: Arc<Vec<Boid>>,
    ) -> Result<Self, wgpu::Error> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("GPU"),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let boid_renderer = BoidRenderer::new(&device, &config, boids);

        Ok(Renderer {
            surface,
            config,
            device,
            queue,
            boid_renderer,
        })
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self) {
        self.boid_renderer.update();

        let frame = self.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&Default::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Test Pass Encoder"),
            });

        {
            self.boid_renderer.encode(&mut encoder, &view);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();
    }
}
