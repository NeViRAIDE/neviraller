package scrap

import (
	"context"
	"net/http"
	"regexp"

	"github.com/PuerkitoBio/goquery"
)

const nvimReleaseURL = "https://github.com/neovim/neovim/releases/tag/nightly"

func GetNeovimVersionFromMetaTag(ctx context.Context) (string, error) {
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
