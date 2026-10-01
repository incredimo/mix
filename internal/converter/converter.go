package converter

import (
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"

	pdf "github.com/stephenafamo/goldmark-pdf"
	"github.com/yuin/goldmark"
	"github.com/yuin/goldmark/extension"
	"github.com/yuin/goldmark/parser"
)

// =============================================================================
// CONFIGURATION
// =============================================================================

// Config holds the complete conversion configuration
type Config struct {
	// Input/Output
	InputFile  string
	OutputFile string

	// Styling
	StyleFile string
	Theme     string
	NoStyle   bool

	// Page Layout
	PaperSize   string  // A4, Letter, Legal, A3, A5
	Orientation string  // portrait, landscape
	MarginTop   float64 // in points
	MarginBot   float64
	MarginLeft  float64
	MarginRight float64

	// Metadata
	Title    string
	Author   string
	Subject  string
	Keywords string

	// Output Control
	Verbose bool
	Quiet   bool
}

// DefaultConfig returns a config with sensible defaults
func DefaultConfig() Config {
	return Config{
		Theme:       "default",
		PaperSize:   "A4",
		Orientation: "portrait",
		MarginTop:   72, // 1 inch
		MarginBot:   72,
		MarginLeft:  72,
		MarginRight: 72,
	}
}

// =============================================================================
// CONVERTER
// =============================================================================

// Converter handles the complete Markdown to PDF conversion pipeline
type Converter struct {
	cfg      Config
	mu       sync.RWMutex
	logger   Logger
	registry *ThemeRegistry
}

// Logger defines a minimal logging interface
type Logger interface {
	Printf(format string, args ...interface{})
	Println(args ...interface{})
}

// defaultLogger wraps stdout with conditional output
type defaultLogger struct {
	verbose bool
	quiet   bool
	out     io.Writer
}

func (l *defaultLogger) Printf(format string, args ...interface{}) {
	if l.quiet {
		return
	}
	fmt.Fprintf(l.out, format, args...)
}

func (l *defaultLogger) Println(args ...interface{}) {
	if l.quiet {
		return
	}
	fmt.Fprintln(l.out, args...)
}

// New creates a new Converter with the given configuration
func New(cfg Config) *Converter {
	c := &Converter{
		cfg:      cfg,
		registry: NewThemeRegistry(),
		logger: &defaultLogger{
			verbose: cfg.Verbose,
			quiet:   cfg.Quiet,
			out:     os.Stdout,
		},
	}
	return c
}

// WithLogger sets a custom logger
func (c *Converter) WithLogger(l Logger) *Converter {
	c.mu.Lock()
	c.logger = l
	c.mu.Unlock()
	return c
}

// =============================================================================
// CONVERSION PIPELINE
// =============================================================================

// Convert performs the full conversion pipeline
func (c *Converter) Convert() error {
	start := time.Now()

	// Phase 1: Validate inputs
	if err := c.validate(); err != nil {
		return fmt.Errorf("validation failed: %w", err)
	}

	// Phase 2: Read source
	source, err := c.readSource()
	if err != nil {
		return err
	}

	// Phase 3: Determine output path
	outputFile := c.resolveOutputPath()

	// Phase 4: Build PDF configuration
	pdfConfig, err := c.buildPDFConfig(outputFile)
	if err != nil {
		return fmt.Errorf("failed to build PDF config: %w", err)
	}

	// Phase 5: Configure Goldmark
	md := c.buildGoldmark(pdfConfig)

	// Phase 6: Execute conversion
	c.log("Converting %s → %s", c.cfg.InputFile, outputFile)
	if err := c.executeConversion(md, source); err != nil {
		return err
	}

	// Report success
	elapsed := time.Since(start)
	c.log("✓ Done in %dms", elapsed.Milliseconds())

	return nil
}

// validate checks that the configuration is valid
func (c *Converter) validate() error {
	if c.cfg.InputFile == "" {
		return fmt.Errorf("input file is required")
	}

	info, err := os.Stat(c.cfg.InputFile)
	if os.IsNotExist(err) {
		return fmt.Errorf("input file does not exist: %s", c.cfg.InputFile)
	}
	if err != nil {
		return fmt.Errorf("cannot access input file: %w", err)
	}
	if info.IsDir() {
		return fmt.Errorf("input path is a directory, not a file: %s", c.cfg.InputFile)
	}

	// Validate paper size
	validPaperSizes := map[string]bool{
		"A3": true, "A4": true, "A5": true,
		"Letter": true, "Legal": true, "Tabloid": true,
	}
	if !validPaperSizes[c.cfg.PaperSize] && c.cfg.PaperSize != "" {
		return fmt.Errorf("invalid paper size: %s (valid: A3, A4, A5, Letter, Legal, Tabloid)", c.cfg.PaperSize)
	}

	return nil
}

