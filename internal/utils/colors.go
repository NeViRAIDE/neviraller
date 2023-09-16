package utils

import "fmt"

const (
	ResetItalic  = "\x1b[23m"
	ResetBold    = "\x1b[22m"
	ColorDefault = "\x1b[39m"
)

var colors = map[string]string{
	"red":   "\x1b[91m",
	"green": "\x1b[32m",
	"blue":  "\x1b[94m",
	"grey":  "\x1b[90m",
}

var styles = map[string]string{
	"bold":   "\x1b[1m",
	"italic": "\x1b[3m",
}

func Color(color, style, text string, args ...interface{}) string {
	var res string
	
	if val, ok := colors[color]; ok {
		res += val
	}
	
	if val, ok := styles[style]; ok {
		res += val
	}

	return fmt.Sprintf(res+text+ResetBold+ResetItalic+ColorDefault, args...)
}
