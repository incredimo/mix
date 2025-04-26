use crate::platform::math::Vec2;
use crate::platform::geometry::{Geometry, GeometryId, VertexFormat};
use crate::platform::shader::{Shader, ShaderId};
use crate::platform::texture::Texture;
use crate::platform::draw_list::{DrawItem, DrawUniform};
use crate::platform::Cx;
use crate::draw::color::Color;
use crate::draw::rect::Rect;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Debug)]
pub struct Font {
    pub name: String,
    pub data: Vec<u8>,
    pub font_info: Option<ttf_parser::Face<'static>>,
    pub texture: Option<Texture>,
    pub glyphs: HashMap<char, GlyphInfo>,
}

impl Default for Font {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            data: Vec::new(),
            font_info: None,
            texture: None,
            glyphs: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GlyphInfo {
    pub code_point: char,
    pub advance: f32,
    pub bearing: Vec2,
    pub size: Vec2,
    pub uv_rect: Rect,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(Clone, Debug)]
pub struct TextStyle {
    pub font_size: f32,
    pub font_name: String,
    pub color: Color,
    pub align: TextAlign,
    pub line_height: f32,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            font_size: 16.0,
            font_name: "default".to_string(),
            color: Color::black(),
            align: TextAlign::Left,
            line_height: 1.2,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DrawText {
    pub text: String,
    pub style: TextStyle,
    pub shader_id: Option<ShaderId>,
    pub geometry_id: Option<GeometryId>,
}

impl Default for DrawText {
    fn default() -> Self {
        Self {
            text: String::new(),
            style: TextStyle::default(),
            shader_id: None,
            geometry_id: None,
        }
    }
}

impl DrawText {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn with_style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.style.font_size = font_size;
        self
    }

    pub fn with_font_name(mut self, font_name: &str) -> Self {
        self.style.font_name = font_name.to_string();
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.style.color = color;
        self
    }

    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.style.align = align;
        self
    }

    pub fn init(&mut self, cx: &mut Cx) {
        if self.shader_id.is_none() {
            let shader_id = cx.create_shader();
            let mut shader = Shader::new(shader_id);

            shader.set_vertex_shader(TEXT_VERTEX_SHADER);
            shader.set_fragment_shader(TEXT_FRAGMENT_SHADER);

            shader.add_uniform("color", crate::platform::shader::ShaderUniformType::Vec4);
            shader.add_uniform("font_size", crate::platform::shader::ShaderUniformType::Float);

            cx.shaders.insert(shader_id, shader);
            self.shader_id = Some(shader_id);
        }

        if self.geometry_id.is_none() {
            let geometry_id = cx.create_geometry();
            let mut geometry = Geometry::new(geometry_id);

            // Define quad vertices for text rendering
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

    pub fn measure_text(&self, font: &Font) -> Vec2 {
        let mut width: f32 = 0.0;
        let mut height = self.style.font_size * self.style.line_height;
        let scale_factor = self.style.font_size / 32.0; // Assuming font metrics are based on 32px

        let mut line_width: f32 = 0.0;

        for grapheme in self.text.graphemes(true) {
            if grapheme == "\n" {
                width = width.max(line_width);
                line_width = 0.0;
                height += self.style.font_size * self.style.line_height;
                continue;
            }

            let c = grapheme.chars().next().unwrap_or(' ');
            if let Some(glyph) = font.glyphs.get(&c) {
                line_width += glyph.advance * scale_factor;
            } else {
                // Use a default advance for unknown characters
                line_width += self.style.font_size * 0.5;
            }
        }

        width = width.max(line_width);

        Vec2::new(width, height)
    }

    pub fn draw(&mut self, cx: &mut Cx, draw_list_id: crate::platform::draw_list::DrawListId, rect: &Rect, font: &Font) {
        if self.shader_id.is_none() || self.geometry_id.is_none() {
            self.init(cx);
        }

        if font.texture.is_none() {
            return;
        }

        let scale_factor = self.style.font_size / 32.0; // Assuming font metrics are based on 32px
        let mut x = rect.pos.x;
        let mut y = rect.pos.y;

        // Adjust starting position based on text alignment
        let text_size = self.measure_text(font);
        match self.style.align {
            TextAlign::Left => {},
            TextAlign::Center => {
                x += (rect.width() - text_size.x) * 0.5;
            },
            TextAlign::Right => {
                x += rect.width() - text_size.x;
            },
        }

        let mut _line_width: f32 = 0.0;
        let line_start_x = x;

        for grapheme in self.text.graphemes(true) {
            if grapheme == "\n" {
                // Move to the next line
                y += self.style.font_size * self.style.line_height;

                // Reset x position based on alignment
                match self.style.align {
                    TextAlign::Left => {
                        x = line_start_x;
                    },
                    TextAlign::Center => {
                        x = line_start_x;
                    },
                    TextAlign::Right => {
                        x = line_start_x;
                    },
                }

                // Reset line width
                _line_width = 0.0;
                continue;
            }

            let c = grapheme.chars().next().unwrap_or(' ');
            if let Some(glyph) = font.glyphs.get(&c) {
                // Calculate glyph position and size
                let _glyph_pos = Vec2::new(
                    x + glyph.bearing.x * scale_factor,
                    y - glyph.bearing.y * scale_factor
                );
                let _glyph_size = Vec2::new(
                    glyph.size.x * scale_factor,
                    glyph.size.y * scale_factor
                );

                let draw_item = DrawItem {
                    shader_id: self.shader_id.unwrap(),
                    geometry_id: self.geometry_id.unwrap(),
                    uniforms: vec![
                        DrawUniform::Vec4(self.style.color.to_array()),
                        DrawUniform::Float(self.style.font_size),
                    ],
                    textures: vec![font.texture.clone().unwrap()],
                    instance_count: 1,
                };

                if let Some(draw_list) = cx.draw_lists.get_mut(&draw_list_id) {
                    draw_list.add_draw_item(draw_item);
                }

                x += glyph.advance * scale_factor;
                _line_width += glyph.advance * scale_factor;
            } else {
                // Skip unknown characters
                x += self.style.font_size * 0.5;
                _line_width += self.style.font_size * 0.5;
            }
        }
    }
}

const TEXT_VERTEX_SHADER: &str = r#"
#version 300 es
precision highp float;

in vec2 position;
in vec2 uv;

uniform float font_size;
uniform mat4 view_transform;

out vec2 v_uv;

void main() {
    v_uv = uv;
    gl_Position = view_transform * vec4(position * font_size, 0.0, 1.0);
}
"#;

const TEXT_FRAGMENT_SHADER: &str = r#"
#version 300 es
precision highp float;

in vec2 v_uv;

uniform vec4 color;
uniform sampler2D font_texture;

out vec4 frag_color;

void main() {
    float alpha = texture(font_texture, v_uv).r;
    frag_color = vec4(color.rgb, color.a * alpha);
}
"#;



