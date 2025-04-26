use crate::math::Mat4;
use crate::shader::ShaderId;
use crate::geometry::GeometryId;
use crate::texture::Texture;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DrawListId(pub usize);

#[derive(Clone, Debug)]
pub struct DrawItem {
    pub shader_id: ShaderId,
    pub geometry_id: GeometryId,
    pub uniforms: Vec<DrawUniform>,
    pub textures: Vec<Texture>,
    pub instance_count: u32,
}

#[derive(Clone, Debug)]
pub enum DrawUniform {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Mat4([f32; 16]),
}

#[derive(Clone, Debug)]
pub struct DrawList {
    pub draw_list_id: DrawListId,
    pub draw_items: Vec<DrawItem>,
    pub view_transform: Mat4,
}

impl DrawList {
    pub fn new(draw_list_id: DrawListId) -> Self {
        Self {
            draw_list_id,
            draw_items: Vec::new(),
            view_transform: Mat4::identity(),
        }
    }
    
    pub fn add_draw_item(&mut self, draw_item: DrawItem) {
        self.draw_items.push(draw_item);
    }
    
    pub fn clear(&mut self) {
        self.draw_items.clear();
    }
    
    pub fn set_view_transform(&mut self, view_transform: Mat4) {
        self.view_transform = view_transform;
    }
}
