use std::sync::Arc;

use wgpu::{
    Adapter, Backends, Device, DeviceDescriptor, Instance, InstanceDescriptor, Queue,
    RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceTargetUnsafe, TextureFormat,
    TextureUsages,
};
use winit::{
    dpi::PhysicalSize,
    event::Event,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct App {
    instance: Instance,
    surface: Surface<'static>,
    surface_format: TextureFormat,
    surface_config: SurfaceConfiguration,
    adapter: Adapter,
    device: Device,
    queue: Queue,

    // Must be declared after surface.
    window: Window,
    size: PhysicalSize<u32>,
}

impl App {
    pub async fn new(window: Window) -> Self {
        // Get current size
        let size = window.inner_size();

        // Instance
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..Default::default()
        });

        // Surface
        let surface = unsafe {
            instance.create_surface_unsafe(SurfaceTargetUnsafe::from_window(&window).unwrap())
        }
        .unwrap();

        // Adapter
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // Device Queue
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        App {
            instance,
            surface,
            surface_config,
            surface_format,
            adapter,
            device,
            queue,

            window,
            size,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        println!("Application Resized to {:?}", new_size);
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(false)
        .with_title("Celestial")
        .build(&event_loop)
        .unwrap();

    let window_id = window.id();

    let mut app: App = pollster::block_on(App::new(window));

    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop
        .run(move |event, target| match event {
            Event::WindowEvent {
                event,
                window_id: id,
            } => {
                if id == window_id {
                    match event {
                        WindowEvent::CloseRequested => {
                            println!("Close Requested");
                            target.exit();
                        }
                        WindowEvent::Resized(physical_size) => {
                            app.resize(physical_size);
                        }
                        WindowEvent::ScaleFactorChanged {
                            mut inner_size_writer,
                            ..
                        } => {
                            inner_size_writer.request_inner_size(app.size).unwrap();
                            app.resize(app.size);
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        })
        .unwrap();
}
