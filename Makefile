.PHONY: run
run:
	go run ./cmd/installer/main.go

.PHONY: build
build:
	go build -v -o ./installer ./cmd/installer/main.go

.DEFAULT_GOAL := run
