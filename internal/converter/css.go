package converter

import (
	"fmt"
	"image/color"
	"os"
	"strconv"
	"strings"

	"github.com/gorilla/css/scanner"
	pdf "github.com/stephenafamo/goldmark-pdf"
)

// =============================================================================
// CSS PARSER
// =============================================================================

// CSSParser handles parsing and applying CSS to PDF configuration
type CSSParser struct {
	config *pdf.Config
	errors []error
}

// parseAndApplyCSS reads a CSS file and applies styles to the PDF config
func parseAndApplyCSS(filename string, config *pdf.Config) error {
	content, err := os.ReadFile(filename)
	if err != nil {
		return fmt.Errorf("failed to read CSS file: %w", err)
	}

	parser := &CSSParser{config: config}
	return parser.Parse(string(content))
}

// Parse processes CSS content and applies it to the config
func (p *CSSParser) Parse(content string) error {
	s := scanner.New(content)

	for {
		token := s.Next()
		if token.Type == scanner.TokenEOF {
			break
		}
		if token.Type == scanner.TokenError {
			p.errors = append(p.errors, fmt.Errorf("CSS scan error: %s", token.Value))
			continue
		}

		// Look for selectors
		if token.Type == scanner.TokenIdent || token.Type == scanner.TokenHash {
			selector := p.normalizeSelector(token.Value)

			// Consume until we hit the opening brace
			if !p.scanUntilBrace(s) {
				break
			}

			// Parse and apply the declaration block
			p.parseDeclarationBlock(s, selector)
		}
	}

	if len(p.errors) > 0 {
		return fmt.Errorf("CSS parsing encountered %d errors", len(p.errors))
	}
	return nil
}

// scanUntilBrace advances scanner until '{' is found
func (p *CSSParser) scanUntilBrace(s *scanner.Scanner) bool {
	for {
		token := s.Next()
		if token.Type == scanner.TokenEOF {
			return false
		}
		if token.Type == scanner.TokenChar && token.Value == "{" {
			return true
		}
	}
}

// normalizeSelector cleans and standardizes selector strings
func (p *CSSParser) normalizeSelector(s string) string {
	s = strings.TrimSpace(s)
	s = strings.ToLower(s)
	s = strings.TrimPrefix(s, "#")
	s = strings.TrimPrefix(s, ".")
	return s
}

// parseDeclarationBlock parses a CSS declaration block and applies to the appropriate style
func (p *CSSParser) parseDeclarationBlock(s *scanner.Scanner, selector string) {
	stylePtr := p.getStyleForSelector(selector)
	isLink := selector == "a" || selector == "link"
	isCode := selector == "code" || selector == "pre" || selector == "kbd" || selector == "samp"

	if stylePtr == nil && !isLink && !isCode {
		// Consume and discard unknown selector's block
		p.consumeBlock(s)
		return
	}

	for {
		token := s.Next()
		if token.Type == scanner.TokenChar && token.Value == "}" {
			return
		}
		if token.Type == scanner.TokenEOF {
			return
		}

		if token.Type == scanner.TokenIdent {
			property := token.Value

			// Skip the colon
			p.skipUntil(s, ":")

			// Read the value
			value := p.readPropertyValue(s)

			// Apply the property
			if isLink && property == "color" {
				p.config.Styles.LinkColor = hexToColor(normalizeColor(value))
			} else if isCode {
				p.applyFontProperty(&p.config.Styles.CodeFont, property, value)
			} else if stylePtr != nil {
				p.applyProperty(stylePtr, property, value)
			}
		}
	}
}

// consumeBlock discards all tokens until the closing brace
func (p *CSSParser) consumeBlock(s *scanner.Scanner) {
	depth := 1
	for depth > 0 {
		token := s.Next()
		if token.Type == scanner.TokenEOF {
			return
		}
		if token.Type == scanner.TokenChar {
			if token.Value == "{" {
				depth++
			} else if token.Value == "}" {
				depth--
			}
		}
	}
}

// skipUntil advances until a specific character is found
func (p *CSSParser) skipUntil(s *scanner.Scanner, char string) {
	for {
		token := s.Next()
		if token.Type == scanner.TokenEOF || token.Value == char {
			return
		}
	}
}

