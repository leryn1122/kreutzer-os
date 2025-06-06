# Project
SHELL := /bin/bash
PROJECT := cmdb-agent
VERSION := 0.1.0
BUILD_DATE := $(shell date +%Y%m%d)
GIT_VERSION := $(shell git describe --long --all 2>/dev/null)
GIT_SHA := $(shell git rev-parse --short=8 HEAD 2>/dev/null)

# Arch
ARCH ?= x86_64

# Toolchain
CARGO := cargo
CARGO_TARGET := x86_64-unknown-uefi

# Main

# Docker


ifeq ($(ARCH),x86_64)
  CARGO_TARGET := $(ARCH)-unknown-kernel
  BOOT_EFI=EFI/BOOT/bootx64.efi
  GRUB_EFI=EFI/BOOT/grubx64.efi
else ifeq ($(ARCH),aarch64)
  CARGO_TARGET := $(ARCH)-unknown-kernel
  BOOT_EFI=EFI/BOOT/bootaa64.efi
  GRUB_EFI=EFI/BOOT/grubaa64.efi
else ifeq ($(ARCH),riscv64)
  CARGO_TARGET := $(ARCH)-unknown-kernel
  # TODO
endif

##@ General

.PHONY: help
help: ## Print help info
	@ awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_0-9-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Development
.PHONY: install check format clean unittest test
install: ## Install dependencies
	$(CARGO) fetch

check: ## Check
	$(CARGO) clippy

format: ## Format against code
	$(CARGO) fmt

clean: ## Clean target artifact
	$(CARGO) clean

unittest: ## Run all unit tests

test: ## Run all integrity tests
	$(CARGO) test \
      --target=triplet/$(patsubst %-none,%-kernel,$(CARGO_TARGET)).json

##@ Build
.PHONY: build run image
build: ## Run the target artifact
	$(CARGO) build \
	  --target=triplet/$(patsubst %-none,%-kernel,$(CARGO_TARGET)).json

run: ## Run local demo
	$(CARGO) run \
      --target=triplet/$(patsubst %-none,%-kernel,$(CARGO_TARGET)).json

image: ## Build the OCI image
