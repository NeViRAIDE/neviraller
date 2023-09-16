.PHONY: run
run:
	go run ./cmd/installer/main.go

.PHONY: build
build:
	go build -v -o ./NEVIRALLER ./cmd/installer/main.go

.DEFAULT_GOAL := run