// readPropertyValue reads a CSS property value until ; or }
func (p *CSSParser) readPropertyValue(s *scanner.Scanner) string {
	var parts []string
	for {
		token := s.Next()
		if token.Type == scanner.TokenEOF {
			break
		}
		if token.Value == ";" || token.Value == "}" {
			break
		}
		if token.Type != scanner.TokenS { // Skip whitespace tokens
			parts = append(parts, token.Value)
		}
	}
	return strings.Join(parts, " ")
}

// getStyleForSelector returns the pdf.Style pointer for a given selector
func (p *CSSParser) getStyleForSelector(selector string) *pdf.Style {
	switch selector {
	case "body", "p", "div", "normal":
		return p.config.Styles.Normal
	case "h1":
		return p.config.Styles.H1
	case "h2":
		return p.config.Styles.H2
	case "h3":
		return p.config.Styles.H3
	case "h4":
		return p.config.Styles.H4
	case "h5":
		return p.config.Styles.H5
	case "h6":
		return p.config.Styles.H6
	case "blockquote", "quote":
		return p.config.Styles.Blockquote
	case "th", "thead":
		return p.config.Styles.THeader
	case "td", "tbody":
		return p.config.Styles.TBody
	default:
		return nil
	}
}

// applyProperty applies a single CSS property to a style
func (p *CSSParser) applyProperty(style *pdf.Style, property, value string) {
	value = strings.TrimSpace(value)
	property = strings.ToLower(strings.TrimSpace(property))

	switch property {
	case "color":
		style.Font.Color = hexToColor(normalizeColor(value))
	case "font-family":
		style.Font.Family = normalizeFont(value)
	case "font-size":
		style.Font.Size = parseSize(value)
	case "font-weight":
		p.applyFontWeight(style, value)
	case "font-style":
		p.applyFontStyle(style, value)
	case "font":
		p.applyFontShorthand(style, value)
	}
}

// applyFontProperty applies a CSS property to a pdf.Font
func (p *CSSParser) applyFontProperty(font *pdf.Font, property, value string) {
	value = strings.TrimSpace(value)
	property = strings.ToLower(strings.TrimSpace(property))

	switch property {
	case "color":
		font.Color = hexToColor(normalizeColor(value))
	case "font-family":
		font.Family = normalizeFont(value)
	case "font-size":
		font.Size = parseSize(value)
	case "font-weight":
		if value == "bold" || value == "bolder" || value == "700" || value == "800" || value == "900" {
			if strings.Contains(font.Style, "I") {
				font.Style = "BI"
			} else {
				font.Style = "B"
			}
		}
	case "font-style":
		if value == "italic" || value == "oblique" {
			if strings.Contains(font.Style, "B") {
				font.Style = "BI"
			} else {
				font.Style = "I"
			}
		}
	}
}

// applyFontWeight handles font-weight property
func (p *CSSParser) applyFontWeight(style *pdf.Style, value string) {
	value = strings.ToLower(value)
	if value == "bold" || value == "bolder" || value == "700" || value == "800" || value == "900" {
		if strings.Contains(style.Font.Style, "I") {
			style.Font.Style = "BI"
		} else {
			style.Font.Style = "B"
		}
	} else if value == "normal" || value == "400" || value == "lighter" {
		style.Font.Style = strings.ReplaceAll(style.Font.Style, "B", "")
	}
}

// applyFontStyle handles font-style property
func (p *CSSParser) applyFontStyle(style *pdf.Style, value string) {
	value = strings.ToLower(value)
	if value == "italic" || value == "oblique" {
		if strings.Contains(style.Font.Style, "B") {
			style.Font.Style = "BI"
		} else {
			style.Font.Style = "I"
		}
	} else if value == "normal" {
		style.Font.Style = strings.ReplaceAll(style.Font.Style, "I", "")
	}
}

