use crate::math::Vec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TextureId(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextureFormat {
    Rgba8,
    Bgra8,
    Rgb8,
    Bgr8,
    R8,
    Depth32,
}

#[derive(Clone, Debug)]
pub struct Texture {
    pub texture_id: TextureId,
    pub width: usize,
    pub height: usize,
    pub format: TextureFormat,
}

impl Texture {
    pub fn new(texture_id: TextureId, width: usize, height: usize, format: TextureFormat) -> Self {
        Self {
            texture_id,
            width,
            height,
            format,
        }
    }
    
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width as f32, self.height as f32)
    }
}
