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

pub use crate::cx::Cx;
pub use crate::event::{Event, EventHandler};
pub use crate::window::{WindowId, WindowHandle, WindowGeom};
pub use crate::area::Area;
pub use crate::pass::{PassId, Pass, PassClearColor, PassClearDepth};
pub use crate::draw_list::{DrawListId, DrawList};
pub use crate::texture::{Texture, TextureFormat};
pub use crate::geometry::{Geometry, GeometryId};
pub use crate::shader::{Shader, ShaderId};
pub use crate::math::{Vec2, Vec3, Vec4, Mat4};
