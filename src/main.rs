use std::sync::Arc;

use winit::{event::WindowEvent, event_loop::EventLoop, window::Window};

async fn run(event_loop: EventLoop<()>, window: Window){

    let window = Arc::new(window);

    let size = window.inner_size();

    //wgpu api handle
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor{
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });

    //render target
    let surface = instance.create_surface(&window).unwrap();

    //physical gpu
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions{
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    }).await.unwrap();

    //api handles to interact with gpu and send to gpu
    let (device,queue) = adapter.request_device(&wgpu::DeviceDescriptor{
        label: None,
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        experimental_features: Default::default(),
        memory_hints: Default::default(),
        trace: wgpu::Trace::Off,
    }).await.unwrap();


    let swapchain_capabilites = surface.get_capabilities(&adapter);
    println!("Swapchain capabilities: {:?}", swapchain_capabilites);
    let swapchain_format = swapchain_capabilites.formats[0];
    println!("Swapchain format: {:?}", swapchain_format);

    let mut surface_config = surface.get_default_config(&adapter, size.width, size.height).unwrap();
    println!("Surface config: {:?}", surface_config);

    surface.configure(&device, &surface_config);
    
    let window = &window;

    //Game loop
    event_loop.run(move |event,win| {
       
       if let winit::event::Event::WindowEvent{window_id: _,event} = event{
        //println!("event: {:?}", event);

        match event {
            WindowEvent::CloseRequested => {
                //println!("The close button was pressed; stopping");
                win.exit();
            },
            WindowEvent::Resized(new_size) => {
                //println!("Window resized to: {:?}", new_size);
                surface_config.width = new_size.width.max(1);
                surface_config.height = new_size.height.max(1);
                surface.configure(&device, &surface_config);

                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                //println!("Redrawing window");
                //Redraw the window here
                window.request_redraw();
            }
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                //println!("Keyboard input: device_id: {:?}, event: {:?}, is_synthetic: {:?}", device_id, event, is_synthetic);
            }
            //Other window events here
            _ => {}
        }
       }
    }).unwrap();
}

fn main() {

    let event_loop = winit::event_loop::EventLoop::new().unwrap();

    let mut window_builder = winit::window::WindowBuilder::new();

    let window = window_builder.
    with_title("example1")
    .build(&event_loop).unwrap();

    //wgpu event logging
    env_logger::init();

    pollster::block_on(run(event_loop, window));


    println!("Hello, world!");
}
