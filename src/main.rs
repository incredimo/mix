// main.rs

use pdf_writer::{Pdf, Rect, Ref, Filter, Finish};
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::channel;
use std::sync::Arc;
use markdown_it::parser::inline::Text;
use markdown_it::{MarkdownIt, Node};
use minimo::*;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, DebouncedEvent, watcher};
use std::path::{Path, PathBuf};
use std::time::Duration;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const BANNER: &str = r#"
███╗   ███╗ ██╗ ██╗  ██╗  🌈 MIX MARKDOWN ENGINE
████╗ ████║ ██║ ╚██╗██╔╝  -----------------------------------
██╔████╔██║ ██║  ╚███╔╝   For those who'd rather eat broccoli
██║╚██╔╝██║ ██║  ██╔██╗   than open another word file 🥦
██║ ╚═╝ ██║ ██║ ██╔╝ ██╗  -----------------------------------
╚═╝     ╚═╝ ╚═╝ ╚═╝  ╚═╝  github.com/incredimo/mix
"#;

fn main() {
    minimo::showln!(white, BANNER);
    startup();
}

/// check if args are passed and if they are valid
/// handle args if any. if no args are passed, start the interactive mode
/// display selection menu asking what to do next
fn startup() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        handle_args(args);
    } else {
        interactive();
    }
}

/// handle args
/// init - creates a new mx config, style.css and [file].md
/// build - builds the [file].md to [file].html and [file].pdf
/// watch - watches the [file].md for changes and rebuilds the [file].html and [file].pdf
fn handle_args(args: Vec<String>) {
    let command = &args[1];
    match command.as_str() {
        "init" => {
            init();
        }
        "build" => {
            build();
        }
        "watch" => {
            watch();
        }
        _ => {
            showln!(red_bold, "invalid command");
        }
    }
}

/// interactive mode
/// display selection menu asking what to do next
fn interactive() {
    let choices = vec![
        minimo::choice!("init", "create a new mx config", || {
            init();
        }),
        minimo::choice!("build", "build the markdown file", || {
            build();
        }),
        minimo::choice!("watch", "watch the markdown file for changes", || {
            watch();
        }),
        minimo::choice!("exit", "exit", || {
            std::process::exit(0);
        }),
    ];

    let selection = minimo::selection!("tell mix what to do", &choices);
    selection.run();
}

/// create a new mx config, style.css and [file].md
fn init() {
    let active_dir = std::env::current_dir().unwrap();
    let style = active_dir.join("style.css");
    let file = active_dir.join("example.md");

    if style.exists() {
        showln!(red_bold, "style.css already exists");
    } else {
        std::fs::write(&style, include_str!("default_styles.css")).unwrap();
        showln!(gray, "created ", green_bold, "style.css");
    }

    if file.exists() {
        showln!(red_bold, "example.md already exists");
    } else {
        std::fs::write(&file, include_str!("../readme.md")).unwrap();
        showln!(gray, "created ", green_bold, "example.md");
    }
}

/// build the [file].md to [file].html and [file].pdf
fn build() {
    let files = get_files_in_dir(&std::env::current_dir().unwrap());
    for file in files {
        let file_content = read_file(&file);
        let html = markdown_to_html(&file_content);
        let html_file = file.with_extension("html");
        write_file(&html_file, &html);
        let pdf_file = file.with_extension("pdf");
        html_to_pdf(&html, &pdf_file);
    }
}

/// watches the directory and its subdirectories for changes in .md and .css files and rebuilds the necessary files
fn watch() {
    let active_dir = std::env::current_dir().unwrap();
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = watcher(tx, Duration::from_secs(2)).unwrap();

    watcher.watch(&active_dir, RecursiveMode::Recursive).unwrap();
    loop {
        match rx.recv() {
            Ok(event) => match event {
                DebouncedEvent::Write(path) | DebouncedEvent::Create(path) => {
                    if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
                        handle_md_file(&path);
                    } else if path.extension().and_then(|ext| ext.to_str()) == Some("css") {
                        handle_css_file(&path);
                    }
                }
                _ => (),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn handle_md_file(path: &Path) {
    let file_content = read_file(path);
    let html = markdown_to_html(&file_content);
    let html_file = path.with_extension("html");
    write_file(&html_file, &html);
    let pdf_file = path.with_extension("pdf");
    html_to_pdf(&html, &pdf_file);
}

fn handle_css_file(css_path: &Path) {
    // Rebuild all markdown files in the same directory as the changed CSS file
    let css_dir = css_path.parent().unwrap();
    let files = get_files_in_dir(css_dir);
    for file in files {
        let file_content = read_file(&file);
        let html = markdown_to_html(&file_content);
        let html_file = file.with_extension("html");
        write_file(&html_file, &html);
        let pdf_file = file.with_extension("pdf");
        html_to_pdf(&html, &pdf_file);
    }
}

fn markdown_to_html(markdown: &str) -> String {
    let md = initialize_mx();
    let mut parsed = md.parse(markdown);
    // Add style to the HTML
    let style = style_in_dir_or_default();
    parsed.children.insert(0, Node::new(Text { content: style }));
    parsed.render()
}

fn html_to_pdf(html: &str, pdf_path: &str) {
    let mut pdf = Pdf::new();
    let page_id = Ref::new(1);
    let page_tree_id = Ref::new(2);
    let content_id = Ref::new(3);
    
    pdf.catalog(Ref::new(0)).pages(page_tree_id);
    pdf.pages(page_tree_id).kids([page_id]).count(1);
    pdf.page(page_id)
        .parent(page_tree_id)
        .media_box(Rect::new(0.0, 0.0, 595.0, 842.0))
        .contents(content_id);
    
    pdf.stream(content_id, html.as_bytes())
        .filter(Filter::FlateDecode);

    std::fs::write(pdf_path, pdf.finish()).unwrap();
}

fn style_in_dir_or_default() -> String {
    let active_dir = std::env::current_dir().unwrap();
    let style = active_dir.join("style.css");
    if style.exists() {
        std::fs::read_to_string(style).unwrap()
    } else {
        let default_style = include_str!("default_styles.css");
        default_style.to_string()
    }
}

fn get_files_in_dir(dir: &Path) -> Vec<PathBuf> {
    let files = std::fs::read_dir(dir).unwrap();
    let mut markdown_files = vec![];
    for file in files {
        let file = file.unwrap().path();
        if file.extension().and_then(|ext| ext.to_str()) == Some("md") {
            markdown_files.push(file);
        }
    }
    markdown_files
}

fn read_file(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn write_file(path: &Path, content: &str) {
    std::fs::write(path, content).unwrap();
    showln!(gray, "created ", green_bold, path.display());
}

fn initialize_mx() -> MarkdownIt {
    let mut md = MarkdownIt::new();

    // add commonmark syntax, you almost always want to do that
    markdown_it::plugins::cmark::add(&mut md);
    markdown_it::plugins::extra::add(&mut md);
    markdown_it::plugins::html::add(&mut md);

    md
}