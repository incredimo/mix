#[allow(warnings)]
// Only keep std::fmt if you actually need it for something else.
use std::fmt;

// --------------------------------------------------------
// 1) The core trait and Element struct
// --------------------------------------------------------

pub trait Html {
    fn render(&self) -> String;
}

pub struct Element {
    tag: String,
    attrs: Vec<(String, String)>,
    children: Vec<Box<dyn Html>>,
}

impl Element {
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            attrs: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn attr(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.attrs.push((name.into(), value.into()));
        self
    }

    pub fn add<T: Html + 'static>(mut self, child: T) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl Html for Element {
    fn render(&self) -> String {
        let attrs = self
            .attrs
            .iter()
            .map(|(k, v)| format!(" {}=\"{}\"", k, v))
            .collect::<String>();

        let inner = self
            .children
            .iter()
            .map(|c| c.render())
            .collect::<String>();

        format!("<{}{}>{}</{}>", self.tag, attrs, inner, self.tag)
    }
}

// --------------------------------------------------------
// 2) Implement Html for strings
// --------------------------------------------------------

impl Html for String {
    fn render(&self) -> String {
        self.clone()
    }
}

impl Html for &str {
    fn render(&self) -> String {
        self.to_string()
    }
}

// --------------------------------------------------------
// 3) The single macro `html!`
// --------------------------------------------------------
//
// This macro returns a String. It parses a *sequence* of
// possible “nodes” (tags or text) and concatenates them.

#[macro_export]
macro_rules! html {
    // -------------- Base case: nothing left --------------
    () => {
        String::new()
    };

    // ---- Tag with attributes + children, then more ----
    ($tag:ident ( $($attr:ident = $val:expr),+ ) { $($inside:tt)* } $($rest:tt)*) => {{
        let mut e = $crate::Element::new(stringify!($tag));
        $(
            e = e.attr(stringify!($attr), $val);
        )+
        let e = e.add($crate::html! { $($inside)* });
        format!("{}{}", e.render(), $crate::html!{ $($rest)* })
    }};

    // ---- Tag with NO attributes + children, then more ----
    ($tag:ident { $($inside:tt)* } $($rest:tt)*) => {{
        let mut e = $crate::Element::new(stringify!($tag));
        let e = e.add($crate::html!{ $($inside)* });
        format!("{}{}", e.render(), $crate::html!{ $($rest)* })
    }};

    // ----------------- Text literal + more -----------------
    ($text:literal $($rest:tt)*) => {{
        // If the user wrote raw text, just append it.
        format!("{}{}", $text, $crate::html!{ $($rest)* })
    }};
}

