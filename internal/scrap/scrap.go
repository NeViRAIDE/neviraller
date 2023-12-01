// Package scrap provides web scraping functionalities, particularly for fetching
// version information of software from web pages. It is used here to scrape
// the version of Neovim from its GitHub release page.
package scrap

import (
	"context"
	"net/http"
	"regexp"

	"github.com/PuerkitoBio/goquery" // Importing goquery for HTML parsing.
)

// nvimReleaseURL is the URL of the Neovim nightly release page on GitHub.
const nvimReleaseURL = "https://github.com/neovim/neovim/releases/tag/nightly"

// GetNeovimVersionFromMetaTag scrapes the Neovim release page to find the current version number.
// ctx: A context to control the request's lifetime.
// Returns: The scraped version string and an error if the operation fails.
func GetNeovimVersionFromMetaTag(ctx context.Context) (string, error) {
	// Creating an HTTP GET request with the provided context.
	req, err := http.NewRequestWithContext(ctx, "GET", nvimReleaseURL, nil)
	if err != nil {
		return "", err
	}

	// Sending the HTTP request.
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close() // Ensure the response body is closed after processing.

	// Parsing the HTML content of the page.
	doc, err := goquery.NewDocumentFromReader(resp.Body)
	if err != nil {
		return "", err
	}

	// Variable to store the extracted version number.
	var version string

	// Searching for a meta tag with a specific name and extracting its content.
	doc.Find("meta[name='twitter:description']").Each(func(_ int, item *goquery.Selection) {
		// Getting the content attribute of the meta tag.
		content, _ := item.Attr("content")

		// Compiling a regular expression to match the version pattern.
		r, _ := regexp.Compile(`NVIM v[0-9]+\.[0-9]+\.[0-9]+-dev-[a-fA-F0-9]+`)

		// Using the regular expression to find the version number in the content string.
		version = r.FindString(content)
	})

	// Returning the extracted version number.
	return version, nil
}
