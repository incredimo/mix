use std::collections::HashMap;
use crate::platform::window::{WindowId, WindowHandle};
use crate::platform::area::Area;
use crate::platform::pass::{PassId, Pass};
use crate::platform::draw_list::{DrawListId, DrawList};
use crate::platform::texture::{TextureId, Texture, TextureFormat};
use crate::platform::geometry::{GeometryId, Geometry};
use crate::platform::shader::{ShaderId, Shader};
use crate::platform::gpu_info::GpuInfo;
use crate::platform::debug::Debug;
use crate::platform::performance_stats::PerformanceStats;
use crate::platform::event::Event;
use crate::platform::os::OsBackend;

#[cfg(target_os = "windows")]
use crate::platform::os::windows::WindowsBackend;

pub struct Cx {
    pub windows: HashMap<WindowId, WindowHandle>,
    pub passes: HashMap<PassId, Pass>,
    pub draw_lists: HashMap<DrawListId, DrawList>,
    pub textures: HashMap<TextureId, Texture>,
    pub geometries: HashMap<GeometryId, Geometry>,
    pub shaders: HashMap<ShaderId, Shader>,
    pub areas: HashMap<Area, AreaData>,
    pub gpu_info: GpuInfo,
    pub debug: Debug,
    pub performance_stats: PerformanceStats,

    next_window_id: usize,
    next_pass_id: usize,
    next_draw_list_id: usize,
    next_texture_id: usize,
    next_geometry_id: usize,
    next_shader_id: usize,
    next_area_id: usize,

    #[cfg(target_os = "windows")]
    os_backend: WindowsBackend,
}

pub struct AreaData {
    pub rect: (f32, f32, f32, f32), // x, y, width, height
    pub draw_list_id: Option<DrawListId>,
}

impl Cx {
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
            passes: HashMap::new(),
            draw_lists: HashMap::new(),
            textures: HashMap::new(),
            geometries: HashMap::new(),
            shaders: HashMap::new(),
            areas: HashMap::new(),
            gpu_info: GpuInfo::new(),
            debug: Debug::new(),
            performance_stats: PerformanceStats::new(),

            next_window_id: 1,
            next_pass_id: 1,
            next_draw_list_id: 1,
            next_texture_id: 1,
            next_geometry_id: 1,
            next_shader_id: 1,
            next_area_id: 1,

            #[cfg(target_os = "windows")]
            os_backend: WindowsBackend::new(),
        }
    }

    pub fn init(&mut self) {
        #[cfg(target_os = "windows")]
        self.os_backend.init();
    }

    pub fn create_window(&mut self, title: &str, width: u32, height: u32) -> WindowId {
        #[cfg(target_os = "windows")]
        let window_id = self.os_backend.create_window(title, width, height);

        #[cfg(not(target_os = "windows"))]
        let window_id = WindowId(self.next_window_id);

        self.next_window_id += 1;

        let window_handle = WindowHandle::new(window_id);
        self.windows.insert(window_id, window_handle);

        window_id
    }

    pub fn create_pass(&mut self) -> PassId {
        let pass_id = PassId(self.next_pass_id);
        self.next_pass_id += 1;

        let pass = Pass::new(pass_id);
        self.passes.insert(pass_id, pass);

        pass_id
    }

    pub fn create_draw_list(&mut self) -> DrawListId {
        let draw_list_id = DrawListId(self.next_draw_list_id);
        self.next_draw_list_id += 1;

        let draw_list = DrawList::new(draw_list_id);
        self.draw_lists.insert(draw_list_id, draw_list);

        draw_list_id
    }

    pub fn create_texture(&mut self, width: usize, height: usize, format: TextureFormat) -> Texture {
        let texture_id = TextureId(self.next_texture_id);
        self.next_texture_id += 1;

        let texture = Texture::new(texture_id, width, height, format);
        self.textures.insert(texture_id, texture.clone());

        texture
    }

    pub fn create_geometry(&mut self) -> GeometryId {
        let geometry_id = GeometryId(self.next_geometry_id);
        self.next_geometry_id += 1;

        let geometry = Geometry::new(geometry_id);
        self.geometries.insert(geometry_id, geometry);

        geometry_id
    }

    pub fn create_shader(&mut self) -> ShaderId {
        let shader_id = ShaderId(self.next_shader_id);
        self.next_shader_id += 1;

        let shader = Shader::new(shader_id);
        self.shaders.insert(shader_id, shader);

        shader_id
    }

    pub fn create_area(&mut self) -> Area {
        let area = Area(self.next_area_id);
        self.next_area_id += 1;

        let area_data = AreaData {
            rect: (0.0, 0.0, 0.0, 0.0),
            draw_list_id: None,
        };

        self.areas.insert(area, area_data);

        area
    }

    pub fn set_area_rect(&mut self, area: Area, x: f32, y: f32, width: f32, height: f32) {
        if let Some(area_data) = self.areas.get_mut(&area) {
            area_data.rect = (x, y, width, height);
        }
    }

    pub fn set_area_draw_list(&mut self, area: Area, draw_list_id: DrawListId) {
        if let Some(area_data) = self.areas.get_mut(&area) {
            area_data.draw_list_id = Some(draw_list_id);
        }
    }

    pub fn process_events(&mut self) -> Vec<Event> {
        #[cfg(target_os = "windows")]
        return self.os_backend.process_events();

        #[cfg(not(target_os = "windows"))]
        return Vec::new();
    }

    pub fn render(&mut self) {
        self.performance_stats.update();

        #[cfg(target_os = "windows")]
        self.os_backend.render();
    }

    pub fn shutdown(&mut self) {
        #[cfg(target_os = "windows")]
        self.os_backend.shutdown();
    }

    pub fn run<F>(&mut self, mut event_handler: F)
    where
        F: FnMut(&mut Cx, Event),
    {
        self.init();

        // Send init event
        event_handler(self, Event::Init);

        // Initial draw
        event_handler(self, Event::Draw);
        self.render();

        // Main event loop
        loop {
            // Process events
            let events = self.process_events();
            let mut needs_redraw = false;

            for event in events {
                match event {
                    Event::Shutdown => {
                        event_handler(self, Event::Shutdown);
                        self.shutdown();
                        return;
                    },
                    Event::WindowResize { .. } => {
                        needs_redraw = true;
                        event_handler(self, event);
                    },
                    _ => {
                        event_handler(self, event);
                    }
                }
            }

            // Render if needed
            if needs_redraw {
                event_handler(self, Event::Draw);
                self.render();
            }

            // Always render for now to keep the UI responsive
            // In a real implementation, we would only render when needed
            event_handler(self, Event::Draw);
            self.render();

            // Sleep a bit to avoid using 100% CPU
            std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
        }
    }
}



