UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
    WASI_OS := linux
endif
ifeq ($(UNAME_S),Darwin)
    WASI_OS := macos
endif

UNAME_M := $(shell uname -m)
ifeq ($(UNAME_M),x86_64)
    WASI_ARCH := x86_64
endif
ifeq ($(UNAME_M),arm64)
    WASI_ARCH := arm64
endif
ifeq ($(UNAME_M),aarch64)
    WASI_ARCH := arm64
endif

WASI_VERSION := 27
WASI_VERSION_FULL := $(WASI_VERSION).0

BUILD_DIR := build
WASI_SDK_DIR := $(BUILD_DIR)/wasi-sdk
WASI_SDK_ARCHIVE_NAME := wasi-sdk-$(WASI_VERSION_FULL)-$(WASI_ARCH)-$(WASI_OS)
WASI_SDK_ARCHIVE := $(BUILD_DIR)/$(WASI_SDK_ARCHIVE_NAME).tar.gz
WASI_SDK_URL := https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-$(WASI_VERSION)/$(WASI_SDK_ARCHIVE_NAME).tar.gz

.PHONY: help install-wasi-sdk build-wasm dev build-web clean

.DEFAULT_GOAL := help

help:
	@echo "Available commands:"
	@echo "  install-wasi-sdk  Install WASI SDK for WebAssembly compilation"
	@echo "  build-wasm        Build the project for WebAssembly target"
	@echo "  dev               Build WASM and start web project"
	@echo "  build-web         Build WASM and build web project"
	@echo "  clean             Clean build artifacts and downloads"

install-wasi-sdk: $(WASI_SDK_DIR)
	@echo "WASI SDK installed at: $(PWD)/$(WASI_SDK_DIR)"
	@echo "Export WASI_SDK_PATH=$(PWD)/$(WASI_SDK_DIR)"

build-wasm: $(WASI_SDK_DIR)
	CC="$(PWD)/$(WASI_SDK_DIR)/bin/clang --sysroot=$(PWD)/$(WASI_SDK_DIR)/share/wasi-sysroot" \
	cargo build --release --package tree-sitter-query-formatter-wasm --target=wasm32-wasip2

dev: build-wasm
	(cd ./web && npm install && npm run dev)

build-web: build-wasm
	(cd ./web && npm install && npm run build)

$(WASI_SDK_DIR): $(WASI_SDK_ARCHIVE)
	@mkdir -p $(BUILD_DIR)
	cd $(BUILD_DIR) && tar xvf ../$(WASI_SDK_ARCHIVE)
	cd $(BUILD_DIR) && mv $(WASI_SDK_ARCHIVE_NAME) wasi-sdk
	@touch $(WASI_SDK_DIR)

$(WASI_SDK_ARCHIVE):
	@echo "Downloading WASI SDK $(WASI_VERSION_FULL) for $(WASI_OS)-$(WASI_ARCH)..."
	@mkdir -p $(BUILD_DIR)
ifeq ($(WASI_OS),macos)
	curl -L -o $(WASI_SDK_ARCHIVE) $(WASI_SDK_URL)
else
	cd $(BUILD_DIR) && wget $(WASI_SDK_URL)
endif

clean:
	@echo "Cleaning build artifacts and downloads..."
	rm -rf target/
	rm -rf $(WASI_SDK_DIR)
	rm -rf $(BUILD_DIR)
	(cd ./web && npm run clean 2>/dev/null || echo "No npm clean script found")
