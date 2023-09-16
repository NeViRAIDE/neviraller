package utils

import "fmt"

const (
	ColorDefault = "\x1b[39m"
	ColorRed     = "\x1b[91m"
	ColorGreen   = "\x1b[32m"
	ColorBlue    = "\x1b[94m"
	ColorGray    = "\x1b[90m"
)

func Red(s string, args ...interface{}) string {
	return fmt.Sprintf(ColorRed+s+ColorDefault, args...)
}
func Green(s string, args ...interface{}) string {
	return fmt.Sprintf(ColorGreen+s+ColorDefault, args...)
}
func Blue(s string, args ...interface{}) string {
	return fmt.Sprintf(ColorBlue+s+ColorDefault, args...)
}
func Gray(s string, args ...interface{}) string {
	return fmt.Sprintf(ColorGray+s+ColorDefault, args...)
}
