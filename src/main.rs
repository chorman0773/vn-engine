use std::path::Path;

use futures::future::FutureExt;

use wgpu::{
    Backends, CommandEncoderDescriptor, DeviceDescriptor, Dx12Compiler, Extent3d, Features,
    ImageCopyTexture, Instance, InstanceDescriptor, Label, Limits, Origin3d, PowerPreference,
    RenderPassDescriptor, RequestAdapterOptions, SurfaceConfiguration, TextureDescriptor,
    TextureFormat, TextureUsages,
};
use winit::{event_loop::EventLoop, window::Window};

mod script;

fn main() {
    let entry_point = Path::new("main.vns");

    let mut args = std::env::args();

    let prg_name = args.next().unwrap();

    let mut backends = Backends::PRIMARY;

    let mut dx12_compiler = None::<Dx12Compiler>;

    let mut power_preference = PowerPreference::LowPower;

    while let Some(arg) = args.next() {
        match &*arg {
            "--enable-wpgu-secondary-backeds" => backends |= Backends::SECONDARY,
            "--wgpu-backend" => {
                let input = args.next().unwrap_or_else(|| {
                    eprintln!("{}: --wpgu-backend requires an argument", prg_name);
                    std::process::exit(1)
                });
                backends = Backends::empty();

                for backend in input.split(',') {
                    match backend {
                        "vulkan" => backends |= Backends::VULKAN,
                        "dx11" => backends |= Backends::DX11,
                        "dx12" => backends |= Backends::DX12,
                        "opengl" | "gl" => backends |= Backends::GL,
                        "metal" => backends |= Backends::METAL,
                        name => {
                            eprintln!("{}: Unknown wgpu backend {}", prg_name, name);
                            std::process::exit(1)
                        }
                    }
                }
            }
            "--wgpu-use-high-adaptor" => power_preference = PowerPreference::HighPerformance,
            "--wpgu-use-low-adaptor" => power_preference = PowerPreference::LowPower,
            "--use-dxc-compiler" => {
                dx12_compiler = Some(Dx12Compiler::Dxc {
                    dxil_path: std::env::var_os("VNENGINE_WGPU_DXCOMPILER").map(Into::into),
                    dxc_path: std::env::var_os("VNENGINE_WGPU_DXIL").map(Into::into),
                })
            }
            "--use-fxc-compiler" => {
                dx12_compiler = Some(Dx12Compiler::Fxc);
            }
            opt => {
                eprintln!("{}: Unknown option {}", prg_name, opt);
                std::process::exit(1)
            }
        }
    }

    let dx12_shader_compiler = if let Some(dx12_compiler) = dx12_compiler {
        dx12_compiler
    } else {
        Dx12Compiler::Fxc
    };

    let config = InstanceDescriptor {
        backends,
        dx12_shader_compiler,
    };

    let instance = Instance::new(config);

    let eloop = EventLoop::new();

    let window = Window::new(&eloop).unwrap_or_else(|e| {
        eprintln!("{}: Failed to open game window, {}.", prg_name, e);
        std::process::exit(1)
    });

    let surface = unsafe { instance.create_surface(&window) }.unwrap_or_else(|e| {
        eprintln!("{}: Failed to create game surface, {}.", prg_name, e);
        std::process::exit(1)
    });

    let options = RequestAdapterOptions {
        power_preference,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    };

    let adapter = instance.request_adapter(&options);

    let adapter = adapter.now_or_never().flatten().unwrap_or_else(|| {
        eprintln!("{}: Could not find a suitable display adaptor", prg_name);
        std::process::exit(1)
    });

    let device_desc = DeviceDescriptor {
        label: None,
        features: Features::empty(),
        limits: Limits::default(),
    };

    let (device, queue) = adapter
        .request_device(&device_desc, None)
        .now_or_never()
        .unwrap()
        .unwrap_or_else(|e| {
            eprintln!("{}: Could not obtain device for window, {}.", prg_name, e);
            std::process::exit(1)
        });

    let mut cur_dimensions = window.inner_size();

    let mut texture_desc = TextureDescriptor {
        label: Some("background"),
        size: Extent3d {
            width: 1920,
            height: 1080,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: TextureUsages::all(),
        view_formats: &[TextureFormat::Rgba8Unorm],
    };

    let config = SurfaceConfiguration {
        usage: Surface,
        format: (),
        width: (),
        height: (),
        present_mode: (),
        alpha_mode: (),
        view_formats: (),
    };

    surface.configure(&device, &config);

    eloop.run(|event, targ, cf| match event {
        winit::event::Event::NewEvents(_) => {}
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::Resized(size) => {
                cur_dimensions = size;
            }
            winit::event::WindowEvent::Moved(_) => {}
            winit::event::WindowEvent::CloseRequested => {
                std::process::exit(0);
            }
            winit::event::WindowEvent::Destroyed => {}
            winit::event::WindowEvent::DroppedFile(_) => {}
            winit::event::WindowEvent::HoveredFile(_) => {}
            winit::event::WindowEvent::HoveredFileCancelled => {}
            winit::event::WindowEvent::ReceivedCharacter(_) => {}
            winit::event::WindowEvent::Focused(_) => {}
            winit::event::WindowEvent::KeyboardInput { .. } => {}
            winit::event::WindowEvent::ModifiersChanged(_) => {}
            winit::event::WindowEvent::Ime(_) => {}
            winit::event::WindowEvent::CursorMoved { .. } => {}
            winit::event::WindowEvent::CursorEntered { .. } => {}
            winit::event::WindowEvent::CursorLeft { .. } => {}
            winit::event::WindowEvent::MouseWheel { .. } => {}
            winit::event::WindowEvent::MouseInput { .. } => {}
            winit::event::WindowEvent::TouchpadMagnify { .. } => {}
            winit::event::WindowEvent::SmartMagnify { .. } => {}
            winit::event::WindowEvent::TouchpadRotate { .. } => {}
            winit::event::WindowEvent::TouchpadPressure { .. } => {}
            winit::event::WindowEvent::AxisMotion { .. } => {}
            winit::event::WindowEvent::Touch(_) => {}
            winit::event::WindowEvent::ScaleFactorChanged { .. } => {}
            winit::event::WindowEvent::ThemeChanged(_) => {}
            winit::event::WindowEvent::Occluded(_) => {}
        },
        winit::event::Event::DeviceEvent { .. } => {}
        winit::event::Event::UserEvent(_) => {}
        winit::event::Event::Suspended => {}
        winit::event::Event::Resumed => {}
        winit::event::Event::MainEventsCleared => {}
        winit::event::Event::RedrawRequested(_) => {
            queue.submit(core::iter::once_with(|| {
                let desc = CommandEncoderDescriptor {
                    label: Some("draw background"),
                };
                let mut enc = device.create_command_encoder(&desc);

                let render_desc = RenderPassDescriptor {
                    label: Some("draw background pass"),
                    color_attachments: &[],
                };

                let mut render = enc.begin_render_pass(&render_desc);

                enc.finish()
            }));
        }
        winit::event::Event::RedrawEventsCleared => {}
        winit::event::Event::LoopDestroyed => std::process::exit(0),
    })
}
