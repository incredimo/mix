use crate::platform::Cx;
use crate::platform::event::Event;
use crate::platform::window::WindowId;
use crate::platform::area::Area;
use crate::platform::pass::{PassId, PassClearColor};
use crate::draw::Cx2d;
use crate::draw::draw_list_2d::DrawList2d;
use crate::draw::quad::DrawQuad;
use crate::draw::math::Vec2;
use crate::widgets::widget::{Widget, DrawStep, WidgetRef};
use crate::widgets::view::View;
use crate::widgets::theme::Theme;

pub struct Window {
    pub title: String,
    pub window_id: WindowId,
    pub pass_id: PassId,
    pub draw_list: DrawList2d,
    pub area: Area,
    pub draw_bg: DrawQuad,
    pub content: WidgetRef,
    pub theme: Theme,
    pub size: Vec2,
}

impl Window {
    pub fn new(cx: &mut Cx, title: &str) -> Self {
        let theme = Theme::default();

        // Create window
        let window_id = cx.create_window(title, 800, 600);

        // Create pass
        let pass_id = cx.create_pass();
        if let Some(pass) = cx.passes.get_mut(&pass_id) {
            pass.set_window_parent(window_id);
            pass.set_clear_color(PassClearColor::new(
                theme.background_color.r,
                theme.background_color.g,
                theme.background_color.b,
                1.0,
            ));
        }

        // Create main draw list
        let draw_list = DrawList2d::new(cx);

        // Create content view
        let content = View::new(cx);

        Self {
            title: title.to_string(),
            window_id,
            pass_id,
            draw_list,
            area: cx.create_area(),
            draw_bg: DrawQuad::new()
                .with_color(theme.background_color),
            content: WidgetRef::new(content),
            theme,
            size: Vec2::new(800.0, 600.0),
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.size = Vec2::new(width, height);
        self
    }

    pub fn with_theme(mut self, theme: Theme) -> Self {
        let bg_color = theme.background_color;
        self.theme = theme;
        self.draw_bg = self.draw_bg.with_color(bg_color);
        self
    }

    pub fn with_content<W: Widget + 'static>(mut self, content: W) -> Self {
        self.content = WidgetRef::new(content);
        self
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
        // Update window title in platform
    }
}

impl Widget for Window {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // Handle window-specific events
        match event {
            Event::WindowResize { window_id, width, height, .. } => {
                if *window_id == self.window_id {
                    self.size = Vec2::new(*width, *height);
                }
            },
            _ => {}
        }

        // Pass events to content
        self.content.handle_event(cx, event);
    }

    fn draw(&mut self, cx: &mut Cx2d) -> DrawStep {
        // Begin drawing
        self.draw_list.begin(cx);

        // Draw background
        let rect = crate::draw::rect::Rect::new(0.0, 0.0, self.size.x, self.size.y);
        self.draw_bg.draw(cx, self.draw_list.id(), &rect);

        // Create a turtle for the content
        cx.begin_sized_turtle(self.size, crate::draw::layout::Layout::vertical());

        // Draw content
        let draw_step = self.content.draw(cx);

        // End turtle
        cx.end_turtle();

        // End drawing
        self.draw_list.end(cx);

        // Set the main draw list for the pass
        if let Some(pass) = cx.passes.get_mut(&self.pass_id) {
            pass.set_main_draw_list(self.draw_list.id());
        }

        draw_step
    }
}



