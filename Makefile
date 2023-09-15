.PHONY: run
run:
	go run .

.PHONY: build
build:
	go build .

.DEFAULT_GOAL := run
