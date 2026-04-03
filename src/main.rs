use std::sync::Arc;

use wgpu::RequestAdapterOptions;
use wgpu::wgc::device::queue;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{self, ControlFlow, OwnedDisplayHandle};
use winit::window::{Window, WindowId};

struct GPUState {
    window: Arc<Window>,
    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    win_size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
}

impl GPUState {
    pub async fn new(window: Arc<Window>, display: OwnedDisplayHandle) -> Self {
        //create gpu api instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(
            Box::new(display),
        ));

        //handle to actual gpu
        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .unwrap();

        //virtual gpu device and command queue
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        let win_size = window.inner_size();

        //swapchain surface for presenting rendered images to the window
        let surface = instance.create_surface(window.clone()).unwrap();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities.formats[0];

        let gpu_state = GPUState {
            window,
            instance,
            device,
            queue,
            win_size,
            surface,
            surface_format,
        };

        gpu_state.configure_surface();

        gpu_state
    }

    pub fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            width: self.window.inner_size().width,
            height: self.window.inner_size().height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            desired_maximum_frame_latency: 2,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.win_size = new_size;
        //request new swapchain image
        self.configure_surface();
    }

    fn render(&mut self) {
        //get image from swapchain to render to
        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(_) => {
                self.configure_surface();
                return;
            }
            wgpu::CurrentSurfaceTexture::Timeout => return,
            wgpu::CurrentSurfaceTexture::Occluded => return,
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.configure_surface();
                return;
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                self.surface = self.instance.create_surface(self.window.clone()).unwrap();
                self.configure_surface();
                return;
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                unreachable!("No error scope registered, so validation errors will panic")
            }
        };

        //create view of the swapchain image to render to
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()), //make sure in srgb space
                ..Default::default()
            });

        //create command encoder for recording render commands
        let mut encoder = self.device.create_command_encoder(&Default::default());

        //encode commands in this scope
        {
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
        }

        //basically compiles commands into a command buffer that can be submitted to the gpu
        let command_buffer = encoder.finish();

        self.queue.submit(Some(command_buffer));

        //after command submission but before presenting
        self.window.pre_present_notify();

        //present the swapchain image to the window
        surface_texture.present();
    }
}

#[derive(Default)]
struct App {
    gpu_state: Option<GPUState>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ApplicationHandler for App {
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let gpu_state = self.gpu_state.as_mut().unwrap();

        match event {
            WindowEvent::Resized(size) => {
                log::info!("Window resized to: {:?}", size);
                //todo: resize here
                gpu_state.resize(size);
            }
            WindowEvent::CloseRequested => {
                log::info!("Window close requested");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                log::info!("Window redraw requested");
                gpu_state.render();

                //ask for redraw since only manual and resize trigger redraw
                gpu_state.window.request_redraw();
            }
            _ => (),
        }
    }

    fn resumed(&mut self, event_loop: &event_loop::ActiveEventLoop) {
        log::info!("Application resumed");

        let window_attributes = winit::window::WindowAttributes::default()
            .with_title("mortar")
            .with_inner_size(winit::dpi::PhysicalSize::new(1200, 1000));

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        let gpu_state = pollster::block_on(GPUState::new(
            window.clone(),
            event_loop.owned_display_handle(),
        ));
        self.gpu_state = Some(gpu_state);
    }
}

fn main() {
    //set up logging for wgpu
    env_logger::init();

    let event_loop = event_loop::EventLoop::new().unwrap();

    //event loop checks for events even if queue is empty, for more responsive apps
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    event_loop.run_app(&mut app).unwrap();
}
