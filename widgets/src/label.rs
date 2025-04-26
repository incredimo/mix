use mix_platform::Cx;
use mix_platform::event::Event;
use mix_platform::area::Area;
use mix_draw::Cx2d;
use mix_draw::draw_list_2d::DrawList2d;
use mix_draw::text::{DrawText, TextStyle, TextAlign};
use mix_draw::color::Color;
use mix_draw::math::Vec2;

use crate::widget::{Widget, DrawStep};
use crate::theme::Theme;

#[derive(Clone, Debug)]
pub struct Label {
    pub text: String,
    pub draw_list: DrawList2d,
    pub area: Area,
    pub draw_text: DrawText,
    pub padding: Vec2,
}

impl Label {
    pub fn new(cx: &mut Cx, text: &str) -> Self {
        let theme = Theme::default();

        Self {
            text: text.to_string(),
            draw_list: DrawList2d::new(cx),
            area: cx.create_area(),
            draw_text: DrawText::new()
                .with_text(text)
                .with_style(theme.default_text_style),
            padding: Vec2::new(theme.spacing_small, theme.spacing_small),
        }
    }

    pub fn with_text_style(mut self, style: TextStyle) -> Self {
        self.draw_text = self.draw_text.with_style(style);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.draw_text = self.draw_text.with_color(color);
        self
    }

    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.draw_text = self.draw_text.with_font_size(font_size);
        self
    }

    pub fn with_align(mut self, align: TextAlign) -> Self {
        self.draw_text = self.draw_text.with_align(align);
        self
    }

    pub fn with_padding(mut self, padding: Vec2) -> Self {
        self.padding = padding;
        self
    }
}

impl Widget for Label {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event) {
        // Labels don't handle events
    }

    fn draw(&mut self, cx: &mut Cx2d) -> DrawStep {
        self.draw_list.begin(cx);

        // Calculate label size based on text
        let text_size = Vec2::new(100.0, 20.0); // Placeholder for actual text measurement
        let label_size = Vec2::new(
            text_size.x + self.padding.x * 2.0,
            text_size.y + self.padding.y * 2.0,
        );

        // Add the label to the current turtle
        if let Some(rect) = cx.add_turtle_item(label_size) {
            // Draw the label text
            self.draw_text.draw(cx, self.draw_list.id(), &rect, &Default::default());

            // Update the area for event handling
            cx.set_area_rect(self.area, rect.x(), rect.y(), rect.width(), rect.height());
            cx.set_area_draw_list(self.area, self.draw_list.id());
        }

        self.draw_list.end(cx);

        DrawStep::done()
    }
}
