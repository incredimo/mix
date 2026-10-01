package main

import (
	"fmt"
	"os"
	"path/filepath"

	"github.com/spf13/cobra"

	"mix/internal/converter"
)

var (
	version = "2.0.0"

	// Flags
	outputFile string
	styleFile  string
	theme      string
	noStyle    bool
	verbose    bool
	quiet      bool
)

func main() {
	rootCmd := &cobra.Command{
		Use:   "mix [file.md]",
		Short: "A powerful, self-contained Markdown to PDF converter",
		Long: `Mix converts Markdown files to beautifully styled PDF documents.

No external dependencies required - no Chrome, no wkhtmltopdf, just pure Go.

Examples:
  mix document.md              Convert document.md to document.pdf
  mix document.md -o out.pdf   Convert with custom output filename
  mix document.md -s custom.css Use custom stylesheet
  mix init .                   Initialize a new project with templates`,
		Args: cobra.MaximumNArgs(1),
		RunE: runConvert,
	}

	// Global flags
	rootCmd.PersistentFlags().BoolVarP(&verbose, "verbose", "v", false, "Show verbose output")
	rootCmd.PersistentFlags().BoolVarP(&quiet, "quiet", "q", false, "Suppress all output except errors")

	// Convert flags
	rootCmd.Flags().StringVarP(&outputFile, "output", "o", "", "Output PDF filename")
	rootCmd.Flags().StringVarP(&styleFile, "style", "s", "", "Custom CSS stylesheet")
	rootCmd.Flags().StringVarP(&theme, "theme", "t", "default", "Built-in theme (default, minimal, academic)")
	rootCmd.Flags().BoolVar(&noStyle, "no-style", false, "Disable all styling")

	// Init command
	initCmd := &cobra.Command{
		Use:   "init [directory]",
		Short: "Initialize a new Mix project with sample files",
		Long: `Creates a new Mix project with:
  - sample.md     A sample markdown document
  - style.css     A customizable stylesheet
  - .mixrc        Configuration file`,
		Args: cobra.MaximumNArgs(1),
		RunE: runInit,
	}
	rootCmd.AddCommand(initCmd)

	// Version command
	versionCmd := &cobra.Command{
		Use:   "version",
		Short: "Print version information",
		Run: func(cmd *cobra.Command, args []string) {
			fmt.Printf("mix version %s\n", version)
		},
	}
	rootCmd.AddCommand(versionCmd)

	if err := rootCmd.Execute(); err != nil {
		os.Exit(1)
	}
}

func runConvert(cmd *cobra.Command, args []string) error {
	if len(args) == 0 {
		return cmd.Help()
	}

	inputFile := args[0]

	// Check file exists
	if _, err := os.Stat(inputFile); os.IsNotExist(err) {
		return fmt.Errorf("file not found: %s", inputFile)
	}

	cfg := converter.Config{
		InputFile:  inputFile,
		OutputFile: outputFile,
		StyleFile:  styleFile,
		Theme:      theme,
		NoStyle:    noStyle,
		Verbose:    verbose,
		Quiet:      quiet,
	}

	conv := converter.New(cfg)
	return conv.Convert()
}

func runInit(cmd *cobra.Command, args []string) error {
	dir := "."
	if len(args) > 0 {
		dir = args[0]
	}

	// Create directory if it doesn't exist
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("failed to create directory: %w", err)
	}

	// Create sample.md
	sampleMD := `# Welcome to Mix

Mix is a powerful, self-contained Markdown to PDF converter.

## Features

- **Fast** - Written in Go for blazing speed
- **Beautiful** - Professional styling out of the box
- **Customizable** - Full CSS support for styling
- **Self-contained** - No external dependencies

## Example Content

Here's some sample content to demonstrate Mix's capabilities.

### Code Blocks

` + "```go" + `
package main

import "fmt"

func main() {
    fmt.Println("Hello, Mix!")
}
` + "```" + `

### Tables

| Feature | Status |
|---------|--------|
| Markdown | ✓ |
| Tables | ✓ |
| Code Blocks | ✓ |
| Links | ✓ |

### Blockquotes

> Mix makes document conversion simple and beautiful.
> No more fighting with complex toolchains.

### Lists

1. Write your content in Markdown
2. Run ` + "`mix document.md`" + `
3. Get a beautiful PDF

---

*Generated with Mix - the modern Markdown to PDF converter*
`

	// Create style.css
	styleCSS := `/* Mix Stylesheet
 * Customize your PDF output by editing this file.
 * Run: mix document.md
 */

/* Body text */
body {
    font-family: "Helvetica";
    font-size: 11pt;
    color: #333333;
    line-height: 1.4;
}

/* Headings */
h1 {
    font-size: 24pt;
    color: #111827;
    margin-bottom: 10pt;
}

h2 {
    font-size: 18pt;
    color: #1F2937;
    margin-top: 15pt;
    margin-bottom: 8pt;
    border-bottom: 1px solid #E5E7EB;
}

h3 {
    font-size: 14pt;
    color: #374151;
}

/* Code blocks */
code {
    font-family: "Courier";
    background-color: #F3F4F6;
    font-size: 10pt;
}

/* Links */
a {
    color: #3B82F6;
    text-decoration: none;
}

/* Blockquotes */
blockquote {
    color: #4B5563;
    background-color: #F9FAFB;
    border-left: 4px solid #D1D5DB;
    padding-left: 10pt;
}
`

	// Write files
	files := map[string]string{
		"sample.md": sampleMD,
		"style.css": styleCSS,
	}

	for name, content := range files {
		path := filepath.Join(dir, name)
		if _, err := os.Stat(path); err == nil {
			if !quiet {
				fmt.Printf("  • Skipping %s (already exists)\n", name)
			}
			continue
		}
		if err := os.WriteFile(path, []byte(content), 0644); err != nil {
			return fmt.Errorf("failed to write %s: %w", name, err)
		}
		if !quiet {
			fmt.Printf("  ✓ Created %s\n", name)
		}
	}

	if !quiet {
		fmt.Println()
		fmt.Println("Project initialized! Try:")
		fmt.Printf("  mix %s\n", filepath.Join(dir, "sample.md"))
	}

	return nil
}
