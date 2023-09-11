package main

import (
	"bufio"
	"fmt"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

func greeting() {
	fmt.Println("Welcome to the NEVIRAIDE installer!")
	fmt.Println("This script will check for required dependencies and install them if they're missing.")
	fmt.Println("It will also set up the NEVIRAIDE configuration for Neovim.")
	fmt.Println("")

	reader := bufio.NewReader(os.Stdin)
	fmt.Print("Would you like to continue with the installation? [y/n]: ")
	answer, _ := reader.ReadString('\n')
	answer = strings.TrimSpace(answer)
	if answer != "y" {
		fmt.Println("Installation canceled by the user.")
		os.Exit(0)
	}
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

func installWithPacman(pkg string) {
	cmd := exec.Command("sudo", "pacman", "-S", "--noconfirm", pkg)
	if err := cmd.Run(); err != nil {
		slog.Error("Installation error %s: %v", pkg, err)
	}
}

func main() {
	greeting()

	dependencies := map[string]string{
		"neovim":  "nvim",
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
		fmt.Printf("%s not found. Installing %s...\n", dep, dep)
		installWithPacman(dep)
	}

	repoURL := "https://github.com/RAprogramm/NEVIRAIDE.git"
	cloneDir := "/tmp/neovim-config"

	_, err := exec.Command("git", "clone", repoURL, cloneDir).Output()
	if err != nil {
		slog.Error("Cloning repository error: %v", err)
	}

	homeDir, err := os.UserHomeDir()
	if err != nil {
		slog.Error("Error getting home directory: %v", err)
		return
	}
	configDir := filepath.Join(homeDir, ".config/nvim")

	if _, err = os.Stat(configDir); !os.IsNotExist(err) {
		reader := bufio.NewReader(os.Stdin)
		fmt.Print("~/.config/nvim already exist. Remove it? [y/n]: ")
		answer, _ := reader.ReadString('\n')
		answer = strings.TrimSpace(answer)

		switch answer {
		case "y":
			err = os.RemoveAll(configDir)
			if err != nil {
				slog.Error("Error removing directory: %v", err)
			}
		case "n":
			err = os.Rename(configDir, configDir+".old")
			if err != nil {
				slog.Error("Error renaming directory: %v", err)
			}
		default:
			slog.Warn("Undefined choice. Abort installation.")
		}
	}

	err = exec.Command("cp", "-r", cloneDir, configDir).Run()
	if err != nil {
		slog.Error("Error copying configuration files: %v", err)
	}

	fmt.Println("NEVIRAIDE was successfully installed!")
}
