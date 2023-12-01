// Package neviraide provides the functionality to install the NEVIRAIDE configuration
// for Neovim. It includes the process of cloning the repository and setting up the
// configuration in the user's environment.
package neviraide

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"

	// Internal packages for user interface and utility functions.
	"github.com/RAprogramm/neviraide-install/internal/ui"
	"github.com/RAprogramm/neviraide-install/internal/utils"
)

// Constants for the NEVIRAIDE repository URL and the temporary directory for cloning.
const (
	repoURL  = "https://github.com/RAprogramm/NEVIRAIDE.git"
	cloneDir = "/tmp/neovim-config"
)

// Install handles the installation of the NEVIRAIDE configuration.
func Install() {
	// Check if the clone directory already exists and remove it if it does.
	if _, err := os.Stat(cloneDir); !os.IsNotExist(err) {
		os.RemoveAll(cloneDir)
	}

	// Inform the user that the cloning process is starting.
	fmt.Println(utils.Color("grey", "italic", "Cloning NEVIRAIDE repository..."))

	// Execute the git clone command to clone the repository.
	_, err := exec.Command("git", "clone", "--depth", "1", repoURL, cloneDir).Output()
	if err != nil {
		// If cloning fails, display an error and return.
		fmt.Print(utils.Color("red", "italic", "Cloning repository error: %v\n", err))
		return
	}
	// Inform the user that the repository was successfully cloned.
	fmt.Println(utils.Color("grey", "italic", "Repository cloned successfully!"))

	// Get the user's home directory.
	homeDir, err := os.UserHomeDir()
	if err != nil {
		// If fetching the home directory fails, display an error and return.
		fmt.Print(utils.Color("red", "italic", "Error getting home directory: %v\n", err))
		return
	}
	// Define the path to the Neovim configuration directory.
	configDir := filepath.Join(homeDir, ".config/nvim")

	// Check if the Neovim configuration directory already exists.
	ui.ExistDir(configDir)

	// Copy the cloned NEVIRAIDE configuration to the Neovim configuration directory.
	err = exec.Command("cp", "-r", cloneDir, configDir).Run()
	if err != nil {
		// If copying fails, display an error and return.
		fmt.Print(utils.Color("red", "italic", "Error copying configuration files: %v\n", err))
		return
	}

	// Remove the temporary clone directory after installation.
	os.RemoveAll(cloneDir)

	// Inform the user that NEVIRAIDE was successfully installed.
	fmt.Println(utils.Color("green", "italic", "NEVIRAIDE was successfully installed!"))
}
