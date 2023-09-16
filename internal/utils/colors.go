package utils

import "fmt"

const (
	ColorDefault = "\x1b[39m"
	ColorRed     = "\x1b[91m"
	ColorGreen   = "\x1b[32m"
	ColorBlue    = "\x1b[94m"
	ColorGray    = "\x1b[90m"
)

func Red(s string, arg ...interface{}) string {
	return fmt.Sprintf(ColorRed+s+ColorDefault, arg...)
}
func Green(s string, arg ...interface{}) string {
	return fmt.Sprintf(ColorGreen+s+ColorDefault, arg...)
}
func Blue(s string, arg ...interface{}) string {
	return fmt.Sprintf(ColorBlue+s+ColorDefault, arg...)
}
func Gray(s string, arg ...interface{}) string {
	return fmt.Sprintf(ColorGray+s+ColorDefault, arg...)
}
