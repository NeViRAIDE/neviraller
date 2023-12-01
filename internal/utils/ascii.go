// Package utils contains utility functions used in the NEVIRAIDE installation program.
// It includes functions for various purposes, such as displaying ASCII art, handling colors in terminal output, etc.
package utils

// ASCII returns a string containing ASCII art to be displayed in the terminal.
// This ASCII art is typically used for decorative purposes in the command-line interface.
func ASCII() string {
	// Defining the ASCII art as a multi-line string.
	asciiArt := `
███╗   ██╗███████╗██╗   ██╗██╗██████╗  █████╗ ██╗     ██╗     ███████╗██████╗ 
████╗  ██║██╔════╝██║   ██║██║██╔══██╗██╔══██╗██║     ██║     ██╔════╝██╔══██╗
██╔██╗ ██║█████╗  ██║   ██║██║██████╔╝███████║██║     ██║     █████╗  ██████╔╝
██║╚██╗██║██╔══╝  ╚██╗ ██╔╝██║██╔══██╗██╔══██║██║     ██║     ██╔══╝  ██╔══██╗
██║ ╚████║███████╗ ╚████╔╝ ██║██║  ██║██║  ██║███████╗███████╗███████╗██║  ██║
╚═╝  ╚═══╝╚══════╝  ╚═══╝  ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝╚═╝  ╚═╝
                                                                                        `

	// Returning the ASCII art.
	return asciiArt
}
