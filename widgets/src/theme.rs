use mix_draw::color::Color;
use mix_draw::text::TextStyle;

#[derive(Clone)]
pub struct Theme {
    // Colors
    pub background_color: Color,
    pub foreground_color: Color,
    pub primary_color: Color,
    pub secondary_color: Color,
    pub accent_color: Color,
    pub error_color: Color,
    pub success_color: Color,
    pub warning_color: Color,

    // Text styles
    pub default_text_style: TextStyle,
    pub heading_text_style: TextStyle,
    pub button_text_style: TextStyle,

    // Spacing
    pub spacing_small: f32,
    pub spacing_medium: f32,
    pub spacing_large: f32,

    // Border radius
    pub border_radius_small: f32,
    pub border_radius_medium: f32,
    pub border_radius_large: f32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            // Colors
            background_color: Color::from_hex(0xFFFFFF),
            foreground_color: Color::from_hex(0x000000),
            primary_color: Color::from_hex(0x2196F3),
            secondary_color: Color::from_hex(0x4CAF50),
            accent_color: Color::from_hex(0xFF9800),
            error_color: Color::from_hex(0xF44336),
            success_color: Color::from_hex(0x4CAF50),
            warning_color: Color::from_hex(0xFFC107),

            // Text styles
            default_text_style: TextStyle {
                font_size: 16.0,
                font_name: "default".to_string(),
                color: Color::from_hex(0x000000),
                align: mix_draw::text::TextAlign::Left,
                line_height: 1.2,
            },
            heading_text_style: TextStyle {
                font_size: 24.0,
                font_name: "default".to_string(),
                color: Color::from_hex(0x000000),
                align: mix_draw::text::TextAlign::Left,
                line_height: 1.2,
            },
            button_text_style: TextStyle {
                font_size: 16.0,
                font_name: "default".to_string(),
                color: Color::from_hex(0xFFFFFF),
                align: mix_draw::text::TextAlign::Center,
                line_height: 1.2,
            },

            // Spacing
            spacing_small: 4.0,
            spacing_medium: 8.0,
            spacing_large: 16.0,

            // Border radius
            border_radius_small: 2.0,
            border_radius_medium: 4.0,
            border_radius_large: 8.0,
        }
    }
}

pub struct DarkTheme;

impl DarkTheme {
    pub fn new() -> Theme {
        Theme {
            // Colors
            background_color: Color::from_hex(0x121212),
            foreground_color: Color::from_hex(0xFFFFFF),
            primary_color: Color::from_hex(0x2196F3),
            secondary_color: Color::from_hex(0x4CAF50),
            accent_color: Color::from_hex(0xFF9800),
            error_color: Color::from_hex(0xF44336),
            success_color: Color::from_hex(0x4CAF50),
            warning_color: Color::from_hex(0xFFC107),

            // Text styles
            default_text_style: TextStyle {
                font_size: 16.0,
                font_name: "default".to_string(),
                color: Color::from_hex(0xFFFFFF),
                align: mix_draw::text::TextAlign::Left,
                line_height: 1.2,
            },
            heading_text_style: TextStyle {
                font_size: 24.0,
                font_name: "default".to_string(),
                color: Color::from_hex(0xFFFFFF),
                align: mix_draw::text::TextAlign::Left,
                line_height: 1.2,
            },
            button_text_style: TextStyle {
                font_size: 16.0,
                font_name: "default".to_string(),
                color: Color::from_hex(0xFFFFFF),
                align: mix_draw::text::TextAlign::Center,
                line_height: 1.2,
            },

            // Spacing
            spacing_small: 4.0,
            spacing_medium: 8.0,
            spacing_large: 16.0,

            // Border radius
            border_radius_small: 2.0,
            border_radius_medium: 4.0,
            border_radius_large: 8.0,
        }
    }
}
