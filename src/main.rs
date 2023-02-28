use std::path::Path;

use futures::future::FutureExt;

use wgpu::{Dx12Compiler,Backends, InstanceDescriptor, Instance, RequestAdapterOptions, PowerPreference, DeviceDescriptor, Features, Limits};
use winit::{event_loop::EventLoop, window::Window};


mod script;



fn main() {
    let entry_point = Path::new("main.vns");

    let mut args = std::env::args();

    let prg_name = args.next().unwrap();

    let mut backends = Backends::PRIMARY;

    let mut dx12_compiler = None::<Dx12Compiler>;

    let mut power_preference = PowerPreference::LowPower;

    while let Some(arg) = args.next(){
        match &*arg{
            "--enable-wpgu-secondary-backeds" => backends |= Backends::SECONDARY,
            "--wgpu-backend" => {
                let input = args.next().unwrap_or_else(||{
                    eprintln!("{}: --wpgu-backend requires an argument",prg_name);
                    std::process::exit(1)
                });
                backends = Backends::empty();

                for backend in input.split(','){
                    match backend{
                        "vulkan" => backends |= Backends::VULKAN,
                        "dx11" => backends |= Backends::DX11,
                        "dx12" => backends |= Backends::DX12,
                        "opengl" | "gl" => backends |= Backends::GL,
                        "metal" => backends |= Backends::METAL,
                        name => {
                            eprintln!("{}: Unknown wgpu backend {}",prg_name,backend);
                            std::process::exit(1)
                        }
                    }
                }
            }
            "--wgpu-use-high-adaptor" => power_preference = PowerPreference::HighPerformance,
            "--wpgu-use-low-adaptor" => power_preference = PowerPreference::LowPower,
            opt => {
                eprintln!("{}: Unknown option {}",prg_name,opt);
                std::process::exit(1)
            }
        }
    }


    let dx12_shader_compiler = dx12_compiler.unwrap_or(Dx12Compiler::Fxc);

    let config = InstanceDescriptor{
        backends,
        dx12_shader_compiler,
    };

    let instance = Instance::new(config);

    let eloop = EventLoop::new();

    let window = Window::new(&eloop)
        .unwrap_or_else(|e|{
            eprintln!("{}: Failed to open game window, {}.", prg_name, e);
            std::process::exit(1)
        });

    let surface = unsafe{instance.create_surface(&window)}
        .unwrap_or_else(|e|{
            eprintln!("{}: Failed to create game surface, {}.",prg_name,e);
            std::process::exit(1)
        });

    let options = RequestAdapterOptions{
        power_preference,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    };
    
    let adapter = instance.request_adapter(&options);

    let adapter = adapter.now_or_never()
        .flatten()
        .unwrap_or_else(||{
            eprintln!("{}: Could not find a suitable display adaptor",prg_name);
            std::process::exit(1)
        });


    let device_desc = DeviceDescriptor{
        label: None,
        features: Features::empty(),
        limits: Limits::default(),
    };

    let (device,queue) = adapter.request_device(&device_desc, None)
        .now_or_never()
        .unwrap()
        .unwrap_or_else(|e|{
            eprintln!("{}: Could not obtain device for window, {}.",prg_name,e);
            std::process::exit(1)
        });


    eloop.run(|event,targ,cf|{
        match event{
            winit::event::Event::NewEvents(_) => {},
            winit::event::Event::WindowEvent {  event, .. } => match event{
                winit::event::WindowEvent::Resized(_) => {},
                winit::event::WindowEvent::Moved(_) => {},
                winit::event::WindowEvent::CloseRequested => {
                    std::process::exit(0);
                },
                winit::event::WindowEvent::Destroyed => {},
                winit::event::WindowEvent::DroppedFile(_) => {},
                winit::event::WindowEvent::HoveredFile(_) => {},
                winit::event::WindowEvent::HoveredFileCancelled => {},
                winit::event::WindowEvent::ReceivedCharacter(_) => {},
                winit::event::WindowEvent::Focused(_) => {},
                winit::event::WindowEvent::KeyboardInput { .. } => {},
                winit::event::WindowEvent::ModifiersChanged(_) => {},
                winit::event::WindowEvent::Ime(_) => {},
                winit::event::WindowEvent::CursorMoved {.. } => {},
                winit::event::WindowEvent::CursorEntered { .. } => {},
                winit::event::WindowEvent::CursorLeft { .. } => {},
                winit::event::WindowEvent::MouseWheel { .. } => {},
                winit::event::WindowEvent::MouseInput { .. } => {},
                winit::event::WindowEvent::TouchpadMagnify { .. } => {},
                winit::event::WindowEvent::SmartMagnify { .. } => {},
                winit::event::WindowEvent::TouchpadRotate { .. } => {},
                winit::event::WindowEvent::TouchpadPressure { .. } => {},
                winit::event::WindowEvent::AxisMotion { .. } => {},
                winit::event::WindowEvent::Touch(_) => {},
                winit::event::WindowEvent::ScaleFactorChanged { .. } => {},
                winit::event::WindowEvent::ThemeChanged(_) => {},
                winit::event::WindowEvent::Occluded(_) => {},
            },
            winit::event::Event::DeviceEvent { .. } => {},
            winit::event::Event::UserEvent(_) => {},
            winit::event::Event::Suspended => {},
            winit::event::Event::Resumed => {},
            winit::event::Event::MainEventsCleared => {},
            winit::event::Event::RedrawRequested(_) => {
            },
            winit::event::Event::RedrawEventsCleared => {},
            winit::event::Event::LoopDestroyed => std::process::exit(0),
        }
    })
    
}
