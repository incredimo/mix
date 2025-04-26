#[derive(Clone, Debug)]
pub struct GpuInfo {
    pub name: String,
    pub vendor: String,
    pub version: String,
    pub max_texture_size: usize,
}

impl GpuInfo {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            vendor: String::new(),
            version: String::new(),
            max_texture_size: 4096,
        }
    }
}
