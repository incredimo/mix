// No need to import Vec2 directly as it's used through the rect module
use mix_platform::geometry::{Geometry, GeometryId, VertexFormat};
use mix_platform::shader::{Shader, ShaderId};
use mix_platform::draw_list::{DrawItem, DrawUniform};
use mix_platform::Cx;
use crate::color::Color;
use crate::rect::Rect;

#[derive(Clone, Debug)]
pub struct DrawQuad {
    pub color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub corner_radius: f32,
    pub shader_id: Option<ShaderId>,
    pub geometry_id: Option<GeometryId>,
}

impl Default for DrawQuad {
    fn default() -> Self {
        Self {
            color: Color::white(),
            border_color: Color::transparent(),
            border_width: 0.0,
            corner_radius: 0.0,
            shader_id: None,
            geometry_id: None,
        }
    }
}

impl DrawQuad {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_border_color(mut self, border_color: Color) -> Self {
        self.border_color = border_color;
        self
    }

    pub fn with_border_width(mut self, border_width: f32) -> Self {
        self.border_width = border_width;
        self
    }

    pub fn with_corner_radius(mut self, corner_radius: f32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    pub fn init(&mut self, cx: &mut Cx) {
        if self.shader_id.is_none() {
            let shader_id = cx.create_shader();
            let mut shader = Shader::new(shader_id);

            shader.set_vertex_shader(QUAD_VERTEX_SHADER);
            shader.set_fragment_shader(QUAD_FRAGMENT_SHADER);

            shader.add_uniform("color", mix_platform::shader::ShaderUniformType::Vec4);
            shader.add_uniform("border_color", mix_platform::shader::ShaderUniformType::Vec4);
            shader.add_uniform("border_width", mix_platform::shader::ShaderUniformType::Float);
            shader.add_uniform("corner_radius", mix_platform::shader::ShaderUniformType::Float);
            shader.add_uniform("size", mix_platform::shader::ShaderUniformType::Vec2);

            cx.shaders.insert(shader_id, shader);
            self.shader_id = Some(shader_id);
        }

        if self.geometry_id.is_none() {
            let geometry_id = cx.create_geometry();
            let mut geometry = Geometry::new(geometry_id);

            // Define quad vertices
            let vertices: Vec<f32> = vec![
                0.0, 0.0,  // Position
                0.0, 0.0,  // UV

                1.0, 0.0,  // Position
                1.0, 0.0,  // UV

                1.0, 1.0,  // Position
                1.0, 1.0,  // UV

                0.0, 1.0,  // Position
                0.0, 1.0,  // UV
            ];

            // Convert to bytes
            let vertices_bytes: Vec<u8> = unsafe {
                std::slice::from_raw_parts(
                    vertices.as_ptr() as *const u8,
                    vertices.len() * std::mem::size_of::<f32>(),
                ).to_vec()
            };

            // Define indices
            let indices: Vec<u16> = vec![
                0, 1, 2,  // First triangle
                0, 2, 3,  // Second triangle
            ];

            geometry.add_vertex_attribute("position", 0, VertexFormat::Float2);
            geometry.add_vertex_attribute("uv", 8, VertexFormat::Float2);

            geometry.set_vertices(vertices_bytes);
            geometry.set_indices(indices);

            cx.geometries.insert(geometry_id, geometry);
            self.geometry_id = Some(geometry_id);
        }
    }

    pub fn draw(&mut self, cx: &mut Cx, draw_list_id: mix_platform::draw_list::DrawListId, rect: &Rect) {
        if self.shader_id.is_none() || self.geometry_id.is_none() {
            self.init(cx);
        }

        let draw_item = DrawItem {
            shader_id: self.shader_id.unwrap(),
            geometry_id: self.geometry_id.unwrap(),
            uniforms: vec![
                DrawUniform::Vec4(self.color.to_array()),
                DrawUniform::Vec4(self.border_color.to_array()),
                DrawUniform::Float(self.border_width),
                DrawUniform::Float(self.corner_radius),
                DrawUniform::Vec2([rect.width(), rect.height()]),
            ],
            textures: Vec::new(),
            instance_count: 1,
        };

        if let Some(draw_list) = cx.draw_lists.get_mut(&draw_list_id) {
            draw_list.add_draw_item(draw_item);
        }
    }
}

const QUAD_VERTEX_SHADER: &str = r#"
#version 300 es
precision highp float;

in vec2 position;
in vec2 uv;

uniform vec2 size;
uniform mat4 view_transform;

out vec2 v_uv;
out vec2 v_size;

void main() {
    v_uv = uv;
    v_size = size;
    vec2 pos = position * size;
    gl_Position = view_transform * vec4(pos, 0.0, 1.0);
}
"#;

const QUAD_FRAGMENT_SHADER: &str = r#"
#version 300 es
precision highp float;

in vec2 v_uv;
in vec2 v_size;

uniform vec4 color;
uniform vec4 border_color;
uniform float border_width;
uniform float corner_radius;

out vec4 frag_color;

float rounded_box(vec2 p, vec2 b, float r) {
    return length(max(abs(p) - b + r, 0.0)) - r;
}

void main() {
    vec2 pos = v_uv * v_size;
    vec2 center = v_size * 0.5;
    vec2 p = pos - center;

    float box = rounded_box(p, center - border_width, corner_radius);
    float border = rounded_box(p, center, corner_radius);

    if (border_width > 0.0 && border <= 0.0 && box > 0.0) {
        frag_color = border_color;
    } else if (box <= 0.0) {
        frag_color = color;
    } else {
        discard;
    }
}
"#;
