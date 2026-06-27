.PHONY: build clean test reconstruct compile run help

BINARY ?= /usr/bin/cat
OUTPUT_DIR := /tmp
OUTPUT_NAME := $(notdir $(BINARY))_reconstructed
RECONSTRUCTOR := ./target/release/binary-reconstructor

help:
	@echo "Usage: make reconstruct BINARY=/path/to/binary"
	@echo "       make compile BINARY=/path/to/binary"
	@echo "       make run BINARY=/path/to/binary"
	@echo ""
	@echo "Examples:"
	@echo "  make reconstruct BINARY=/usr/bin/cat"
	@echo "  make reconstruct BINARY=/usr/bin/dolphin"
	@echo "  make reconstruct BINARY=/usr/bin/nmap"

build:
	TMPDIR=/dev/shm/cargo cargo build --release

clean:
	rm -f /tmp/$(OUTPUT_NAME)*
	rm -rf /tmp/test_* /tmp/dolphin_* /tmp/reconstructed_*

reconstruct: build
	@echo "Reconstructing $(BINARY)..."
	@$(RECONSTRUCTOR) $(BINARY) 2>/dev/null > /tmp/$(OUTPUT_NAME).cpp
	@echo "Generated: /tmp/$(OUTPUT_NAME).cpp"

compile: reconstruct
	@echo "Compiling..."
	@file /tmp/$(OUTPUT_NAME).cpp | grep -q "C\+\+ source" && \
		g++ /tmp/$(OUTPUT_NAME).cpp -o /tmp/$(OUTPUT_NAME) $$(pkg-config --cflags --libs Qt5Core Qt5Gui Qt5Widgets) 2>&1 && echo "✓ Compiled C++" || echo "Compile failed" || \
		rustc /tmp/$(OUTPUT_NAME).cpp -o /tmp/$(OUTPUT_NAME) 2>&1 && echo "✓ Compiled Rust" || echo "Compile failed"

run: compile
	@echo "Running $(OUTPUT_NAME)..."
	@/tmp/$(OUTPUT_NAME)

test-all: build
	@echo "=== Testing cat ==="
	@$(RECONSTRUCTOR) /usr/bin/cat 2>&1 | grep Type
	@echo "=== Testing nmap ==="
	@$(RECONSTRUCTOR) /usr/bin/nmap 2>&1 | grep Type
	@echo "=== Testing dolphin ==="
	@$(RECONSTRUCTOR) /usr/bin/dolphin 2>&1 | grep Type
	@echo "=== Testing grep ==="
	@$(RECONSTRUCTOR) /usr/bin/grep 2>&1 | grep Type
