// OS-specific implementations
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_arch = "wasm32")]
pub mod web;

// Re-export the OS-specific implementation
#[cfg(target_os = "windows")]
pub use self::windows::*;

#[cfg(target_os = "macos")]
pub use self::macos::*;

#[cfg(target_os = "linux")]
pub use self::linux::*;

#[cfg(target_arch = "wasm32")]
pub use self::web::*;

// Define a common OS trait
pub trait OsBackend {
    fn init(&mut self);
    fn create_window(&mut self, title: &str, width: u32, height: u32) -> crate::window::WindowId;
    fn process_events(&mut self) -> Vec<crate::event::Event>;
    fn render(&mut self);
    fn shutdown(&mut self);
}



