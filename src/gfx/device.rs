use std::iter;
use wasm_bindgen::prelude::*;
use winit::window::Window;

use std::fs::File;
use std::io::prelude::*;
use std::borrow::Cow;

use wgpu::util::DeviceExt;

use crate::gfx::mesh::*;
pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    render_pipeline: Option<wgpu::RenderPipeline>,

    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    num_indices: u32,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        // init renderer
        let (surface, device, queue, config, size) = State::init_renderer(window).await;

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline: None,
            vertex_buffer: None,
            index_buffer: None,
            num_indices: 0,
        }
    }

    async fn init_renderer(
        window: &Window,
    ) -> (
        wgpu::Surface,
        wgpu::Device,
        wgpu::Queue,
        wgpu::SurfaceConfiguration,
        winit::dpi::PhysicalSize<u32>,
    ) {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };

        let req_adaptor_options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };

        let adapter = instance
            .request_adapter(&req_adaptor_options)
            .await
            .unwrap();

        let device_desc = wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            },
        };

        let (device, queue) = adapter.request_device(&device_desc, None).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        (surface, device, queue, config, size)
    }

    pub fn create_mesh(&mut self) {
        let vb = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Vertex Buffer 0"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let ib = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
            label: Some("Index Buffer 0"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        self.vertex_buffer = Some(vb);
        self.index_buffer = Some(ib);
        self.num_indices = INDICES.len() as u32;
    }

    pub fn create_render_pipeline(&mut self) {
        // load shader
        let shader = self.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("shader 0"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let render_pipeline_layout =
            self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("render pipeline layout 0"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline 0"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState{
                    format: self.config.format,
                    blend: Some(wgpu::BlendState{
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        self.render_pipeline = Some(render_pipeline);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // command list
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("command list"),
            });

        // get back bufer
        let back_buffer = self.surface.get_current_texture()?;
        // clear back buffer
        {
            // get back buffer view
            let rt_view = back_buffer
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("clear back buffer"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &rt_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.8,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            };
            let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

            if let (Some(r), Some(vb), Some(ib)) = (&self.render_pipeline, &self.vertex_buffer, &self.index_buffer){
                render_pass.set_pipeline(r);
                render_pass.set_vertex_buffer(0, vb.slice(..));
                render_pass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
            }
        }

        self.queue.submit(iter::once(encoder.finish()));

        back_buffer.present();

        Ok(())
    }
}
