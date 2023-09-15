package main

import (
	"bufio"
	"fmt"
	"github.com/RAprogramm/neviraide-install/internal/neovim_nightly"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

func requestSudo() {
	cmd := exec.Command("sudo", "-v")
	if err := cmd.Run(); err != nil {
		fmt.Println("Failed to obtain sudo privileges. Please ensure you have the correct permissions.")
		os.Exit(1)
	}
}

func greeting() bool {
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

func checkCommandsAvailability(names map[string]string) []string {
	missing := []string{}
	for name, cmd := range names {
		cmd := exec.Command("command", "-v", cmd)
		if err := cmd.Run(); err != nil {
			missing = append(missing, name)
		}
	}
	return missing
}

func checkSudo() bool {
	cmd := exec.Command("command", "-v", "sudo")
	return cmd.Run() == nil
}

func installWithPacman(pkg string) bool {
	fmt.Printf("Installing %s...\n", pkg)
	cmd := exec.Command("sudo", "pacman", "-S", "--noconfirm", pkg)
	err := cmd.Run()
	if err != nil {
		fmt.Printf("Installation error for %s: %v\n", pkg, err)
		return false
	}
	fmt.Printf("%s was installed successfully!\n", pkg)
	return true
}

func main() {
	if !greeting() {
		return
	}

	requestSudo()

	if !checkSudo() {
		fmt.Println("sudo is not available or you don't have sudo privileges.")
		os.Exit(1)
	}

	dependencies := map[string]string{
		"git":     "git",
		"ripgrep": "rg",
		"fd":      "fd",
		"unzip":   "unzip",
		"tar":     "tar",
		"wget":    "wget",
		"curl":    "curl",
		"npm":     "npm",
	}

	missingDeps := checkCommandsAvailability(dependencies)
	for _, dep := range missingDeps {
		success := installWithPacman(dep)
		if !success {
			fmt.Printf("Failed to install %s. Aborting installation.\n", dep)
			os.Exit(1)
		}
	}

	repoURL := "https://github.com/RAprogramm/NEVIRAIDE.git"
	cloneDir := "/tmp/neovim-config"

	fmt.Println("Cloning NEVIRAIDE repository...")
	_, err := exec.Command("git", "clone", "--depth", "1", repoURL, cloneDir).Output()
	if err != nil {
		fmt.Printf("Cloning repository error: %v\n", err)
		return
	}
	fmt.Println("Repository cloned successfully!")

	homeDir, err := os.UserHomeDir()
	if err != nil {
		fmt.Printf("Error getting home directory: %v\n", err)
		return
	}
	configDir := filepath.Join(homeDir, ".config/nvim")

	if _, err = os.Stat(configDir); !os.IsNotExist(err) {
		reader := bufio.NewReader(os.Stdin)
		fmt.Print("~/.config/nvim already exists. Remove it? [y/n]: ")
		answer, err1 := reader.ReadString('\n')
		if err1 != nil {
			fmt.Printf("Error reading input: %v\n", err1)
			os.Exit(1)
		}
		answer = strings.TrimSpace(answer)

		switch answer {
		case "y":
			err = os.RemoveAll(configDir)
			if err != nil {
				fmt.Printf("Error removing directory: %v\n", err)
				os.Exit(1)
			}
		case "n":
			err = os.Rename(configDir, configDir+".old")
			if err != nil {
				fmt.Printf("Error renaming directory: %v\n", err)
				os.Exit(1)
			}
		default:
			fmt.Println("Undefined choice. Abort installation.")
			os.Exit(0)
		}
	}

	err = exec.Command("cp", "-r", cloneDir, configDir).Run()
	if err != nil {
		fmt.Printf("Error copying configuration files: %v\n", err)
		return
	}

	os.RemoveAll(cloneDir)

	fmt.Println("NEVIRAIDE was successfully installed!")
}
