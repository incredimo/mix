# Mix

A powerful, self-contained Markdown to PDF converter written in Go.

**No external dependencies** - no Chrome, no wkhtmltopdf, no browser automation. Just pure Go.

## Features

- ⚡ **Fast** - Written in Go for blazing speed
- 🎨 **Beautiful** - Professional styling out of the box
- 🎯 **Simple** - Just run `mix document.md`
- 📝 **CSS Styling** - Full CSS support for customization
- 📦 **Self-contained** - Single binary, no dependencies
- 🔧 **Customizable** - Themes, styles, CLI flags

## Installation

```bash
go install github.com/incredimo/mix@latest
```

Or build from source:

```bash
git clone https://github.com/incredimo/mix
cd mix
go build .
```

## Quick Start

### Initialize a Project

```bash
mix init .
```

This creates:
- `sample.md` - A sample markdown document
- `style.css` - A customizable stylesheet

### Convert a Document

```bash
mix document.md
```

Creates `document.pdf` in the same directory.

## Usage

```
mix [file.md] [flags]

Flags:
  -o, --output string   Output PDF filename
  -s, --style string    Custom CSS stylesheet
  -t, --theme string    Built-in theme (default, minimal, academic)
      --no-style        Disable all styling
  -v, --verbose         Show verbose output
  -q, --quiet           Suppress all output except errors

Commands:
  mix init [dir]        Initialize a new project with sample files
  mix version           Print version information
  mix help              Show help
```

## Examples

```bash
# Basic conversion
mix document.md

# Custom output filename
mix document.md -o report.pdf

# Use custom stylesheet
mix document.md -s corporate-style.css

# Verbose output for debugging
mix document.md -v

# Initialize in a new directory
mix init my-project
```

## Styling with CSS

Mix automatically looks for `style.css` in the same directory as your markdown file. You can also specify a custom stylesheet with `-s`.

### Supported CSS Selectors

| Selector | Description |
|----------|-------------|
| `body` | Main document text |
| `h1` - `h6` | Headings |
| `code`, `pre` | Code blocks |
| `blockquote` | Block quotes |
| `a` | Links |

### Supported CSS Properties

| Property | Example |
|----------|---------|
| `font-family` | `"Helvetica"`, `"Times"`, `"Courier"` |
| `font-size` | `12pt`, `16px` |
| `color` | `#333333`, `#F00` |
| `background-color` | `#F5F5F5` |
| `line-height` | `18pt` |

### Example style.css

```css
body {
    font-family: "Helvetica";
    font-size: 11pt;
    color: #333333;
}

h1 {
    font-size: 28pt;
    color: #111827;
}

h2 {
    font-size: 22pt;
    color: #1F2937;
}

code {
    font-family: "Courier";
    background-color: #F3F4F6;
}

a {
    color: #3B82F6;
}
```

## Markdown Features

Mix supports GitHub Flavored Markdown (GFM):

- **Headings** (H1-H6)
- **Bold** and *italic* text
- ~~Strikethrough~~
- `Inline code`
- Code blocks with syntax highlighting
- Tables
- Blockquotes
- Ordered and unordered lists
- Links and images
- Horizontal rules
- Task lists

## License

MIT License
