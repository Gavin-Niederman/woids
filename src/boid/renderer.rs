use std::{cell::RefCell, rc::Rc};

use nalgebra::Vector2;
use wgpu::{include_wgsl, util::DeviceExt, SurfaceConfiguration};

use crate::srgb_to_linear;

use super::{Boid, NUM_BOIDS, NUM_VERTS};

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Debug)]
pub struct BoidVertex {
    pub position: [f32; 2],
}

pub struct BoidRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    boids: Rc<RefCell<Vec<Boid>>>,
}
impl BoidRenderer {
    pub fn new(
        device: &wgpu::Device,
        config: &SurfaceConfiguration,
        boids: Rc<RefCell<Vec<Boid>>>,
    ) -> Self {
        let shader_module = device.create_shader_module(include_wgsl!("../test.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Test Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let vert_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Boid Vertex Buffer"),
            size: NUM_VERTS * NUM_BOIDS * std::mem::size_of::<BoidVertex>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
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
            boids,
        }
    }

    pub fn update_vertex_buffer(&self, device: &mut wgpu::Device, queue: &mut wgpu::Queue) {
        let verts: Vec<_> = self
            .boids
            .borrow()
            .clone()
            .into_iter()
            .flat_map(|boid| {
                [
                    Vector2::new(0.02, -0.05),
                    Vector2::new(0.0, 0.05),
                    Vector2::new(-0.02, -0.05),
                ]
                .map(|vert| (boid.rotation * vert) + boid.position.coords)
                .map(|vert| BoidVertex {
                    position: vert.into(),
                })
            })
            .collect();

        let staging_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Staging Buffer"),
            contents: bytemuck::cast_slice(&verts),
            usage: wgpu::BufferUsages::COPY_SRC,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Boid Vertex Copy Encoder"),
        });
        encoder.copy_buffer_to_buffer(
            &staging_buffer,
            0,
            &self.vertex_buffer,
            0,
            NUM_VERTS * NUM_BOIDS * std::mem::size_of::<BoidVertex>() as wgpu::BufferAddress,
        );
        let buf = encoder.finish();
        queue.submit(std::iter::once(buf));
    }

    pub fn render(
        &self,
        device: &mut wgpu::Device,
        queue: &mut wgpu::Queue,
        view: &wgpu::TextureView,
    ) {
        self.update_vertex_buffer(device, queue);

        let mut pass_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Boid Render Pass Encoder"),
        });

        {
            let mut pass = pass_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
            pass.draw(0..(NUM_BOIDS * NUM_VERTS) as _, 0..1);
        }

        let buf = pass_encoder.finish();
        queue.submit(std::iter::once(buf));
    }
}