// applyFontShorthand handles the font shorthand property
func (p *CSSParser) applyFontShorthand(style *pdf.Style, value string) {
	parts := strings.Fields(value)
	for _, part := range parts {
		lower := strings.ToLower(part)

		// Check for style
		if lower == "italic" || lower == "oblique" {
			p.applyFontStyle(style, lower)
			continue
		}

		// Check for weight
		if lower == "bold" || lower == "bolder" || lower == "lighter" {
			p.applyFontWeight(style, lower)
			continue
		}

		// Check for numeric weight
		if w, err := strconv.Atoi(part); err == nil && w >= 100 && w <= 900 {
			p.applyFontWeight(style, part)
			continue
		}

		// Check for size (has unit)
		if strings.HasSuffix(lower, "pt") || strings.HasSuffix(lower, "px") ||
			strings.HasSuffix(lower, "em") || strings.HasSuffix(lower, "rem") {
			style.Font.Size = parseSize(part)
			continue
		}

		// Otherwise treat as font family
		style.Font.Family = normalizeFont(part)
	}
}

// =============================================================================
// COLOR UTILITIES
// =============================================================================

// colorNames maps CSS color names to hex values
var colorNames = map[string]string{
	// Basic colors
	"black":   "#000000",
	"white":   "#FFFFFF",
	"red":     "#FF0000",
	"green":   "#008000",
	"blue":    "#0000FF",
	"yellow":  "#FFFF00",
	"cyan":    "#00FFFF",
	"magenta": "#FF00FF",

	// Extended colors
	"gray":       "#808080",
	"grey":       "#808080",
	"silver":     "#C0C0C0",
	"maroon":     "#800000",
	"olive":      "#808000",
	"lime":       "#00FF00",
	"aqua":       "#00FFFF",
	"teal":       "#008080",
	"navy":       "#000080",
	"fuchsia":    "#FF00FF",
	"purple":     "#800080",
	"orange":     "#FFA500",
	"pink":       "#FFC0CB",
	"brown":      "#A52A2A",
	"coral":      "#FF7F50",
	"crimson":    "#DC143C",
	"darkblue":   "#00008B",
	"darkgray":   "#A9A9A9",
	"darkgreen":  "#006400",
	"darkred":    "#8B0000",
	"gold":       "#FFD700",
	"indigo":     "#4B0082",
	"ivory":      "#FFFFF0",
	"khaki":      "#F0E68C",
	"lavender":   "#E6E6FA",
	"lightblue":  "#ADD8E6",
	"lightgray":  "#D3D3D3",
	"lightgreen": "#90EE90",
	"salmon":     "#FA8072",
	"tan":        "#D2B48C",
	"tomato":     "#FF6347",
	"turquoise":  "#40E0D0",
	"violet":     "#EE82EE",
	"wheat":      "#F5DEB3",
}

// normalizeColor converts CSS color values to hex format
func normalizeColor(c string) string {
	c = strings.TrimSpace(c)
	c = strings.ToLower(c)

	// Already a hex color
	if strings.HasPrefix(c, "#") {
		// Expand shorthand (#RGB -> #RRGGBB)
		if len(c) == 4 {
			return "#" + string(c[1]) + string(c[1]) + string(c[2]) + string(c[2]) + string(c[3]) + string(c[3])
		}
		return strings.ToUpper(c)
	}

	// Check named colors
	if hex, ok := colorNames[c]; ok {
		return hex
	}

	// Handle rgb() and rgba()
	if strings.HasPrefix(c, "rgb") {
		return parseRGBColor(c)
	}

	// Default fallback
	return "#000000"
}

// parseRGBColor parses rgb(r, g, b) or rgba(r, g, b, a) format
func parseRGBColor(c string) string {
	c = strings.TrimPrefix(c, "rgba(")
	c = strings.TrimPrefix(c, "rgb(")
	c = strings.TrimSuffix(c, ")")

	parts := strings.Split(c, ",")
	if len(parts) < 3 {
		return "#000000"
	}

	r := parseColorComponent(strings.TrimSpace(parts[0]))
	g := parseColorComponent(strings.TrimSpace(parts[1]))
	b := parseColorComponent(strings.TrimSpace(parts[2]))

	return fmt.Sprintf("#%02X%02X%02X", r, g, b)
}

