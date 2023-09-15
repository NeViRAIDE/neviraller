package dependencies

import (
	"fmt"
	"os"

	"github.com/RAprogramm/neviraide-install/internal/utils"
)

func Check() {
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
}
