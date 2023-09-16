package utils

import (
	"fmt"
	"os"
	"os/exec"
)

func RequestSudo() {
	cmd := exec.Command("sudo", "-v")
	if err := cmd.Run(); err != nil {
		fmt.Println("Failed to obtain sudo privileges. Please ensure you have the correct permissions.")
		os.Exit(1)
	}
}

func CheckSudo() bool {
	cmd := exec.Command("command", "-v", "sudo")
	return cmd.Run() == nil
}

func InstallWithPacman(pkg string) bool {
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
