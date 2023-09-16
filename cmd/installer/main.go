package main

import (
	"fmt"

	"github.com/RAprogramm/neviraide-install/internal/neviraide"
	"github.com/RAprogramm/neviraide-install/internal/ui"
	"github.com/RAprogramm/neviraide-install/internal/utils"
)

func main() {
    fmt.Println(utils.Color("green", "bold", "Welcome to the NEVIRAIDE installer!\n"))

	ui.Start(neviraide.Install)
}
