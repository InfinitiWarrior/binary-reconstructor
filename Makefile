.PHONY: all build release run clean test poc

BINARY     := target/release/reconstructor
DEBUG_BIN  := target/debug/reconstructor
BIN        ?= /usr/bin/md5sum
CARGO_ENV  := TMPDIR=/dev/shm/cargo

all: build

build:
	$(CARGO_ENV) cargo build

release:
	$(CARGO_ENV) cargo build --release

run: build
	$(DEBUG_BIN) $(BIN) 2>analysis.log
	@echo ""
	@echo "=== Analysis log ==="
	@cat analysis.log

poc: release
	@echo "=== Binary Reconstructor PoC ==="
	@echo "Target: $(BIN)"
	@echo ""
	$(BINARY) $(BIN) 2>analysis.log | tee reconstructed.c | \
		grep -E "void fn_|MD5|SSL|connect|fwrite|setlocale|dcgettext|malloc|calloc|fopen|socket" | head -40
	@echo ""
	@echo "=== Analysis Summary ==="
	@cat analysis.log
	@echo ""
	@echo "=== Output saved to reconstructed.c ==="
	@wc -l reconstructed.c

clean:
	cargo clean
	rm -f reconstructed.c analysis.log

test: build
	@echo "--- md5sum ---"
	$(DEBUG_BIN) /usr/bin/md5sum 2>/dev/null | grep -cE "void fn_" || true
	@echo "functions found"
	$(DEBUG_BIN) /usr/bin/md5sum 2>/dev/null | grep -E "MD5_Init|MD5_Update|MD5_Final" | head -5