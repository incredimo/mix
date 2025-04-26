#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GeometryId(pub usize);

#[derive(Clone, Debug)]
pub struct Geometry {
    pub geometry_id: GeometryId,
    pub vertices: Vec<u8>,
    pub indices: Vec<u16>,
    pub vertex_attributes: Vec<VertexAttribute>,
}

#[derive(Clone, Debug)]
pub struct VertexAttribute {
    pub name: String,
    pub offset: usize,
    pub format: VertexFormat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VertexFormat {
    Float1,
    Float2,
    Float3,
    Float4,
    Byte4,
    UByte4,
    Short2,
    UShort2,
    Short4,
    UShort4,
}

impl VertexFormat {
    pub fn size(&self) -> usize {
        match self {
            VertexFormat::Float1 => 4,
            VertexFormat::Float2 => 8,
            VertexFormat::Float3 => 12,
            VertexFormat::Float4 => 16,
            VertexFormat::Byte4 => 4,
            VertexFormat::UByte4 => 4,
            VertexFormat::Short2 => 4,
            VertexFormat::UShort2 => 4,
            VertexFormat::Short4 => 8,
            VertexFormat::UShort4 => 8,
        }
    }
}

impl Geometry {
    pub fn new(geometry_id: GeometryId) -> Self {
        Self {
            geometry_id,
            vertices: Vec::new(),
            indices: Vec::new(),
            vertex_attributes: Vec::new(),
        }
    }
    
    pub fn add_vertex_attribute(&mut self, name: &str, offset: usize, format: VertexFormat) {
        self.vertex_attributes.push(VertexAttribute {
            name: name.to_string(),
            offset,
            format,
        });
    }
    
    pub fn set_vertices(&mut self, vertices: Vec<u8>) {
        self.vertices = vertices;
    }
    
    pub fn set_indices(&mut self, indices: Vec<u16>) {
        self.indices = indices;
    }
}



