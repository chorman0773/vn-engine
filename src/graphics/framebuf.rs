use core::{
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

use wgpu::{RenderPass, ShaderModule, TextureView};

use crate::graphics::{Colour, Renderable, Result};

pub struct LayerFramebuffer(Vec<Colour>);

impl LayerFramebuffer {
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.0)
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        bytemuck::cast_slice_mut(&mut self.0)
    }
}

impl<I: SliceIndex<[Colour]>> Index<I> for LayerFramebuffer {
    type Output = I::Output;

    fn index(&self, idx: I) -> &I::Output {
        &self.0[idx]
    }
}

impl<I: SliceIndex<[Colour]>> IndexMut<I> for LayerFramebuffer {
    fn index_mut(&mut self, idx: I) -> &mut I::Output {
        &mut self.0[idx]
    }
}

impl Renderable for LayerFramebuffer {
    type Output<'a> = ();

    fn render(&self, state: &mut super::RenderState) -> Result<()> {
        Ok(())
    }
}
