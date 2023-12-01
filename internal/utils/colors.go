// Package utils contains utility functions used in the NEVIRAIDE installation program,
// including those for text formatting in terminal outputs.
package utils

import "fmt"

// ANSI escape code constants for resetting text styles and setting default color.
const (
	ResetItalic  = "\x1b[23m"
	ResetBold    = "\x1b[22m"
	ColorDefault = "\x1b[39m"
)

// colors map defines ANSI escape codes for various text colors.
var colors = map[string]string{
	"red":   "\x1b[91m",
	"green": "\x1b[32m",
	"blue":  "\x1b[94m",
	"grey":  "\x1b[90m",
}

// styles map defines ANSI escape codes for text styles like bold and italic.
var styles = map[string]string{
	"bold":   "\x1b[1m",
	"italic": "\x1b[3m",
}

// Color formats the given text with the specified color and style.
// color: The text color to be applied.
// style: The style (e.g., bold, italic) to be applied.
// text: The text string to be formatted.
// args: Additional arguments for formatting, similar to fmt.Sprintf.
// Returns: A formatted string with applied color and style.
func Color(color, style, text string, args ...interface{}) string {
	var res string

	// Applying the color if it's defined in the colors map.
	if val, ok := colors[color]; ok {
		res += val
	}

	// Applying the style if it's defined in the styles map.
	if val, ok := styles[style]; ok {
		res += val
	}

	// Formatting the string with the specified text, color, and style.
	// Also, resetting the style and color to default at the end.
	return fmt.Sprintf(res+text+ResetBold+ResetItalic+ColorDefault, args...)
}
