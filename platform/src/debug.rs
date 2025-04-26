#[derive(Clone, Debug)]
pub struct Debug {
    pub enabled: bool,
}

impl Debug {
    pub fn new() -> Self {
        Self {
            enabled: false,
        }
    }
    
    pub fn log(&self, message: &str) {
        if self.enabled {
            println!("[DEBUG] {}", message);
        }
    }
    
    pub fn error(&self, message: &str) {
        if self.enabled {
            eprintln!("[ERROR] {}", message);
        }
    }
}
