use crate::platform::Cx;
use crate::platform::event::{Event, MouseButton};
use crate::platform::area::Area;
use crate::draw::Cx2d;
use crate::draw::draw_list_2d::DrawList2d;
use crate::draw::quad::DrawQuad;
use crate::draw::text::DrawText;
use crate::draw::color::Color;
use crate::draw::math::Vec2;
use crate::widgets::widget::{Widget, DrawStep};
use crate::widgets::theme::Theme;

pub enum ButtonState {
    Normal,
    Hover,
    Pressed,
    Disabled,
}

pub struct Button {
    pub text: String,
    pub state: ButtonState,
    pub draw_list: DrawList2d,
    pub area: Area,
    pub draw_bg: DrawQuad,
    pub draw_text: DrawText,
    pub padding: Vec2,
    pub on_click: Option<Box<dyn FnMut(&mut Cx)>>,
}

impl Button {
    pub fn new(cx: &mut Cx, text: &str) -> Self {
        let theme = Theme::default();

        Self {
            text: text.to_string(),
            state: ButtonState::Normal,
            draw_list: DrawList2d::new(cx),
            area: cx.create_area(),
            draw_bg: DrawQuad::new()
                .with_color(theme.primary_color)
                .with_corner_radius(theme.border_radius_medium),
            draw_text: DrawText::new()
                .with_text(text)
                .with_style(theme.button_text_style),
            padding: Vec2::new(theme.spacing_medium, theme.spacing_medium),
            on_click: None,
        }
    }

    pub fn with_on_click<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&mut Cx) + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.draw_bg = self.draw_bg.with_color(color);
        self
    }

    pub fn with_text_color(mut self, color: Color) -> Self {
        self.draw_text = self.draw_text.with_color(color);
        self
    }

    pub fn with_padding(mut self, padding: Vec2) -> Self {
        self.padding = padding;
        self
    }
}

impl Widget for Button {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::MouseDown { button: MouseButton::Left, x, y, .. } => {
                if let Some(area_data) = cx.areas.get(&self.area) {
                    let (ax, ay, aw, ah) = area_data.rect;
                    if *x >= ax && *x <= ax + aw && *y >= ay && *y <= ay + ah {
                        self.state = ButtonState::Pressed;
                    }
                }
            },
            Event::MouseUp { button: MouseButton::Left, x, y, .. } => {
                if let Some(area_data) = cx.areas.get(&self.area) {
                    let (ax, ay, aw, ah) = area_data.rect;
                    if *x >= ax && *x <= ax + aw && *y >= ay && *y <= ay + ah {
                        if let ButtonState::Pressed = self.state {
                            if let Some(on_click) = &mut self.on_click {
                                on_click(cx);
                            }
                        }
                        self.state = ButtonState::Hover;
                    } else {
                        self.state = ButtonState::Normal;
                    }
                }
            },
            Event::MouseMove { x, y, .. } => {
                if let Some(area_data) = cx.areas.get(&self.area) {
                    let (ax, ay, aw, ah) = area_data.rect;
                    if *x >= ax && *x <= ax + aw && *y >= ay && *y <= ay + ah {
                        if let ButtonState::Normal = self.state {
                            self.state = ButtonState::Hover;
                        }
                    } else {
                        if let ButtonState::Hover = self.state {
                            self.state = ButtonState::Normal;
                        }
                    }
                }
            },
            _ => {}
        }
    }

    fn draw(&mut self, cx: &mut Cx2d) -> DrawStep {
        self.draw_list.begin(cx);

        // Update button appearance based on state
        match self.state {
            ButtonState::Normal => {
                // Use default colors
            },
            ButtonState::Hover => {
                // Lighten the background color
                let mut color = self.draw_bg.color;
                color.r = (color.r + 0.1).min(1.0);
                color.g = (color.g + 0.1).min(1.0);
                color.b = (color.b + 0.1).min(1.0);
                self.draw_bg.color = color;
            },
            ButtonState::Pressed => {
                // Darken the background color
                let mut color = self.draw_bg.color;
                color.r = (color.r - 0.1).max(0.0);
                color.g = (color.g - 0.1).max(0.0);
                color.b = (color.b - 0.1).max(0.0);
                self.draw_bg.color = color;
            },
            ButtonState::Disabled => {
                // Gray out the button
                self.draw_bg.color = Color::new(0.5, 0.5, 0.5, 0.5);
                self.draw_text.style.color = Color::new(0.7, 0.7, 0.7, 0.7);
            },
        }

        // Calculate button size based on text
        let text_size = Vec2::new(100.0, 20.0); // Placeholder for actual text measurement
        let button_size = Vec2::new(
            text_size.x + self.padding.x * 2.0,
            text_size.y + self.padding.y * 2.0,
        );

        // Add the button to the current turtle
        if let Some(rect) = cx.add_turtle_item(button_size) {
            // Draw the button background
            self.draw_bg.draw(cx, self.draw_list.id(), &rect);

            // Draw the button text
            let text_rect = rect.clone();
            self.draw_text.draw(cx, self.draw_list.id(), &text_rect, &Default::default());

            // Update the area for event handling
            cx.set_area_rect(self.area, rect.x(), rect.y(), rect.width(), rect.height());
            cx.set_area_draw_list(self.area, self.draw_list.id());
        }

        self.draw_list.end(cx);

        DrawStep::done()
    }
}



