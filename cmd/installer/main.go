package main

import (
	"bufio"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"

	"github.com/RAprogramm/neviraide-install/internal/greeting"
	"github.com/RAprogramm/neviraide-install/internal/utils"
)

func main() {
	if !greeting.Greeting() {
		return
	}

	utils.RequestSudo()

	if !utils.CheckSudo() {
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

	missingDeps := utils.CheckCommandsAvailability(dependencies)
	for _, dep := range missingDeps {
		success := utils.InstallWithPacman(dep)
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
