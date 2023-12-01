// Package neovimnightly manages the installation and updating of Neovim Nightly.
// It checks for necessary commands, scrapes the latest version of Neovim, and handles the installation process.
package neovimnightly

import (
	// Importing necessary packages.
	"context"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"os/exec"
	"strings"
	"time"

	// Internal packages for scraping and utility functions.
	"github.com/RAprogramm/neviraide-install/internal/scrap"
	"github.com/RAprogramm/neviraide-install/internal/utils"
)

// nvimDownloadURL is the URL to download the Neovim Nightly version.
const nvimDownloadURL = "https://github.com/neovim/neovim/releases/download/nightly/nvim.appimage"

// checkCommandExists checks if a command is available on the system.
func checkCommandExists(command string) error {
	// Using exec.LookPath to check for the command.
	_, err := exec.LookPath(command)
	if err != nil {
		// Returning an error if the command is not found.
		return fmt.Errorf("command %s not found: %v", command, err)
	}
	return nil
}

// InstallNeovim handles the installation or updating of Neovim Nightly.
func InstallNeovim() {
	// Checking for the existence of required commands.
	if err := checkCommandExists("wget"); err != nil {
		log.Fatal(err)
	}
	if err := checkCommandExists("curl"); err != nil {
		log.Fatal(err)
	}
	if err := checkCommandExists("xmllint"); err != nil {
		log.Fatal(err)
	}
	if err := checkCommandExists("datediff"); err != nil {
		log.Fatal(err)
	}

	// Setting a timeout context for scraping operations.
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	// Scraping the latest Neovim version from the web.
	newNvimVer, err := scrap.GetNeovimVersionFromMetaTag(ctx)
	if err != nil {
		log.Fatalf("Error getting Neovim version: %v", err)
	}

	// Getting the current Neovim version installed on the system.
	cmd := exec.Command("nvim", "--version")
	out, err := cmd.CombinedOutput()
	if err != nil {
		log.Fatalf("Error getting current Neovim version: %v", err)
	}

	// Extracting the current Neovim version from the output.
	currNvimVer := strings.Split(string(out), "\n")[0]

	// Checking if the current version matches the new version.
	if currNvimVer == newNvimVer {
		fmt.Print(
			utils.Color("green", "", "\nYou already have the latest version: %s\n", currNvimVer),
		)
		fmt.Println(utils.Color("red", "bold", "NOT FOUND new version of Neovim Nightly!"))
		fmt.Println("")
		return
	}

	// Notifying about the new version found.
	fmt.Print(utils.Color("blue", "italic", "\nNew Neovim Nightly version found!\n"))
	fmt.Print(utils.Color("grey", "italic", "%s -> %s\n\n", currNvimVer, newNvimVer))

	// Confirming with the user to proceed with the update.
	if utils.Confirm("Do you wish to update neovim") {
		// Downloading the new version of Neovim.
		resp, err := http.Get(nvimDownloadURL)
		if err != nil {
			log.Fatalf("Error downloading Neovim Nightly: %v", err)
		}
		defer resp.Body.Close()

		// Creating a temporary file for the downloaded content.
		out, err := os.Create("/tmp/nvim")
		if err != nil {
			log.Fatalf("Error creating temp file: %v", err)
		}
		defer out.Close()

		// Copying the downloaded content to the temporary file.
		_, err = io.Copy(out, resp.Body)
		if err != nil {
			log.Fatalf("Error writing to temp file: %v", err)
		}

		// Terminating any running instances of Neovim.
		cmd = exec.Command("pkill", "nvim")
		cmd.Run()

		// Copying the downloaded Neovim to the bin directories.
		cmd = exec.Command("sudo", "cp", "/tmp/nvim", "/usr/local/bin")
		cmd.Run()

		cmd = exec.Command("sudo", "mv", "/tmp/nvim", "/usr/bin")
		cmd.Run()

		// Indicating successful update.
		fmt.Print(
			utils.Color("green", "italic", "\nNeovim Nightly has been updated successfully!\n"),
		)
	}
}