// readSource reads the markdown source file
func (c *Converter) readSource() ([]byte, error) {
	source, err := os.ReadFile(c.cfg.InputFile)
	if err != nil {
		return nil, fmt.Errorf("failed to read input file %s: %w", c.cfg.InputFile, err)
	}

	if len(source) == 0 {
		return nil, fmt.Errorf("input file is empty: %s", c.cfg.InputFile)
	}

	c.logVerbose("Read %d bytes from %s", len(source), c.cfg.InputFile)
	return source, nil
}

// resolveOutputPath determines the output file path
func (c *Converter) resolveOutputPath() string {
	if c.cfg.OutputFile != "" {
		return c.cfg.OutputFile
	}
	ext := filepath.Ext(c.cfg.InputFile)
	return strings.TrimSuffix(c.cfg.InputFile, ext) + ".pdf"
}

// buildPDFConfig creates the PDF configuration with all styling applied
func (c *Converter) buildPDFConfig(outputFile string) (pdf.Config, error) {
	pdfCfg := pdf.Config{
		OutputFile: outputFile,
		PaperSize:  c.resolvePaperSize(),
	}

	// Apply styles unless disabled
	if !c.cfg.NoStyle {
		if err := c.applyAllStyles(&pdfCfg); err != nil {
			// Don't fail on style errors, just warn
			c.log("Warning: Style application issue: %v", err)
		}
	}

	return pdfCfg, nil
}

// resolvePaperSize resolves the paper size string
func (c *Converter) resolvePaperSize() string {
	if c.cfg.PaperSize == "" {
		return "A4"
	}
	return c.cfg.PaperSize
}

// buildGoldmark creates a configured goldmark instance
func (c *Converter) buildGoldmark(pdfCfg pdf.Config) goldmark.Markdown {
	return goldmark.New(
		goldmark.WithExtensions(
			extension.Table,
			extension.Strikethrough,
			extension.TaskList,
			extension.Linkify,
			extension.DefinitionList,
			extension.Footnote,
			extension.Typographer,
		),
		goldmark.WithParserOptions(
			parser.WithAutoHeadingID(),
			parser.WithAttribute(),
		),
		goldmark.WithRenderer(
			pdf.New(
				pdf.WithConfig(pdfCfg),
				pdf.WithTraceWriter(c.getTraceWriter()),
			),
		),
	)
}

// executeConversion runs the goldmark conversion
func (c *Converter) executeConversion(md goldmark.Markdown, source []byte) error {
	ctx := parser.NewContext()
	if err := md.Convert(source, nil, parser.WithContext(ctx)); err != nil {
		return fmt.Errorf("conversion failed: %w", err)
	}
	return nil
}

// =============================================================================
// STYLE APPLICATION
// =============================================================================

// applyAllStyles applies theme and custom CSS in the correct order
func (c *Converter) applyAllStyles(config *pdf.Config) error {
	// Layer 1: Apply base theme
	theme := c.registry.Get(c.cfg.Theme)
	if theme == nil {
		c.logVerbose("Theme '%s' not found, using default", c.cfg.Theme)
		theme = c.registry.Get("default")
	}
	if theme != nil {
		theme.Apply(config)
		c.logVerbose("Applied theme: %s", theme.Name)
	}

	// Layer 2: Apply custom CSS file (if specified or found)
	cssFile := c.resolveStyleFile()
	if cssFile != "" {
		c.logVerbose("Loading custom styles from %s", cssFile)
		if err := parseAndApplyCSS(cssFile, config); err != nil {
			return fmt.Errorf("CSS parsing failed for %s: %w", cssFile, err)
		}
	}

	return nil
}

// resolveStyleFile finds the CSS file to use
func (c *Converter) resolveStyleFile() string {
	// Explicit style file takes priority
	if c.cfg.StyleFile != "" {
		if _, err := os.Stat(c.cfg.StyleFile); err == nil {
			return c.cfg.StyleFile
		}
		c.log("Warning: Specified style file not found: %s", c.cfg.StyleFile)
		return ""
	}

	// Check for style.css in input directory
	inputDir := filepath.Dir(c.cfg.InputFile)
	localStyle := filepath.Join(inputDir, "style.css")
	if _, err := os.Stat(localStyle); err == nil {
		return localStyle
	}

	// Check for .mix/style.css in input directory
	mixStyle := filepath.Join(inputDir, ".mix", "style.css")
	if _, err := os.Stat(mixStyle); err == nil {
		return mixStyle
	}

	return ""
}

// =============================================================================
// LOGGING
// =============================================================================

func (c *Converter) getTraceWriter() io.Writer {
	if c.cfg.Verbose {
		return os.Stdout
	}
	return io.Discard
}

func (c *Converter) log(format string, args ...interface{}) {
	if c.cfg.Quiet {
		return
	}
	c.logger.Printf(format+"\n", args...)
}

func (c *Converter) logVerbose(format string, args ...interface{}) {
	if !c.cfg.Verbose || c.cfg.Quiet {
		return
	}
	c.logger.Printf("[verbose] "+format+"\n", args...)
}
