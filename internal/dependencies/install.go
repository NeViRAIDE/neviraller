package dependencies

import (
	"fmt"
	"os/exec"
)

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
