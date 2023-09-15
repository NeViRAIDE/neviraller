package neovimnightly

import (
	"context"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"os/exec"
	"regexp"
	"strings"
	"time"

	"github.com/PuerkitoBio/goquery"
)

const (
	nvimReleaseURL  = "https://github.com/neovim/neovim/releases/tag/nightly"
	nvimDownloadURL = "https://github.com/neovim/neovim/releases/download/nightly/nvim.appimage"
)

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

	newNvimVer, err := getNeovimVersionFromMetaTag(ctx)
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
		fmt.Println("No new version of Neovim Nightly found!")
		return
	}

	fmt.Printf("New Neovim Nightly version found!\n%s -> %s\n", currNvimVer, newNvimVer)

	var input string
	fmt.Print("Do you wish to update neovim? [yes/no] ")
	fmt.Scanln(&input)

	if strings.ToLower(input) == "yes" || strings.ToLower(input) == "y" {
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

		fmt.Println("Neovim Nightly has been updated successfully!")
	}
}

func getNeovimVersionFromMetaTag(ctx context.Context) (string, error) {
	req, err := http.NewRequestWithContext(ctx, "GET", nvimReleaseURL, nil)
	if err != nil {
		return "", err
	}

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	doc, err := goquery.NewDocumentFromReader(resp.Body)
	if err != nil {
		return "", err
	}

	var version string
	doc.Find("meta[name='twitter:description']").Each(func(_ int, item *goquery.Selection) {
		content, _ := item.Attr("content")
		r, _ := regexp.Compile(`NVIM v[0-9]+\.[0-9]+\.[0-9]+-dev-[a-fA-F0-9]+`)
		version = r.FindString(content)
	})

	return version, nil
}
