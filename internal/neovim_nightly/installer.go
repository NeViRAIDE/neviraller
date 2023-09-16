package neovimnightly

import (
	"context"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"os/exec"
	"strings"
	"time"

	"github.com/RAprogramm/neviraide-install/internal/scrap"
	"github.com/RAprogramm/neviraide-install/internal/utils"
)

const nvimDownloadURL = "https://github.com/neovim/neovim/releases/download/nightly/nvim.appimage"

func checkCommandExists(command string) error {
	_, err := exec.LookPath(command)
	if err != nil {
		return fmt.Errorf("command %s not found: %v", command, err)
	}
	return nil
}

func InstallNeovim() {
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

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	newNvimVer, err := scrap.GetNeovimVersionFromMetaTag(ctx)
	if err != nil {
		log.Fatalf("Error getting Neovim version: %v", err)
	}

	cmd := exec.Command("nvim", "--version")
	out, err := cmd.CombinedOutput()
	if err != nil {
		log.Fatalf("Error getting current Neovim version: %v", err)
	}

	currNvimVer := strings.Split(string(out), "\n")[0]

	if currNvimVer == newNvimVer {
		fmt.Print(utils.Color("green", "", "\nYou already have latest version: %s\n", currNvimVer))
		fmt.Println(utils.Color("red", "bold", "NOT FOUND new version of Neovim Nightly!"))
		fmt.Println("")
		return
	}

	fmt.Print(utils.Color("blue", "italic", "\nNew Neovim Nightly version found!\n"))
	fmt.Print(utils.Color("grey", "italic", "%s -> %s\n\n", currNvimVer, newNvimVer))

	if utils.Confirm("Do you wish to update neovim") {
		resp, err := http.Get(nvimDownloadURL)
		if err != nil {
			log.Fatalf("Error downloading Neovim Nightly: %v", err)
		}
		defer resp.Body.Close()

		out, err := os.Create("/tmp/nvim")
		if err != nil {
			log.Fatalf("Error creating temp file: %v", err)
		}
		defer out.Close()

		_, err = io.Copy(out, resp.Body)
		if err != nil {
			log.Fatalf("Error writing to temp file: %v", err)
		}

		cmd = exec.Command("pkill", "nvim")
		cmd.Run()

		cmd = exec.Command("sudo", "cp", "/tmp/nvim", "/usr/local/bin")
		cmd.Run()

		cmd = exec.Command("sudo", "mv", "/tmp/nvim", "/usr/bin")
		cmd.Run()

		fmt.Print(utils.Color("green", "italic", "\nNeovim Nightly has been updated successfully!\n"))
	}

}
