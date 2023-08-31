use super::{Colour, ScreenDimension};

pub enum Layer {
    SolidColour(Colour),
    FrameBuffer(Vec<Colour>, ScreenDimension),
}
