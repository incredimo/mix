# Mix 🎨

A lightweight, ergonomic HTML templating library for Rust that makes building HTML structures feel natural and Rusty.

## Features ✨

- Declarative HTML-like syntax using macros
- Type-safe attribute handling
- Composable and reusable components
- Zero external dependencies
- Simple and intuitive API
- Compile-time validation

## Quick Start 🚀

Add Mix to your `Cargo.toml`:

```toml
[dependencies]
mix = "0.1.0"
```

Create your first template:

```rust
use mix::html;

fn main() {
    let page = html! {
        html {
            head {
                title { "Welcome to Mix!" }
            }
            body (class = "container") {
                h1 { "Hello, World!" }
                p { "Building HTML in Rust has never been easier." }
            }
        }
    };
    
    println!("{}", page);
}
```

## Usage Examples 🎯

### Basic Elements

Create simple HTML elements with attributes:

```rust
let button = html! {
    button (class = "btn", id = "submit") {
        "Click me!"
    }
};
```

### Nested Structures

Build complex nested structures with ease:

```rust
let card = html! {
    div (class = "card") {
        div (class = "card-header") {
            h2 { "Featured" }
        }
        div (class = "card-body") {
            p { "This is some sample content." }
            a (href = "#", class = "btn btn-primary") {
                "Learn more"
            }
        }
    }
};
```

### Custom Components

Create reusable components using the `Html` trait:

```rust
struct NavLink {
    text: String,
    href: String,
    active: bool,
}

impl Html for NavLink {
    fn render(&self) -> String {
        html! {
            a (
                href = self.href,
                class = if self.active { "nav-link active" } else { "nav-link" }
            ) {
                self.text
            }
        }
    }
}
```

## How It Works 🔧

Mix uses a combination of traits and macros to provide a seamless HTML templating experience:

1. The `Html` trait defines how elements are rendered to strings
2. The `Element` struct represents HTML elements with their attributes and children
3. The `html!` macro provides the familiar HTML-like syntax
4. String literals are automatically escaped and handled appropriately

## Best Practices 📚

1. Keep your components small and focused
2. Use semantic HTML tags
3. Leverage Rust's type system for safe templates
4. Break down complex structures into smaller, reusable components

## Contributing 🤝

We welcome contributions! Here's how you can help:

1. Fork the repository
2. Create a new branch for your feature
3. Add tests for new functionality
4. Submit a pull request

Please make sure to update tests as appropriate and follow our code style.

## License 📄

MIT License 

## Credits 👏

Created with ❤️ by the Mix team. Special thanks to all our contributors!