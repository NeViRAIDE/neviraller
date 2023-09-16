package utils

import (
	"fmt"
	"os"
	"os/exec"

	"github.com/manifoldco/promptui"
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

func Confirm(text string) bool {
	prompt := promptui.Prompt{
		Label:     text,
		IsConfirm: true,
	}
	_, err := prompt.Run()
	if err != nil {
		if err == promptui.ErrAbort {
			return false
		}
		fmt.Print(Color("red", "italic", "Prompt failed %v\n", err))
		return false
	}
	return true
}
