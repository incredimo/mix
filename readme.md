# mix UI Library

mix is a high-performance UI library for Rust, inspired by Makepad's architecture but with a focus on simplicity and performance. mix provides a clean, modular API for building cross-platform user interfaces without the complexity of live rendering and compiler features.

## Features

- **High Performance**: Built with performance in mind, using efficient rendering techniques
- **Cross-Platform**: Supports Windows, macOS, Linux, and Web (via WebAssembly)
- **Modular Architecture**: Clean separation between platform, drawing, and widget layers
- **Simple API**: Easy-to-use API for building UIs with minimal boilerplate
- **Customizable Theming**: Flexible theming system for consistent UI appearance

## Architecture

mix is organized into three main crates:

1. **mix-platform**: Core platform abstraction layer that handles windowing, events, and rendering
2. **mix-draw**: Drawing primitives and utilities for 2D rendering
3. **mix-widgets**: UI widgets and layout system

## Getting Started

Add mix to your Cargo.toml:

```toml
[dependencies]
mix-platform = "0.1.0"
mix-draw = "0.1.0"
mix-widgets = "0.1.0"
```

Create a simple application:

```rust
use mix_platform::Cx;
use mix_platform::event::Event;
use mix_widgets::*;

struct MyApp {
    window: Window,
}

impl MyApp {
    fn new() -> Self {
        let mut cx = Cx::new();
        
        // Create a view with a label
        let mut content = View::new(&mut cx);
        content.add_child(Label::new(&mut cx, "Hello, mix!"));
        
        // Create window with content
        let window = Window::new(&mut cx, "My App")
            .with_content(content);
        
        Self { window }
    }
}

impl AppMain for MyApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.window.handle_event(cx, event);
        
        if let Event::Draw = event {
            let mut cx2d = Cx2d::new(cx);
            self.window.draw(&mut cx2d);
        }
    }
}

app_main!(MyApp);
```

## Examples

Check out the examples directory for more examples:

- **hello_world**: A simple Hello World application
- **counter**: A counter application demonstrating state management
- **todo_list**: A todo list application demonstrating more complex UI

## License

MIT or Apache-2.0, at your option.
