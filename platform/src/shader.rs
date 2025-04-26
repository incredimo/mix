#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ShaderId(pub usize);

#[derive(Clone, Debug)]
pub struct Shader {
    pub shader_id: ShaderId,
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub uniforms: Vec<ShaderUniform>,
}

#[derive(Clone, Debug)]
pub struct ShaderUniform {
    pub name: String,
    pub uniform_type: ShaderUniformType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShaderUniformType {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat4,
    Texture2D,
}

impl Shader {
    pub fn new(shader_id: ShaderId) -> Self {
        Self {
            shader_id,
            vertex_shader: String::new(),
            fragment_shader: String::new(),
            uniforms: Vec::new(),
        }
    }
    
    pub fn set_vertex_shader(&mut self, vertex_shader: &str) {
        self.vertex_shader = vertex_shader.to_string();
    }
    
    pub fn set_fragment_shader(&mut self, fragment_shader: &str) {
        self.fragment_shader = fragment_shader.to_string();
    }
    
    pub fn add_uniform(&mut self, name: &str, uniform_type: ShaderUniformType) {
        self.uniforms.push(ShaderUniform {
            name: name.to_string(),
            uniform_type,
        });
    }
}
