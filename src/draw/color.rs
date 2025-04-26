use crate::platform::math::Vec4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
    
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    
    pub fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let b = (hex & 0xFF) as f32 / 255.0;
        Self { r, g, b, a: 1.0 }
    }
    
    pub fn from_hex_str(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            if let Ok(hex) = u32::from_str_radix(hex, 16) {
                return Some(Self::from_hex(hex));
            }
        }
        None
    }
    
    pub fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }
    
    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
    
    pub fn black() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }
    }
    
    pub fn white() -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }
    }
    
    pub fn red() -> Self {
        Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 }
    }
    
    pub fn green() -> Self {
        Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 }
    }
    
    pub fn blue() -> Self {
        Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 }
    }
    
    pub fn transparent() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
    }
}



