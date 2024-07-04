use std::sync::{atomic::AtomicBool, Arc};

use nalgebra::{Point2, Rotation2, Vector2};
use wgpu::{include_wgsl, SurfaceConfiguration};

use crate::srgb_to_linear;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct BoidVertex {
    pub position: [f32; 2],
}

pub struct Boid {
    pub position: Point2<f32>,
    pub rotation: Rotation2<f32>,
}

pub struct BoidRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    write_buffer: wgpu::Buffer,
    boids: Arc<Vec<Boid>>,
}
impl BoidRenderer {
    pub fn new(
        device: &wgpu::Device,
        config: &SurfaceConfiguration,
        boids: Arc<Vec<Boid>>,
    ) -> Self {
        let shader_module = device.create_shader_module(include_wgsl!("./test.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Test Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let vert_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Boid Vertex Buffer"),
            size: 3 * std::mem::size_of::<BoidVertex>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let write_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Boid Write Buffer"),
            size: 3 * std::mem::size_of::<BoidVertex>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::MAP_WRITE,
            mapped_at_creation: false,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Test pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vs",
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<BoidVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    }],
                }],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fs",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            multiview: None,
        });

        Self {
            pipeline,
            vertex_buffer: vert_buffer,
            write_buffer,
            boids,
        }
    }

    pub fn update(&mut self) {
        let boid = &self.boids[0];
        let verts = [
            Vector2::new(0.2, 0.0),
            Vector2::new(0.0, 0.2),
            Vector2::new(0.0, 0.0),
        ]
        .map(|vert| (boid.rotation * vert) + boid.position.coords)
        .map(|vert| BoidVertex {
            position: vert.into(),
        });

        let finished = Arc::new(AtomicBool::new(false));
        let finished_clone = finished.clone();
        self.write_buffer
            .slice(..)
            .map_async(wgpu::MapMode::Write, move |_| {
                println!("Buffer map finished");
                finished.store(true, std::sync::atomic::Ordering::SeqCst);
            });

        while !finished_clone.load(std::sync::atomic::Ordering::SeqCst) {
            // println!("Waiting for buffer map")
        }
        self.write_buffer
            .slice(..)
            .get_mapped_range_mut()
            .copy_from_slice(bytemuck::cast_slice(&verts));
        self.write_buffer.unmap();
    }

    pub fn encode(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        encoder.copy_buffer_to_buffer(
            &self.write_buffer,
            0,
            &self.vertex_buffer,
            0,
            3 * std::mem::size_of::<BoidVertex>() as wgpu::BufferAddress,
        );

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Test Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(srgb_to_linear(0x191E1F)),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_pipeline(&self.pipeline);
        pass.draw(0..3, 0..1);
    }
}