// parseColorComponent parses a single color component (0-255 or percentage)
func parseColorComponent(s string) int {
	if strings.HasSuffix(s, "%") {
		s = strings.TrimSuffix(s, "%")
		pct, _ := strconv.ParseFloat(s, 64)
		return int(pct * 255 / 100)
	}
	v, _ := strconv.Atoi(s)
	if v < 0 {
		v = 0
	}
	if v > 255 {
		v = 255
	}
	return v
}

// hexStringToColor converts a hex color string to color.Color
func hexStringToColor(hex string) color.Color {
	return hexToColor(hex)
}

// =============================================================================
// FONT UTILITIES
// =============================================================================

// fontAliases maps common font names to PDF-standard fonts
var fontAliases = map[string]string{
	// Monospace
	"courier":        "Courier",
	"courier new":    "Courier",
	"consolas":       "Courier",
	"monaco":         "Courier",
	"monospace":      "Courier",
	"menlo":          "Courier",
	"source code":    "Courier",
	"fira code":      "Courier",
	"jetbrains":      "Courier",
	"lucida console": "Courier",

	// Serif
	"times":           "Times",
	"times new roman": "Times",
	"georgia":         "Times",
	"palatino":        "Times",
	"garamond":        "Times",
	"serif":           "Times",
	"cambria":         "Times",
	"baskerville":     "Times",
	"book antiqua":    "Times",

	// Sans-serif
	"helvetica":     "Helvetica",
	"arial":         "Helvetica",
	"sans-serif":    "Helvetica",
	"verdana":       "Helvetica",
	"tahoma":        "Helvetica",
	"trebuchet":     "Helvetica",
	"calibri":       "Helvetica",
	"segoe ui":      "Helvetica",
	"roboto":        "Helvetica",
	"open sans":     "Helvetica",
	"lato":          "Helvetica",
	"inter":         "Helvetica",
	"system-ui":     "Helvetica",
	"apple system":  "Helvetica",
	"-apple-system": "Helvetica",
	"sf pro":        "Helvetica",
}

// normalizeFont maps CSS font families to PDF-standard fonts
func normalizeFont(f string) string {
	// Clean up the input
	f = strings.Trim(f, "\"' ")

	// Try direct lookup
	if mapped, ok := fontAliases[strings.ToLower(f)]; ok {
		return mapped
	}

	// Try partial matches
	lower := strings.ToLower(f)

	// Check for monospace indicators
	if strings.Contains(lower, "mono") || strings.Contains(lower, "courier") ||
		strings.Contains(lower, "consola") || strings.Contains(lower, "code") {
		return "Courier"
	}

	// Check for serif indicators
	if strings.Contains(lower, "times") || strings.Contains(lower, "serif") ||
		strings.Contains(lower, "georgia") || strings.Contains(lower, "palatino") {
		if !strings.Contains(lower, "sans") {
			return "Times"
		}
	}

	// Default to Helvetica
	return "Helvetica"
}

// =============================================================================
// SIZE PARSING
// =============================================================================

// parseSize converts CSS size values to points
func parseSize(s string) float64 {
	s = strings.TrimSpace(strings.ToLower(s))

	// Handle named sizes
	switch s {
	case "xx-small":
		return 7
	case "x-small":
		return 8
	case "small":
		return 10
	case "medium":
		return 12
	case "large":
		return 14
	case "x-large":
		return 18
	case "xx-large":
		return 24
	case "xxx-large":
		return 32
	case "smaller":
		return 10
	case "larger":
		return 14
	}

	// Extract numeric value and unit
	var value float64
	var unit string

	for i, c := range s {
		if (c < '0' || c > '9') && c != '.' {
			value, _ = strconv.ParseFloat(s[:i], 64)
			unit = s[i:]
			break
		}
	}

	if value == 0 {
		value, _ = strconv.ParseFloat(s, 64)
	}

	// Convert based on unit
	unit = strings.TrimSpace(unit)
	switch unit {
	case "pt", "":
		return value
	case "px":
		return value * 0.75 // px to pt
	case "em", "rem":
		return value * 12 // Assume 12pt base
	case "in":
		return value * 72 // inches to points
	case "cm":
		return value * 28.35 // cm to points
	case "mm":
		return value * 2.835 // mm to points
	case "%":
		return value * 12 / 100 // Percentage of 12pt base
	default:
		return value
	}
}
