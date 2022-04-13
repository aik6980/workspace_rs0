use std::iter;
use wasm_bindgen::prelude::*;
use winit::window::Window;

pub struct State {
    surface : wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    render_pipeline: Option<wgpu::RenderPipeline>,
}

impl State {
    pub async fn new(window: &Window) -> Self {

        // init renderer
        let(surface, device, queue, config, size) = State::init_renderer(window).await;

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline: None,
        }
    }

    async fn init_renderer(window : &Window) -> (wgpu::Surface, wgpu::Device, wgpu::Queue, wgpu::SurfaceConfiguration, winit::dpi::PhysicalSize<u32>) {
        
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe{ instance.create_surface(window) };

        let req_adaptor_options = wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false, 
        };

        let adapter = instance.request_adapter(&req_adaptor_options).await.unwrap();

        let device_desc = wgpu::DeviceDescriptor{
            label: None,
            features: wgpu::Features::empty(),
            limits: 
                if cfg!(target_arch = "wasm32") {
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
//
    //fn create_render_pipeline(ref device: wgpu::Device) -> wgpu::RenderPipeline {
    //    // load shader
    //    let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor{
    //        label: Some("shader 0"),
    //        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into())
    //    });
    //    
    //    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
    //        label: Some("render pipeline layout 0"),
    //        bind_group_layouts: &[],
    //        push_constant_ranges: &[],
    //    });
//
    //    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
//
    //    });
    //}
//
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        
        // command list
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
            label: Some("command list"),
        });

        // get back bufer
        let back_buffer = self.surface.get_current_texture()?;
        // clear back buffer
        {
            // get back buffer view
            let rt_view = back_buffer.texture.create_view(&wgpu::TextureViewDescriptor::default());

            let render_pass_desc = wgpu::RenderPassDescriptor {
                label: Some("clear back buffer"),
                color_attachments: &[wgpu::RenderPassColorAttachment{
                    view: &rt_view,
                    resolve_target: None,
                    ops: wgpu::Operations { 
                        load: wgpu::LoadOp::Clear(wgpu::Color {r:0.8, g:0.2, b:0.3, a:1.0}), 
                        store: true, }
                }],
                depth_stencil_attachment: None,
            };
            let render_pass = encoder.begin_render_pass(&render_pass_desc);
        }

        self.queue.submit(iter::once(encoder.finish()));
        
        back_buffer.present();
        
        Ok(())
    }
}