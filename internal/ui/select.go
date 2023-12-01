// Package ui manages the user interface for the NEVIRAIDE installation program.
// It uses the promptui package to create an interactive CLI for users to select
// installation options, handle existing configurations, and manage dependencies.
package ui

import (
	"fmt"
	"os"

	// Importing internal packages for managing dependencies and utility functions.
	"github.com/RAprogramm/neviraide-install/internal/dependencies"
	"github.com/RAprogramm/neviraide-install/internal/utils"
	"github.com/manifoldco/promptui"

	// Importing the neovim package for handling Neovim Nightly installation.
	neovim "github.com/RAprogramm/neviraide-install/internal/neovim_nightly"
)

// option struct defines an individual option in the menu.
type option struct {
	Number int    // Number is the option number in the menu.
	Text   string // Text is the display text for the option.
	Desc   string // Desc provides a short description of the option.
}

// templates define the display format for the options in the prompt.
var templates = &promptui.SelectTemplates{
	Label:    "{{ . }}?",
	Active:   "{{ .Number | green }}. {{ .Text | green }} ({{ .Desc | cyan }})",
	Inactive: "{{ .Number | white }}. {{ .Text | white }}",
	Selected: "{{ .Text | red | cyan }}",
}

// Start initiates the main loop of the installation program.
// install_func: A function to be called for NEVIRAIDE installation.
func Start(installFunc func()) {
Loop:
	for {
		// Defining various options for the installation program.
		options := []option{
			{1, "Check", "Check all dependencies"},
			{2, "Update", "Check latest release Neovim Nighly and install it"},
			{3, "Install", "Install NEVIRAIDE"},
			{4, "Update and install", "Install Neovim and NEVIRAIDE"},
			{5, "Exit", "Quit from installation program"},
		}

		// Creating a new prompt with the defined options.
		prompt := promptui.Select{
			Label:     "What would you like",
			Items:     options,
			Templates: templates,
		}

		// Running the prompt and handling user input.
		i, _, err := prompt.Run()
		if err != nil {
			fmt.Print(utils.Color("red", "italic", "Prompt failed %v\n", err))
			return
		}

		// Switch case to handle different options selected by the user.
		switch options[i].Number {
		case 1:
			// Code for handling dependency checks and installation...
			missingDeps := dependencies.Check()
			missingCount := 0
			for _, dep := range missingDeps {
				if !dep.Exist {
					missingCount++
				}
			}
			if missingCount > 0 {
				if utils.Confirm("Would you like to install missing dependencies") {
					for _, dep := range missingDeps {
						if !dep.Exist {
							dependencies.InstallWithPacman(dep.Name)
						}
					}
				}
			}
		case 2:
			// Code for updating Neovim Nightly...
			neovim.InstallNeovim()
		case 3:
			// Code for installing NEVIRAIDE...
			installFunc()
		case 4:
			// Code for updating Neovim Nightly and installing NEVIRAIDE...
			neovim.InstallNeovim()
			installFunc()
		case 5:
			// Code for exiting the installation program...
			if utils.Confirm("Exit from intallation") {
				break Loop
			}
		}
	}
}

// ExistDir checks if a given directory exists and provides options to handle it.
// configDir: The directory path to check.
func ExistDir(configDir string) {
Loop:
	for {
		// Defining options for handling existing configurations.
		options := []option{
			{1, "Rename", "Rename existing \"nvim\" directory to \"nvim.old\""},
			{2, "Remove", "Remove existing \"nvim\" directory"},
			{3, "Exit", "Abort installation"},
		}

		// Creating a new prompt for handling existing configurations.
		prompt := promptui.Select{
			Label:     "~/.config/nvim already exists. What should to do",
			Items:     options,
			Templates: templates,
		}

		// Checking if the configuration directory exists.
		if _, err := os.Stat(configDir); !os.IsNotExist(err) {
			// Running the prompt and handling user input.
			i, _, err := prompt.Run()
			if err != nil {
				fmt.Print(utils.Color("red", "italic", "Prompt failed %v\n", err))
				return
			}

			// Switch case to handle different options selected by the user.
			switch options[i].Number {
			// Code for handling each option...
			case 1:
				err = os.Rename(configDir, configDir+".old")
				if err != nil {
					fmt.Print(utils.Color("red", "italic", "Error renaming directory %v\n", err))
					os.Exit(1)
				}
			case 2:
				err = os.RemoveAll(configDir)
				if err != nil {
					fmt.Print(utils.Color("red", "italic", "Error removing directory %v\n", err))
					os.Exit(1)
				}
			case 3:
				fmt.Println(utils.Color("grey", "italic", "Abort installation..."))
				break Loop
			}
		}
	}
}
