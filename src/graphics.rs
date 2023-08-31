use wgpu::{
    Buffer, BufferDescriptor, BufferUsages, Color, CommandBuffer, CommandEncoder,
    CommandEncoderDescriptor, Device, Operations, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, Surface, Texture, TextureDimension, TextureFormat, TextureViewDescriptor,
};
use winit::dpi::PhysicalSize;

pub mod framebuf;
pub mod image;
pub mod layer;

pub type Result<T> = core::result::Result<T, wgpu::SurfaceError>;

#[repr(C, align(4))]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Colour {
    pub const WHITE: Colour = Colour {
        r: !0,
        g: !0,
        b: !0,
        a: !0,
    };

    pub const HALFWHITE: Colour = Colour {
        r: 0xBF,
        g: 0xBF,
        b: 0xBF,
        a: 0xFF,
    };
}

#[repr(C, align(4))]
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct ScreenDimension {
    pub width: u32,
    pub height: u32,
}
impl From<PhysicalSize<u32>> for ScreenDimension {
    fn from(value: PhysicalSize<u32>) -> Self {
        let PhysicalSize { width, height } = value;

        Self { width, height }
    }
}

impl From<Colour> for Color {
    fn from(value: Colour) -> Self {
        Self {
            r: value.r as f64 / 255.0,
            g: value.g as f64 / 255.0,
            b: value.b as f64 / 255.0,
            a: value.a as f64 / 255.0,
        }
    }
}

pub trait Renderable {
    type Output<'a>
    where
        Self: 'a;

    fn render(&self, state: &mut RenderState) -> Result<Self::Output<'_>>;
}

impl<I, F: Fn(&mut RenderState) -> Result<I>> Renderable for F {
    type Output<'a> = I where Self: 'a;

    fn render(&self, state: &mut RenderState) -> Result<Self::Output<'_>> {
        self(state)
    }
}

pub struct GraphicsState {
    device: Device,
    surface: Surface,
    queue: Queue,
    screen_dimension: ScreenDimension,
}

impl GraphicsState {
    pub fn new(device: Device, surface: Surface, queue: Queue, dim: ScreenDimension) -> Self {
        Self {
            device,
            surface,
            queue,
            screen_dimension: dim,
        }
    }

    pub fn set_dimension(&mut self, dim: ScreenDimension) {
        self.screen_dimension = dim.into();
    }

    pub fn render_with<F: Fn(&mut RenderState) -> Result<()>>(&mut self, f: F) -> Result<()> {
        self.render(&f)
    }

    pub fn render<'a, R: Renderable>(&mut self, target: &'a R) -> Result<R::Output<'a>> {
        let encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Pass"),
            });
        let surface_texture = self.surface.get_current_texture()?;

        let texture = &surface_texture.texture;

        let mut state = RenderState {
            inner: self,
            cmd_encoder: encoder,
            texture,
        };

        let res = target.render(&mut state)?;

        let cmds = state.cmd_encoder;

        self.queue.submit(core::iter::once(cmds.finish()));

        surface_texture.present();

        Ok(res)
    }
}

pub struct RenderState<'a> {
    inner: &'a mut GraphicsState,
    cmd_encoder: CommandEncoder,
    texture: &'a Texture,
}

impl<'a> RenderState<'a> {
    pub fn draw_solid_color(&mut self, colour: Colour) -> Result<()> {
        let view = self.texture.create_view(&TextureViewDescriptor {
            label: Some("draw colour view"),
            format: Some(TextureFormat::Bgra8Unorm),
            dimension: Some(wgpu::TextureViewDimension::D2),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });
        let render = self.cmd_encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Draw Solid Colour"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: wgpu::LoadOp::Clear(colour.into()),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        Ok(())
    }
}
