package main

import (
	"fmt"

	"github.com/RAprogramm/neviraide-install/internal/neviraide"
	"github.com/RAprogramm/neviraide-install/internal/ui"
	"github.com/RAprogramm/neviraide-install/internal/utils"
)

func main() {
	fmt.Println("")
	fmt.Println(utils.Color("green", "", utils.ASCII()))
	fmt.Println(utils.Color("green", "bold", "\t\t\tWelcome to the NEVIRALLER!"))
	fmt.Println(utils.Color("grey", "italic", "\tThis is 2 in 1 - Neovim Nightly Updater and NEVIRAIDE installer!"))
	fmt.Println(utils.Color("grey", "italic", "\tBefore installation you should to check all needed dependencies."))
	fmt.Println("")

	ui.Start(neviraide.Install)
}
