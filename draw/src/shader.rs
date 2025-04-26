use mix_platform::shader::{Shader, ShaderId};
use mix_platform::Cx;

pub struct ShaderBuilder {
    vertex_shader: String,
    fragment_shader: String,
    uniforms: Vec<(String, mix_platform::shader::ShaderUniformType)>,
}

impl ShaderBuilder {
    pub fn new() -> Self {
        Self {
            vertex_shader: String::new(),
            fragment_shader: String::new(),
            uniforms: Vec::new(),
        }
    }
    
    pub fn vertex_shader(mut self, source: &str) -> Self {
        self.vertex_shader = source.to_string();
        self
    }
    
    pub fn fragment_shader(mut self, source: &str) -> Self {
        self.fragment_shader = source.to_string();
        self
    }
    
    pub fn uniform(mut self, name: &str, uniform_type: mix_platform::shader::ShaderUniformType) -> Self {
        self.uniforms.push((name.to_string(), uniform_type));
        self
    }
    
    pub fn build(self, cx: &mut Cx) -> ShaderId {
        let shader_id = cx.create_shader();
        let mut shader = Shader::new(shader_id);
        
        shader.set_vertex_shader(&self.vertex_shader);
        shader.set_fragment_shader(&self.fragment_shader);
        
        for (name, uniform_type) in self.uniforms {
            shader.add_uniform(&name, uniform_type);
        }
        
        cx.shaders.insert(shader_id, shader);
        
        shader_id
    }
}
