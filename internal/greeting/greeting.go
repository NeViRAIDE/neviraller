package greeting

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/RAprogramm/neviraide-install/internal/neovim_nightly"
)

func Greeting() bool {
	reader := bufio.NewReader(os.Stdin)

	fmt.Println("Welcome to the NEVIRAIDE installer!")
	fmt.Println("This script will check for required dependencies and install them if they're missing.")
	fmt.Println("It will also set up the NEVIRAIDE configuration for Neovim.")

	fmt.Print("Would you like to install the latest pre-release of Neovim Nightly? [y/n]: ")
	answer1, err := reader.ReadString('\n')
	if err != nil {
		fmt.Printf("Error reading input: %v\n", err)
		os.Exit(1)
	}
	answer1 = strings.TrimSpace(answer1)
	if answer1 == "y" {
		neovimnightly.InstallNeovim()
	}

	fmt.Print("Would you like to continue with the installation? [y/n]: ")
	answer2, err := reader.ReadString('\n')
	if err != nil {
		fmt.Printf("Error reading input: %v\n", err)
		os.Exit(1)
	}
	answer2 = strings.TrimSpace(answer2)
	if answer2 != "y" {
		fmt.Println("Installation canceled by the user.")
		return false
	}
	return true
}
