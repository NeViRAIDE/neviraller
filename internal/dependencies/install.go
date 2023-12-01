// Package dependencies provides functions for managing and installing system dependencies
// required by the NEVIRALLER application. It includes support for package installation
// using different package managers.
package dependencies

import (
	"fmt"
	"os/exec"

	"github.com/RAprogramm/neviraide-install/internal/utils"
)

// InstallWithPacman installs a given package using the Pacman package manager.
// It prints status messages and returns true if the installation is successful.
// pkg: The name of the package to be installed.
// Returns: true if installation succeeds, false otherwise.
func InstallWithPacman(pkg string) bool {
	// Print a message indicating the start of the installation process.
	fmt.Print(utils.Color("grey", "italic", "\nInstalling %s...\n", pkg))

	// Execute the Pacman command to install the package without requiring confirmation.
	cmd := exec.Command("sudo", "pacman", "-S", "--noconfirm", pkg)
	err := cmd.Run()

	// Check for errors in executing the Pacman command.
	if err != nil {
		// Print an error message if the installation fails.
		fmt.Print(utils.Color("red", "italic", "Installation error for %s: %v\n", pkg, err))
		return false
	}

	// Print a success message if the installation completes successfully.
	fmt.Print(utils.Color("green", "italic", "%s was installed successfully!\n\n", pkg))
	return true
}
