use mix::Cx;
use mix::event::{Event, MouseButton};
use mix::platform::math::Vec2;
use mix::platform::area::Area;
use mix::Cx2d;
use mix::draw::color::Color;
use mix::draw::layout::{Layout, LayoutAlign, LayoutDirection};
use mix::draw::text::TextAlign;
use mix::*;

struct CounterApp {
    window: Window,
    counter: i32,
    counter_label: Label,
    increment_button_area: Area,
    decrement_button_area: Area,
    reset_button_area: Area,
}

impl CounterApp {
    fn new() -> Self {
        let mut cx = Cx::new();

        // Create areas for buttons to track clicks
        let increment_button_area = cx.create_area();
        let decrement_button_area = cx.create_area();
        let reset_button_area = cx.create_area();

        // Create a counter label
        let counter_label = Label::new(&mut cx, "Counter: 0")
            .with_font_size(24.0)
            .with_color(Color::from_hex(0x2196F3))
            .with_align(TextAlign::Center);

        // Create a view for the buttons with a horizontal layout
        let mut button_row = View::new(&mut cx)
            .with_layout(
                Layout::horizontal()
                    .with_align_items(LayoutAlign::Center)
                    .with_justify_content(LayoutAlign::Center)
                    .with_spacing(10.0)
            );

        // Add decrement button
        let decrement_button = Button::new(&mut cx, "Decrement")
            .with_padding(Vec2::new(15.0, 10.0))
            .with_color(Color::from_hex(0xF44336));
        button_row.add_child(decrement_button);

        // Add reset button
        let reset_button = Button::new(&mut cx, "Reset")
            .with_padding(Vec2::new(15.0, 10.0))
            .with_color(Color::from_hex(0xFF9800));
        button_row.add_child(reset_button);

        // Add increment button
        let increment_button = Button::new(&mut cx, "Increment")
            .with_padding(Vec2::new(15.0, 10.0))
            .with_color(Color::from_hex(0x4CAF50));
        button_row.add_child(increment_button);

        // Create main content view with vertical layout
        let mut content = View::new(&mut cx)
            .with_layout(
                Layout::vertical()
                    .with_align_items(LayoutAlign::Center)
                    .with_justify_content(LayoutAlign::Center)
                    .with_spacing(20.0)
            );

        // Add title
        content.add_child(
            Label::new(&mut cx, "Counter Example")
                .with_font_size(32.0)
                .with_color(Color::from_hex(0x333333))
        );

        // Add counter label
        content.add_child(counter_label.clone());

        // Add button row
        content.add_child(button_row);

        // Create window with content
        let window = Window::new(&mut cx, "mix Counter Example")
            .with_size(500.0, 400.0)
            .with_content(content);

        Self {
            window,
            counter: 0,
            counter_label,
            increment_button_area,
            decrement_button_area,
            reset_button_area,
        }
    }

    fn update_counter_label(&mut self) {
        // Update the counter label text
        self.counter_label.text = format!("Counter: {}", self.counter);
    }
}

impl AppMain for CounterApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // Handle events
        self.window.handle_event(cx, event);

        // Handle button clicks
        match event {
            Event::MouseUp { button: MouseButton::Left, x, y, .. } => {
                // Check if increment button was clicked
                if let Some(area_data) = cx.areas.get(&self.increment_button_area) {
                    let (ax, ay, aw, ah) = area_data.rect;
                    if *x >= ax && *x <= ax + aw && *y >= ay && *y <= ay + ah {
                        self.counter += 1;
                        self.update_counter_label();
                    }
                }

                // Check if decrement button was clicked
                if let Some(area_data) = cx.areas.get(&self.decrement_button_area) {
                    let (ax, ay, aw, ah) = area_data.rect;
                    if *x >= ax && *x <= ax + aw && *y >= ay && *y <= ay + ah {
                        self.counter -= 1;
                        self.update_counter_label();
                    }
                }

                // Check if reset button was clicked
                if let Some(area_data) = cx.areas.get(&self.reset_button_area) {
                    let (ax, ay, aw, ah) = area_data.rect;
                    if *x >= ax && *x <= ax + aw && *y >= ay && *y <= ay + ah {
                        self.counter = 0;
                        self.update_counter_label();
                    }
                }
            },
            _ => {}
        }

        // Handle draw event
        if let Event::Draw = event {
            let mut cx2d = Cx2d::new(cx);
            self.window.draw(&mut cx2d);
        }
    }
}

app_main!(CounterApp);
