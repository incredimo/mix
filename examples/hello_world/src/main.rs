use mix::Cx;
use mix::event::Event;
use mix::platform::math::Vec2;
use mix::draw::color::Color;
use mix::draw::layout::{Layout, LayoutAlign};
use mix::Cx2d;
use mix::*;

struct HelloWorldApp {
    window: Window,
    counter: i32,
}

impl HelloWorldApp {
    fn new() -> Self {
        let mut cx = Cx::new();

        // Create a view with a vertical layout
        let mut content = View::new(&mut cx)
            .with_layout(
                Layout::vertical()
                    .with_align_items(LayoutAlign::Center)
                    .with_justify_content(LayoutAlign::Center)
                    .with_spacing(40.0) // Increase spacing for better visibility
            );

        // Add a label
        content.add_child(
            Label::new(&mut cx, "Hello, mix!")
                .with_font_size(32.0) // Larger font
                .with_color(Color::from_hex(0x2196F3))
        );

        // Add a counter label
        content.add_child(
            Label::new(&mut cx, "Counter: 0")
                .with_font_size(24.0) // Larger font
        );

        // Add increment button
        content.add_child(
            Button::new(&mut cx, "Increment")
                .with_padding(Vec2::new(40.0, 20.0)) // Larger padding
                .with_color(Color::from_hex(0x4CAF50))
        );

        // Create window with content
        let window = Window::new(&mut cx, "Hello mix")
            .with_size(800.0, 600.0) // Larger window
            .with_content(content);

        Self {
            window,
            counter: 0,
        }
    }
}

impl AppMain for HelloWorldApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // Handle events
        self.window.handle_event(cx, event);

        // Handle button click
        if let Event::MouseUp { .. } = event {
            // In a real implementation, we would check if the button was clicked
            // and update the counter label
            // For simplicity, we're just incrementing on any mouse up event
            self.counter += 1;

            // Update the counter label
            // In a real implementation, we would find the label and update its text
        }

        // Handle draw event
        if let Event::Draw = event {
            let mut cx2d = Cx2d::new(cx);
            self.window.draw(&mut cx2d);
        }
    }
}

app_main!(HelloWorldApp);
