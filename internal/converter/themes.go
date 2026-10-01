package converter

import (
	"image/color"
	"sync"

	pdf "github.com/stephenafamo/goldmark-pdf"
)

// =============================================================================
// THEME SYSTEM
// =============================================================================

// Theme represents a complete visual theme for PDF output
type Theme struct {
	Name        string
	Description string
	Styles      ThemeStyles
}

// ThemeStyles holds all style definitions for a theme
type ThemeStyles struct {
	Normal     FontDef
	H1         FontDef
	H2         FontDef
	H3         FontDef
	H4         FontDef
	H5         FontDef
	H6         FontDef
	CodeFont   FontDef
	Blockquote FontDef
	LinkColor  string // hex color
	THeader    FontDef
	TBody      FontDef
}

// FontDef defines font properties
type FontDef struct {
	Family string
	Size   float64
	Color  string
	Style  string // B=Bold, I=Italic, BI=BoldItalic
}

// Apply applies this theme to a PDF config
func (t *Theme) Apply(config *pdf.Config) {
	if t == nil {
		return
	}

	config.Styles.Normal = t.Styles.Normal.toStyle()
	config.Styles.H1 = t.Styles.H1.toStyle()
	config.Styles.H2 = t.Styles.H2.toStyle()
	config.Styles.H3 = t.Styles.H3.toStyle()
	config.Styles.H4 = t.Styles.H4.toStyle()
	config.Styles.H5 = t.Styles.H5.toStyle()
	config.Styles.H6 = t.Styles.H6.toStyle()
	config.Styles.Blockquote = t.Styles.Blockquote.toStyle()
	config.Styles.THeader = t.Styles.THeader.toStyle()
	config.Styles.TBody = t.Styles.TBody.toStyle()
	config.Styles.CodeFont = t.Styles.CodeFont.toFont()

	// Link color
	if t.Styles.LinkColor != "" {
		config.Styles.LinkColor = hexToColor(t.Styles.LinkColor)
	}
}

func (f FontDef) toStyle() *pdf.Style {
	return &pdf.Style{
		Font: pdf.Font{
			Family: f.Family,
			Size:   f.Size,
			Color:  hexToColor(f.Color),
			Style:  f.Style,
		},
	}
}

func (f FontDef) toFont() pdf.Font {
	return pdf.Font{
		Family: f.Family,
		Size:   f.Size,
		Color:  hexToColor(f.Color),
		Style:  f.Style,
	}
}

// hexToColor converts a hex color string to color.Color
func hexToColor(hex string) color.Color {
	if hex == "" {
		return color.Black
	}

	// Remove # prefix
	if len(hex) > 0 && hex[0] == '#' {
		hex = hex[1:]
	}

	// Expand shorthand (#RGB -> RRGGBB)
	if len(hex) == 3 {
		hex = string(hex[0]) + string(hex[0]) + string(hex[1]) + string(hex[1]) + string(hex[2]) + string(hex[2])
	}

	if len(hex) != 6 {
		return color.Black
	}

	r := parseHexByte(hex[0:2])
	g := parseHexByte(hex[2:4])
	b := parseHexByte(hex[4:6])

	return color.RGBA{R: r, G: g, B: b, A: 255}
}

func parseHexByte(s string) uint8 {
	var val uint8
	for _, c := range s {
		val *= 16
		if c >= '0' && c <= '9' {
			val += uint8(c - '0')
		} else if c >= 'a' && c <= 'f' {
			val += uint8(c - 'a' + 10)
		} else if c >= 'A' && c <= 'F' {
			val += uint8(c - 'A' + 10)
		}
	}
	return val
}

// =============================================================================
// THEME REGISTRY
// =============================================================================

// ThemeRegistry manages available themes
type ThemeRegistry struct {
	mu     sync.RWMutex
	themes map[string]*Theme
}

// NewThemeRegistry creates a new registry with built-in themes
func NewThemeRegistry() *ThemeRegistry {
	r := &ThemeRegistry{
		themes: make(map[string]*Theme),
	}
	r.registerBuiltinThemes()
	return r
}

// Get retrieves a theme by name
func (r *ThemeRegistry) Get(name string) *Theme {
	r.mu.RLock()
	defer r.mu.RUnlock()
	return r.themes[name]
}

// Register adds a custom theme
func (r *ThemeRegistry) Register(t *Theme) {
	r.mu.Lock()
	r.themes[t.Name] = t
	r.mu.Unlock()
}