// --------------------------------------------------------
// 4) Tests
// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Basic Element Tests
    #[test]
    fn test_empty_element() {
        let elem = Element::new("div");
        assert_eq!(elem.render(), "<div></div>");
    }

    #[test]
    fn test_element_with_single_attribute() {
        let elem = Element::new("input").attr("type", "text");
        assert_eq!(elem.render(), "<input type=\"text\"></input>");
    }

    #[test]
    fn test_element_with_multiple_attributes() {
        let elem = Element::new("div")
            .attr("class", "container")
            .attr("id", "main")
            .attr("data-test", "true");
        assert_eq!(
            elem.render(),
            "<div class=\"container\" id=\"main\" data-test=\"true\"></div>"
        );
    }

    // Text Content Tests
    #[test]
    fn test_element_with_text() {
        let elem = Element::new("p").add("Hello, World!");
        assert_eq!(elem.render(), "<p>Hello, World!</p>");
    }

    #[test]
    fn test_mixed_text_and_elements() {
        let rendered = html! {
            div {
                "Start "
                span { "middle" }
                " end"
            }
        };
        assert_eq!(rendered, "<div>Start <span>middle</span> end</div>");
    }

    // Nesting Tests
    #[test]
    fn test_deeply_nested_elements() {
        let rendered = html! {
            div (class = "level-1") {
                div (class = "level-2") {
                    div (class = "level-3") {
                        div (class = "level-4") {
                            "Deep content"
                        }
                    }
                }
            }
        };
        assert_eq!(
            rendered,
            "<div class=\"level-1\"><div class=\"level-2\"><div class=\"level-3\">\
             <div class=\"level-4\">Deep content</div></div></div></div>"
        );
    }

    // Common HTML Patterns Tests
    #[test]
    fn test_form_elements() {
        let rendered = html! {
            form (method = "post", action = "/submit") {
                div (class = "form-group") {
                    label (for = "name") { "Name:" }
                    input (
                        type = "text",
                        id = "name",
                        name = "name",
                        required = "true"
                    ) {}
                }
                div (class = "form-group") {
                    label (for = "email") { "Email:" }
                    input (
                        type = "email",
                        id = "email",
                        name = "email",
                        required = "true"
                    ) {}
                }
                button (type = "submit") { "Submit" }
            }
        };
        assert!(rendered.contains("<form method=\"post\" action=\"/submit\">"));
        assert!(rendered.contains("<input type=\"text\" id=\"name\""));
        assert!(rendered.contains("<input type=\"email\" id=\"email\""));
        assert!(rendered.contains("<button type=\"submit\">Submit</button>"));
    }

    #[test]
    fn test_list_rendering() {
        let rendered = html! {
            ul (class = "list") {
                li { "Item 1" }
                li { "Item 2" }
                li { "Item 3" }
            }
        };
        assert_eq!(
            rendered,
            "<ul class=\"list\"><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>"
        );
    }

    // Component Tests
    #[test]
    fn test_custom_component() {
        struct Card {
            title: String,
            content: String,
        }

        impl Html for Card {
            fn render(&self) -> String {
                html! {
                    div (class = "card") {
                        div (class = "card-header") {
                            h3 { "self.title" }
                        }
                        div (class = "card-body") {
                            p { "self.content" }
                        }
                    }
                }
            }
        }

        let card = Card {
            title: "Test Card".to_string(),
            content: "This is a test card content.".to_string(),
        };

        let rendered = card.render();
        assert!(rendered.contains("<div class=\"card\">"));
        assert!(rendered.contains("<h3>Test Card</h3>"));
        assert!(rendered.contains("This is a test card content."));
    }

    // Edge Cases Tests
    #[test]
    fn test_empty_attributes() {
        let elem = Element::new("div").attr("data-empty", "");
        assert_eq!(elem.render(), "<div data-empty=\"\"></div>");
    }

    #[test]
    fn test_multiple_text_nodes() {
        let rendered = html! {
            p {
                "First "
                "Second "
                "Third"
            }
        };
        assert_eq!(rendered, "<p>First Second Third</p>");
    }

    #[test]
    fn test_complex_webpage() {
        let rendered = html! {
            html {
                head {
                    title { "Complex Test Page" }
                    meta (charset = "utf-8") {}
                    link (rel = "stylesheet", href = "style.css") {}
                }
                body {
                    header (class = "main-header") {
                        nav {
                            ul {
                                li { a (href = "/") { "Home" } }
                                li { a (href = "/about") { "About" } }
                                li { a (href = "/contact") { "Contact" } }
                            }
                        }
                    }
                    main (class = "content") {
                        article {
                            h1 { "Welcome" }
                            p { "This is a complex test page." }
                            section (class = "features") {
                                div (class = "feature") {
                                    h3 { "Feature 1" }
                                    p { "Description 1" }
                                }
                                div (class = "feature") {
                                    h3 { "Feature 2" }
                                    p { "Description 2" }
                                }
                            }
                        }
                    }
                    footer {
                        p { "© 2025 Mix. All rights reserved." }
                    }
                }
            }
        };

        // Verify key elements are present
        assert!(rendered.contains("<html>"));
        assert!(rendered.contains("<head>"));
        assert!(rendered.contains("<title>Complex Test Page</title>"));
        assert!(rendered.contains("<meta charset=\"utf-8\">"));
        assert!(rendered.contains("<nav>"));
        assert!(rendered.contains("<main class=\"content\">"));
        assert!(rendered.contains("<footer>"));
    }

    // Real-world Use Case Tests
    #[test]
    fn test_table_generation() {
        let rendered = html! {
            table (class = "data-table") {
                thead {
                    tr {
                        th { "ID" }
                        th { "Name" }
                        th { "Email" }
                        th { "Actions" }
                    }
                }
                tbody {
                    tr {
                        td { "1" }
                        td { "John Doe" }
                        td { "john@example.com" }
                        td {
                            button (class = "btn btn-edit") { "Edit" }
                            button (class = "btn btn-delete") { "Delete" }
                        }
                    }
                }
            }
        };

        assert!(rendered.contains("<table class=\"data-table\">"));
        assert!(rendered.contains("<thead>"));
        assert!(rendered.contains("<tbody>"));
        assert!(rendered.contains("<button class=\"btn btn-edit\">"));
    }
}