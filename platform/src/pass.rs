use crate::math::Vec4;
use crate::window::WindowId;
use crate::draw_list::DrawListId;
use crate::texture::Texture;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PassId(pub usize);

#[derive(Clone, Debug)]
pub enum PassParent {
    None,
    Window(WindowId),
    Pass(PassId),
}

#[derive(Clone, Debug)]
pub struct PassClearColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl PassClearColor {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }
}

#[derive(Clone, Debug)]
pub struct PassClearDepth {
    pub depth: f32,
}

impl PassClearDepth {
    pub fn new(depth: f32) -> Self {
        Self { depth }
    }
}

#[derive(Clone, Debug)]
pub struct Pass {
    pub pass_id: PassId,
    pub parent: PassParent,
    pub main_draw_list_id: Option<DrawListId>,
    pub clear_color: Option<PassClearColor>,
    pub clear_depth: Option<PassClearDepth>,
    pub color_texture: Option<Texture>,
    pub depth_texture: Option<Texture>,
    pub dpi_factor: Option<f32>,
    pub zbias_step: f32,
}

impl Pass {
    pub fn new(pass_id: PassId) -> Self {
        Self {
            pass_id,
            parent: PassParent::None,
            main_draw_list_id: None,
            clear_color: None,
            clear_depth: None,
            color_texture: None,
            depth_texture: None,
            dpi_factor: None,
            zbias_step: 0.001,
        }
    }

    pub fn set_window_parent(&mut self, window_id: WindowId) {
        self.parent = PassParent::Window(window_id);
    }

    pub fn set_pass_parent(&mut self, pass_id: PassId) {
        self.parent = PassParent::Pass(pass_id);
    }

    pub fn set_main_draw_list(&mut self, draw_list_id: DrawListId) {
        self.main_draw_list_id = Some(draw_list_id);
    }

    pub fn set_clear_color(&mut self, clear_color: PassClearColor) {
        self.clear_color = Some(clear_color);
    }

    pub fn set_clear_depth(&mut self, clear_depth: PassClearDepth) {
        self.clear_depth = Some(clear_depth);
    }

    pub fn set_color_texture(&mut self, texture: Texture) {
        self.color_texture = Some(texture);
    }

    pub fn set_depth_texture(&mut self, texture: Texture) {
        self.depth_texture = Some(texture);
    }

    pub fn set_dpi_factor(&mut self, dpi_factor: f32) {
        self.dpi_factor = Some(dpi_factor);
    }
}