// List returns all available theme names
func (r *ThemeRegistry) List() []string {
	r.mu.RLock()
	defer r.mu.RUnlock()
	names := make([]string, 0, len(r.themes))
	for name := range r.themes {
		names = append(names, name)
	}
	return names
}

// =============================================================================
// BUILT-IN THEMES
// =============================================================================

func (r *ThemeRegistry) registerBuiltinThemes() {
	r.Register(themeDefault())
	r.Register(themeMinimal())
	r.Register(themeAcademic())
	r.Register(themeDark())
	r.Register(themeTechnical())
	r.Register(themeModern())
}

func themeDefault() *Theme {
	return &Theme{
		Name:        "default",
		Description: "Clean, professional default theme",
		Styles: ThemeStyles{
			Normal:     FontDef{Family: "Helvetica", Size: 11, Color: "#333333"},
			H1:         FontDef{Family: "Helvetica", Size: 26, Color: "#111827", Style: "B"},
			H2:         FontDef{Family: "Helvetica", Size: 20, Color: "#1F2937", Style: "B"},
			H3:         FontDef{Family: "Helvetica", Size: 16, Color: "#374151", Style: "B"},
			H4:         FontDef{Family: "Helvetica", Size: 14, Color: "#4B5563", Style: "B"},
			H5:         FontDef{Family: "Helvetica", Size: 12, Color: "#6B7280", Style: "B"},
			H6:         FontDef{Family: "Helvetica", Size: 11, Color: "#9CA3AF", Style: "BI"},
			CodeFont:   FontDef{Family: "Courier", Size: 10, Color: "#1F2937"},
			Blockquote: FontDef{Family: "Helvetica", Size: 11, Color: "#6B7280", Style: "I"},
			LinkColor:  "#2563EB",
			THeader:    FontDef{Family: "Helvetica", Size: 10, Color: "#111827", Style: "B"},
			TBody:      FontDef{Family: "Helvetica", Size: 10, Color: "#374151"},
		},
	}
}

func themeMinimal() *Theme {
	return &Theme{
		Name:        "minimal",
		Description: "Clean minimalist theme with serif fonts",
		Styles: ThemeStyles{
			Normal:     FontDef{Family: "Times", Size: 12, Color: "#000000"},
			H1:         FontDef{Family: "Times", Size: 24, Color: "#000000", Style: "B"},
			H2:         FontDef{Family: "Times", Size: 18, Color: "#000000", Style: "B"},
			H3:         FontDef{Family: "Times", Size: 14, Color: "#000000", Style: "B"},
			H4:         FontDef{Family: "Times", Size: 12, Color: "#000000", Style: "B"},
			H5:         FontDef{Family: "Times", Size: 11, Color: "#333333", Style: "B"},
			H6:         FontDef{Family: "Times", Size: 10, Color: "#666666", Style: "I"},
			CodeFont:   FontDef{Family: "Courier", Size: 11, Color: "#000000"},
			Blockquote: FontDef{Family: "Times", Size: 11, Color: "#444444", Style: "I"},
			LinkColor:  "#000000",
			THeader:    FontDef{Family: "Times", Size: 11, Color: "#000000", Style: "B"},
			TBody:      FontDef{Family: "Times", Size: 11, Color: "#000000"},
		},
	}
}

func themeAcademic() *Theme {
	return &Theme{
		Name:        "academic",
		Description: "Academic paper style with Times New Roman",
		Styles: ThemeStyles{
			Normal:     FontDef{Family: "Times", Size: 12, Color: "#000000"},
			H1:         FontDef{Family: "Times", Size: 16, Color: "#000000", Style: "B"},
			H2:         FontDef{Family: "Times", Size: 14, Color: "#000000", Style: "B"},
			H3:         FontDef{Family: "Times", Size: 12, Color: "#000000", Style: "BI"},
			H4:         FontDef{Family: "Times", Size: 12, Color: "#000000", Style: "I"},
			H5:         FontDef{Family: "Times", Size: 11, Color: "#000000", Style: "I"},
			H6:         FontDef{Family: "Times", Size: 10, Color: "#333333", Style: "I"},
			CodeFont:   FontDef{Family: "Courier", Size: 10, Color: "#000000"},
			Blockquote: FontDef{Family: "Times", Size: 11, Color: "#333333", Style: "I"},
			LinkColor:  "#000080",
			THeader:    FontDef{Family: "Times", Size: 10, Color: "#000000", Style: "B"},
			TBody:      FontDef{Family: "Times", Size: 10, Color: "#000000"},
		},
	}
}

