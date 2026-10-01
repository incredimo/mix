package converter

import (
	"path/filepath"
	"strings"
)

// =============================================================================
// TEXT UTILITIES
// =============================================================================

// cleanText removes HTML entities that may have slipped through
func cleanText(s string) string {
	replacer := strings.NewReplacer(
		"&quot;", `"`,
		"&amp;", "&",
		"&lt;", "<",
		"&gt;", ">",
		"&apos;", "'",
		"&#39;", "'",
		"&nbsp;", " ",
		"&copy;", "\u00A9",
		"&reg;", "\u00AE",
		"&trade;", "\u2122",
		"&mdash;", "\u2014",
		"&ndash;", "\u2013",
		"&hellip;", "\u2026",
		"&lsquo;", "\u2018",
		"&rsquo;", "\u2019",
		"&ldquo;", "\u201C",
		"&rdquo;", "\u201D",
		"&bull;", "\u2022",
		"&middot;", "\u00B7",
		"&deg;", "\u00B0",
		"&plusmn;", "\u00B1",
		"&times;", "\u00D7",
		"&divide;", "\u00F7",
		"&frac12;", "\u00BD",
		"&frac14;", "\u00BC",
		"&frac34;", "\u00BE",
	)
	return replacer.Replace(s)
}

// =============================================================================
// PATH UTILITIES
// =============================================================================

// isMarkdownFile checks if a file has a markdown extension
func isMarkdownFile(path string) bool {
	ext := strings.ToLower(filepath.Ext(path))
	switch ext {
	case ".md", ".markdown", ".mdown", ".mkd", ".mkdn", ".mdwn", ".mdtxt", ".mdtext":
		return true
	default:
		return false
	}
}

// isCSSFile checks if a file has a CSS extension
func isCSSFile(path string) bool {
	ext := strings.ToLower(filepath.Ext(path))
	return ext == ".css"
}

// changeExtension replaces a file's extension
func changeExtension(path, newExt string) string {
	ext := filepath.Ext(path)
	return strings.TrimSuffix(path, ext) + newExt
}

// =============================================================================
// STRING UTILITIES
// =============================================================================

// truncate limits string length with ellipsis
func truncate(s string, maxLen int) string {
	if len(s) <= maxLen {
		return s
	}
	if maxLen <= 3 {
		return s[:maxLen]
	}
	return s[:maxLen-3] + "..."
}

// containsAny checks if string contains any of the given substrings
func containsAny(s string, substrs ...string) bool {
	lower := strings.ToLower(s)
	for _, sub := range substrs {
		if strings.Contains(lower, strings.ToLower(sub)) {
			return true
		}
	}
	return false
}

// startsWithAny checks if string starts with any of the given prefixes
func startsWithAny(s string, prefixes ...string) bool {
	for _, prefix := range prefixes {
		if strings.HasPrefix(s, prefix) {
			return true
		}
	}
	return false
}

// =============================================================================
// SLICE UTILITIES
// =============================================================================

// stringInSlice checks if a string is in a slice
func stringInSlice(s string, slice []string) bool {
	for _, item := range slice {
		if item == s {
			return true
		}
	}
	return false
}

// uniqueStrings removes duplicates from a string slice
func uniqueStrings(slice []string) []string {
	seen := make(map[string]struct{}, len(slice))
	result := make([]string, 0, len(slice))
	for _, s := range slice {
		if _, ok := seen[s]; !ok {
			seen[s] = struct{}{}
			result = append(result, s)
		}
	}
	return result
}
