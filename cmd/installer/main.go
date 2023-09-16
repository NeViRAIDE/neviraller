package main

import (
	"fmt"

	"github.com/RAprogramm/neviraide-install/internal/neviraide"
	"github.com/RAprogramm/neviraide-install/internal/ui"

)

func main() {
	fmt.Printf("Welcome to the NEVIRAIDE installer!\n\n")
	ui.Start(neviraide.Install)
}
