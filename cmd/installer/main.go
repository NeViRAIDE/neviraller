// Package main is the entry point of the NEVIRALLER application.
// This application serves as a 2-in-1 tool for updating Neovim Nightly
// and installing NEVIRAIDE. It utilizes internal packages for managing
// the user interface, utility functions, and the core installation logic.
package main

import (
	"fmt"

	"github.com/RAprogramm/neviraide-install/internal/neviraide"
	"github.com/RAprogramm/neviraide-install/internal/ui"
	"github.com/RAprogramm/neviraide-install/internal/utils"
)

func main() {
	// Print an empty line for better formatting in the console.
	fmt.Println("")

	// Display an ASCII art (presumably of the application logo) in green color.
	fmt.Println(utils.Color("green", "", utils.ASCII()))

	// Print the welcome message for NEVIRALLER in bold green.
	fmt.Println(utils.Color("green", "bold", "\t\t\tWelcome to the NEVIRALLER!"))

	// Print the application description in grey italic.
	fmt.Println(
		utils.Color(
			"grey",
			"italic",
			"\tThis is 2 in 1 - Neovim Nightly Updater and NEVIRAIDE installer!",
		),
	)
	fmt.Println(
		utils.Color(
			"grey",
			"italic",
			"\tBefore installation you should to check all needed dependencies.",
		),
	)

	// Print another empty line for spacing.
	fmt.Println("")

	// Start the user interface for the NEVIRAIDE installation process.
	// The `Install` function from the `neviraide` package is passed as an argument,
	// which suggests that it contains the core logic for the installation process.
	ui.Start(neviraide.Install)
}