func themeDark() *Theme {
	return &Theme{
		Name:        "dark",
		Description: "High contrast dark mode optimized for reading",
		Styles: ThemeStyles{
			Normal:     FontDef{Family: "Helvetica", Size: 11, Color: "#E5E7EB"},
			H1:         FontDef{Family: "Helvetica", Size: 28, Color: "#F9FAFB", Style: "B"},
			H2:         FontDef{Family: "Helvetica", Size: 22, Color: "#F3F4F6", Style: "B"},
			H3:         FontDef{Family: "Helvetica", Size: 18, Color: "#E5E7EB", Style: "B"},
			H4:         FontDef{Family: "Helvetica", Size: 14, Color: "#D1D5DB", Style: "B"},
			H5:         FontDef{Family: "Helvetica", Size: 12, Color: "#9CA3AF", Style: "B"},
			H6:         FontDef{Family: "Helvetica", Size: 11, Color: "#6B7280", Style: "I"},
			CodeFont:   FontDef{Family: "Courier", Size: 10, Color: "#A5F3FC"},
			Blockquote: FontDef{Family: "Helvetica", Size: 11, Color: "#9CA3AF", Style: "I"},
			LinkColor:  "#60A5FA",
			THeader:    FontDef{Family: "Helvetica", Size: 10, Color: "#F3F4F6", Style: "B"},
			TBody:      FontDef{Family: "Helvetica", Size: 10, Color: "#D1D5DB"},
		},
	}
}

func themeTechnical() *Theme {
	return &Theme{
		Name:        "technical",
		Description: "Optimized for technical documentation",
		Styles: ThemeStyles{
			Normal:     FontDef{Family: "Helvetica", Size: 10, Color: "#1F2937"},
			H1:         FontDef{Family: "Helvetica", Size: 22, Color: "#0F172A", Style: "B"},
			H2:         FontDef{Family: "Helvetica", Size: 16, Color: "#1E293B", Style: "B"},
			H3:         FontDef{Family: "Helvetica", Size: 13, Color: "#334155", Style: "B"},
			H4:         FontDef{Family: "Helvetica", Size: 11, Color: "#475569", Style: "B"},
			H5:         FontDef{Family: "Helvetica", Size: 10, Color: "#64748B", Style: "B"},
			H6:         FontDef{Family: "Helvetica", Size: 9, Color: "#94A3B8", Style: "I"},
			CodeFont:   FontDef{Family: "Courier", Size: 9, Color: "#0F172A"},
			Blockquote: FontDef{Family: "Helvetica", Size: 10, Color: "#64748B", Style: "I"},
			LinkColor:  "#0284C7",
			THeader:    FontDef{Family: "Helvetica", Size: 9, Color: "#0F172A", Style: "B"},
			TBody:      FontDef{Family: "Courier", Size: 9, Color: "#1F2937"},
		},
	}
}

func themeModern() *Theme {
	return &Theme{
		Name:        "modern",
		Description: "Contemporary design with vibrant accents",
		Styles: ThemeStyles{
			Normal:     FontDef{Family: "Helvetica", Size: 11, Color: "#18181B"},
			H1:         FontDef{Family: "Helvetica", Size: 32, Color: "#09090B", Style: "B"},
			H2:         FontDef{Family: "Helvetica", Size: 24, Color: "#18181B", Style: "B"},
			H3:         FontDef{Family: "Helvetica", Size: 18, Color: "#27272A", Style: "B"},
			H4:         FontDef{Family: "Helvetica", Size: 14, Color: "#3F3F46", Style: "B"},
			H5:         FontDef{Family: "Helvetica", Size: 12, Color: "#52525B", Style: "B"},
			H6:         FontDef{Family: "Helvetica", Size: 10, Color: "#71717A", Style: "I"},
			CodeFont:   FontDef{Family: "Courier", Size: 10, Color: "#7C3AED"},
			Blockquote: FontDef{Family: "Helvetica", Size: 12, Color: "#52525B", Style: "I"},
			LinkColor:  "#8B5CF6",
			THeader:    FontDef{Family: "Helvetica", Size: 10, Color: "#18181B", Style: "B"},
			TBody:      FontDef{Family: "Helvetica", Size: 10, Color: "#3F3F46"},
		},
	}
}
