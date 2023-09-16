package dependencies

import (
	"fmt"
	"os/exec"

	"github.com/RAprogramm/neviraide-install/internal/utils"
)

func InstallWithPacman(pkg string) bool {
	fmt.Print(utils.Color("grey", "italic", "\nInstalling %s...\n", pkg))
	cmd := exec.Command("sudo", "pacman", "-S", "--noconfirm", pkg)
	err := cmd.Run()
	if err != nil {
		fmt.Print(utils.Color("red", "italic", "Installation error for %s: %v\n", pkg, err))
		return false
	}
	fmt.Print(utils.Color("green", "italic", "%s was installed successfully!\n\n", pkg))
	return true
}
