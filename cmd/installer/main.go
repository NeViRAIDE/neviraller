package main

import (
	"fmt"

	"github.com/RAprogramm/neviraide-install/internal/neviraide"
	"github.com/RAprogramm/neviraide-install/internal/ui"
)

func main() {
	fmt.Println("Welcome to the NEVIRAIDE installer!")
	fmt.Println("")
	ui.Main(neviraide.Install)
}
