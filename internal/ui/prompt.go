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
	Inactive: "{{ .Number | white }}. {{ .Text | white }} ({{ .Desc | white }})",
	Selected: "{{ .Text | red | cyan }}",
}

func Main(install_func func()) {
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
		dependencies.Check()
	case 2:
		neovim.InstallNeovim()
	case 3:
		install_func()
	case 4:
		neovim.InstallNeovim()
		install_func()
	case 5:
		return
	}
}

func ExistDir(configDir string) {
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
			os.Exit(0)
		}
	}
}
