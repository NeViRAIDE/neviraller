// Package utils contains utility functions used across the neviraide-install project.
// It includes functions for requesting sudo privileges, checking for sudo availability,
// and handling user confirmations through prompts.
package utils

import (
	"fmt"
	"os"
	"os/exec"

	"github.com/manifoldco/promptui"
)

// RequestSudo attempts to obtain sudo privileges by running the 'sudo -v' command.
// It exits the program with a status code of 1 if sudo privileges cannot be obtained.
func RequestSudo() {
	// Execute the 'sudo -v' command to verify or refresh sudo privileges.
	cmd := exec.Command("sudo", "-v")
	if err := cmd.Run(); err != nil {
		// Print an error message and exit if sudo privileges cannot be obtained.
		fmt.Println(
			"Failed to obtain sudo privileges. Please ensure you have the correct permissions.",
		)
		os.Exit(1)
	}
}

// CheckSudo checks if the sudo command is available on the system.
// It returns true if sudo is available, and false otherwise.
func CheckSudo() bool {
	// Execute the 'command -v sudo' to check for the presence of the sudo command.
	cmd := exec.Command("command", "-v", "sudo")
	// Return true if the command executes successfully, indicating sudo is available.
	return cmd.Run() == nil
}

// Confirm displays a confirmation prompt to the user with the provided text.
// It returns true if the user confirms, and false if the user aborts or an error occurs.
func Confirm(text string) bool {
	// Set up a confirmation prompt using promptui.
	prompt := promptui.Prompt{
		Label:     text,
		IsConfirm: true,
	}
	// Run the prompt and wait for user input.
	_, err := prompt.Run()
	if err != nil {
		// Handle the case where the user aborts the prompt.
		if err == promptui.ErrAbort {
			return false
		}
		// Print an error message if the prompt fails for any other reason.
		fmt.Print(Color("red", "italic", "Prompt failed %v\n", err))
		return false
	}
	// Return true if the user confirms.
	return true
}
