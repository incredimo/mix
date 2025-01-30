# 🚀 Mix HTML Builder

A zero-dependency, lightweight HTML builder for Rust with an elegant macro syntax. Build HTML structures with the simplicity of JSX and the power of Rust - in just ~200 lines of code!

## ✨ Features

- **Zero Dependencies**: no serde no thiserror how come? 
- **Minimal Core**: The entire implementation is less than 200 lines of code
- **Type-Safe**: Leverage Rust's type system
- **JSX-like Syntax**: Familiar and intuitive
- **Component-Based**: Create reusable HTML components

## 🎯 Quick Start

```rust
let page = html! {
    div (class = "container") {
        h1 { "Welcome!" }
        p { "This is a " span (class = "highlight") { "simple" } " example." }
    }
};
```

## 🔧 Usage

### Basic Elements

```rust
html! {
    div (class = "card", id = "main") {
        h2 { "Hello, World!" }
        p { "This is a paragraph." }
    }
}
```

### Custom Components

```rust
struct Card {
    title: String,
    content: String,
}

impl Html for Card {
    fn render(&self) -> String {
        html! {
            div (class = "card") {
                h3 { (self.title) }
                p { (self.content) }
            }
        }
    }
}

// Use it in your HTML:
let card = Card {
    title: "My Card".into(),
    content: "Some content".into(),
};

html! {
    div {
        (card)
    }
}
```

### Nested Structures

```rust
html! {
    nav (class = "navbar") {
        ul {
            li { a (href = "/") { "Home" } }
            li { a (href = "/about") { "About" } }
            li { a (href = "/contact") { "Contact" } }
        }
    }
}
```

## 🛠 How It Works

The entire implementation is built around just three core components:

1. A simple `Html` trait:
```rust
pub trait Html {
    fn render(&self) -> String;
}
```

2. An `Element` struct for building HTML elements:
```rust
pub struct Element {
    tag: String,
    attrs: Vec<(String, String)>,
    children: Vec<Box<dyn Html>>,
}
```

3. A powerful `html!` macro that makes it all work together seamlessly

## 🎨 Why Use This?

- **Simplicity**: The implementation is so minimal you can understand the entire codebase in minutes
- **No Dependencies**: Keep your project lean
- **Flexible**: Build anything from simple components to complex layouts
- **Type-Safe**: Catch errors at compile time, not runtime
- **Extensible**: Easy to customize and extend

## 📝 License

MIT License

## 🤝 Contributing

Contributions are welcome! Feel free to open issues and pull requests.