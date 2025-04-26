use crate::platform::math::Vec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WindowId(pub usize);

#[derive(Clone, Debug)]
pub struct WindowHandle {
    pub window_id: WindowId,
    pub position: Vec2,
    pub inner_size: Vec2,
    pub outer_size: Vec2,
    pub dpi_factor: f32,
    pub title: String,
    pub is_fullscreen: bool,
    pub is_topmost: bool,
}

impl WindowHandle {
    pub fn new(window_id: WindowId) -> Self {
        Self {
            window_id,
            position: Vec2::zero(),
            inner_size: Vec2::new(800.0, 600.0),
            outer_size: Vec2::new(800.0, 600.0),
            dpi_factor: 1.0,
            title: "mix Window".to_string(),
            is_fullscreen: false,
            is_topmost: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct WindowGeom {
    pub position: Vec2,
    pub inner_size: Vec2,
    pub outer_size: Vec2,
    pub dpi_factor: f32,
}



