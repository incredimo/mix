// Mix - A Rust UI framework
// Single crate that includes platform, draw, and widgets modules

// Re-export platform modules
pub mod platform;
pub use platform::{Cx, Event, EventHandler, Area, PassId, Pass, PassClearColor, PassClearDepth,
                   DrawListId, DrawList, Texture, TextureFormat, Geometry, GeometryId,
                   math::{Vec2, Vec3, Vec4, Mat4}};

// Re-export draw modules
pub mod draw;
pub use draw::{Cx2d, DrawList2d, color::Color, layout::{Layout, LayoutAlign, LayoutDirection},
               text::{TextAlign, Font, DrawText}, turtle::{Turtle, Walk}, rect::Rect, quad::DrawQuad};

// Re-export widgets modules
pub mod widgets;
pub use widgets::widget::{Widget, DrawStep, WidgetRef};
pub use widgets::view::View;
pub use widgets::button::Button;
pub use widgets::label::Label;
pub use widgets::window::Window;
pub use widgets::app::{AppMain, App};
pub use widgets::theme::Theme;

// Re-export the app_main macro
#[macro_export]
macro_rules! app_main {
    ($app_type:ty) => {
        fn main() {
            let app = <$app_type>::new();
            let app_runner = $crate::widgets::app::App::new(app);
            app_runner.run();
        }
    };
}

// Re-export modules to avoid ambiguity
pub use platform::window;
pub use platform::shader;
pub use platform::event;
pub use platform::area;
pub use platform::pass;
pub use platform::draw_list;
pub use platform::texture;
pub use platform::geometry;



