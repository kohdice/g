SRC := .
BIN := g
ifeq ($(OS),Windows_NT)
BIN := $(BIN).exe
endif
VERSION := $(shell git describe --tags --always --dirty)
CURRENT_REVISION := $(shell git rev-parse --short HEAD)
BUILD_LDFLAGS := "-s -w -X main.revision=$(CURRENT_REVISION) -X main.version=$(VERSION)"
GOOS := $(shell go env GOOS)

.PHONY: all
all: clean build

.PHONY: build
build:
	go build -trimpath -ldflags=$(BUILD_LDFLAGS) -o $(BIN) $(SRC)

.PHONY: install
install:
	go install -ldflags=$(BUILD_LDFLAGS) .

.PHONY: release
release: build
ifeq ($(GOOS), windows)
	zip -r g-$(GOOS)-$(VERSION).zip $(BIN)
else
	tar -czf g-$(GOOS)-$(VERSION).tar.gz $(BIN)
endif

.PHONY: clean
clean:
	go clean

.PHONY: tidy
tidy:
	go mod tidy

.PHONY: fmt
fmt:
	go fmt ./...

.PHONY: lint
lint:
	go vet ./...

.PHONY: test
test:
	go test -v -cover ./...

.PHONY: coverage
coverage:
	go test -v -coverprofile=coverage.out -covermode=atomic ./...
	go tool cover -html=coverage.out -o coverage.html
