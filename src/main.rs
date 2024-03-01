use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use markdown_it::parser::inline::Text;
use markdown_it::MarkdownIt;
use markdown_it::Node;
use minimo::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const BANNER: &str = r#"
███╗   ███╗ ██╗ ██╗  ██╗  🌈 MIX MARKDOWN ENGINE
████╗ ████║ ██║ ╚██╗██╔╝  -----------------------------------
██╔████╔██║ ██║  ╚███╔╝   For those who'd rather eat broccoli
██║╚██╔╝██║ ██║  ██╔██╗   than open another word file 🥦
██║ ╚═╝ ██║ ██║ ██╔╝ ██╗  -----------------------------------
╚═╝     ╚═╝ ╚═╝ ╚═╝  ╚═╝  github.com/incredimo/mix
"#;

// const BANNER: &str = r#"
//  __    __     __     __  __
// /\ "-./  \   /\ \   /\_\_\_\      [MIX] MARKDOWN ENGINE
// \ \ \-./\ \  \ \ \  \/_/\_\/_     For those who'd rather eat broccoli
//  \ \_\ \ \_\  \ \_\   /\_\/\_\    than open another word file 🥦
//   \/_/  \/_/   \/_/   \/_/\/_/    github.com/incredimo/mix
// "#;

fn main() {
    minimo::showln!(white, BANNER);
    // divider();
    startup();
}

/// check if args are passed and if they are valid
/// handle args if any. if no args are passed, start the interactive mode
/// display selction menu asking what to do next
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

use std::sync::Arc;
/// interactive mode
/// display selction menu asking what to do next
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

    let selection = minimo::selection!("tell mix wht to do", &choices);
    selection.run();
}

/// create a new mx config, style.css and [file].md
fn init() {
    let active_dir = std::env::current_dir().unwrap();
    let config = active_dir.join("mx.toml");
    let style = active_dir.join("style.css");
    let file = active_dir.join("example.md");

    if style.exists() {
        showln!(red_bold, "style.css already exists");
    } else {
        std::fs::write(style, include_str!("default_styles.css")).unwrap();
        showln!(gray, "created ", green_bold, "style.css");
    }

    if file.exists() {
        showln!(red_bold, "example.md already exists");
    } else {
        std::fs::write(file, include_str!("../readme.md")).unwrap();
        showln!(gray, "created ", green_bold, "example.md");
    }
}

/// build the [file].md to [file].html and [file].pdf
fn build() {
    let files = get_files_in_dir();
    for file in files {
        let file_content = read_file(file.clone());
        let html = markdown_to_html(file_content);
        let html_file = file.replace(".md", ".html");
        write_file(html_file, html.clone());
        let pdf_file = file.replace(".md", ".pdf");
        html_to_pdf(html, pdf_file);
    }
}

/// watches the [file].md for changes and rebuilds the [file].html and [file].pdf
fn watch() {
    let files = get_files_in_dir();
    for file in files {
        let file_content = read_file(file.clone());
        let html = markdown_to_html(file_content);
        let html_file = file.replace(".md", ".html");
        write_file(html_file, html.clone());
        let pdf_file = file.replace(".md", ".pdf");
        html_to_pdf(html, pdf_file);
        watch_file(file);
    }
}

fn markdown_to_html(markdown: String) -> String {
    let md = initialize_mx();
    let mut parsed = md.parse(&markdown);
    //add style to the html
    let style = style_in_dir_or_default();
    parsed
        .children
        .insert(0, Node::new(Text { content: style }));
    let html = parsed.render();
    html
}

use pdf_writer::*;
fn html_to_pdf(html: String, pdf: String) {
    let mut pdf = pdf_writer::Pdf::new()
    .pages(Ref::new(1)).insert(0, pdf_writer::Page::new(Ref::new(1)).content(Content::new(html)));

    

}

fn watch_file(file: String) {
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => {
            let file_content = read_file(file.clone());
            let html = markdown_to_html(file_content);
            let html_file = file.replace(".md", ".html");
            write_file(html_file, html.clone());
            let pdf_file = file.replace(".md", ".pdf");
            html_to_pdf(html, pdf_file);
        }
        Err(e) => println!("watch error: {:?}", e),
    });
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

fn get_files_in_dir() -> Vec<String> {
    let active_dir = std::env::current_dir().unwrap();
    let files = std::fs::read_dir(active_dir).unwrap();
    let mut markdown_files = vec![];
    for file in files {
        let file = file.unwrap().path();
        let file = file.to_str().unwrap().to_string();
        if file.ends_with(".md") {
            markdown_files.push(file);
        }
    }
    markdown_files
}

fn read_file(file: String) -> String {
    let file_content = std::fs::read_to_string(file).unwrap();
    file_content
}

fn write_file(file: String, content: String) {
    std::fs::write(file.clone(), content).unwrap();
    showln!(gray, "created ", green_bold, &file);
}

fn initialize_mx() -> MarkdownIt {
    let mut md = markdown_it::MarkdownIt::new();

    // add commonmark syntax, you almost always want to do that
    markdown_it::plugins::cmark::add(&mut md);
    markdown_it::plugins::extra::add(&mut md);
    markdown_it::plugins::html::add(&mut md);
    // markdown_it::plugins::sourcepos::add(&mut md);

    return md;
}
