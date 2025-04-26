pub mod cx;
pub mod event;
pub mod window;
pub mod area;
pub mod pass;
pub mod draw_list;
pub mod texture;
pub mod geometry;
pub mod shader;
pub mod gpu_info;
pub mod os;
pub mod math;

mod debug;
mod performance_stats;

pub use crate::platform::cx::Cx;
pub use crate::platform::event::{Event, EventHandler};
pub use crate::platform::window::{WindowId, WindowHandle, WindowGeom};
pub use crate::platform::area::Area;
pub use crate::platform::pass::{PassId, Pass, PassClearColor, PassClearDepth};
pub use crate::platform::draw_list::{DrawListId, DrawList};
pub use crate::platform::texture::{Texture, TextureFormat};
pub use crate::platform::geometry::{Geometry, GeometryId};
pub use crate::platform::shader::{Shader, ShaderId};
pub use crate::platform::math::{Vec2, Vec3, Vec4, Mat4};



