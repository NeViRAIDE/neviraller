package ui

import (
	"fmt"
	"os"

	"github.com/RAprogramm/neviraide-install/internal/dependencies"
	"github.com/manifoldco/promptui"

	neovim "github.com/RAprogramm/neviraide-install/internal/neovim_nightly"
)

type option struct {
	Number int
	Text   string
	Desc   string
}

var templates = &promptui.SelectTemplates{
	Label:    "{{ . }}?",
	Active:   "{{ .Number | green }}. {{ .Text | green }} ({{ .Desc | cyan }})",
	Inactive: "{{ .Number | white }}. {{ .Text | white }}",
	Selected: "{{ .Text | red | cyan }}",
}

func Start(install_func func()) {
Loop:
	for {
		options := []option{
			{1, "Check", "Check all dependencies"},
			{2, "Update", "Check latest release Neovim Nighly and install it"},
			{3, "Install", "Install NEVIRAIDE"},
			{4, "Update and install", "Install Neovim and NEVIRAIDE"},
			{5, "Exit", "Quit from installation program"},
		}

		prompt := promptui.Select{
			Label:     "What would you like",
			Items:     options,
			Templates: templates,
		}

		i, _, err := prompt.Run()
		if err != nil {
			fmt.Printf("Prompt failed %v\n", err)
			return
		}
		switch options[i].Number {
		case 1:
			missingCount := dependencies.Check()
			if missingCount > 0 {
				if confirm("Would you like to install missing dependencies") {
					break Loop
				}
			}
		case 2:
			neovim.InstallNeovim()
		case 3:
			install_func()
		case 4:
			neovim.InstallNeovim()
			install_func()
		case 5:
			if confirm("Exit from intallation") {
				break Loop
			}
		}
	}
}

func confirm(text string) bool {
	prompt := promptui.Prompt{
		Label:     text,
		IsConfirm: true,
	}
	_, err := prompt.Run()
	if err != nil {
		if err == promptui.ErrAbort {
			return false
		}
		fmt.Printf("Prompt failed %v\n", err)
		return false
	}
	return true
}

func ExistDir(configDir string) {
Loop:
	for {
		options := []option{
			{1, "Rename", "Rename existing \"nvim\" directory to \"nvim.old\""},
			{2, "Remove", "Remove existing \"nvim\" directory"},
			{3, "Exit", "Abort installation"},
		}

		prompt := promptui.Select{
			Label:     "~/.config/nvim already exists. What should to do",
			Items:     options,
			Templates: templates,
		}

		if _, err := os.Stat(configDir); !os.IsNotExist(err) {
			i, _, err := prompt.Run()
			if err != nil {
				fmt.Printf("Prompt failed %v\n", err)
				return
			}

			switch options[i].Number {
			case 1:
				err = os.Rename(configDir, configDir+".old")
				if err != nil {
					fmt.Printf("Error renaming directory: %v\n", err)
					os.Exit(1)
				}
			case 2:
				err = os.RemoveAll(configDir)
				if err != nil {
					fmt.Printf("Error removing directory: %v\n", err)
					os.Exit(1)
				}
			case 3:
				fmt.Println("Abort installation.")
				break Loop
			}
		}
	}
}
