#!/bin/bash
set -e

BINARY="${1:?Usage: ./submit.sh <binary_path>}"

if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found: $BINARY"
    exit 1
fi

echo "=== BINARY RECONSTRUCTOR SUBMISSION ==="
echo "Target: $BINARY"
echo

file "$BINARY"
echo

TMPDIR=/dev/shm/br_$$
mkdir -p "$TMPDIR"
trap "rm -rf $TMPDIR" EXIT

TEXT_LINE=$(objdump -h "$BINARY" 2>/dev/null | grep "\.text")
TEXT_OFFSET="0x$(echo "$TEXT_LINE" | awk '{print $6}')"
TEXT_SIZE="0x$(echo "$TEXT_LINE" | awk '{print $3}')"

if [ -z "$TEXT_OFFSET" ] || [ -z "$TEXT_SIZE" ]; then
    echo "Error: Could not extract .text section"
    exit 1
fi

echo ".text: offset=$TEXT_OFFSET size=$TEXT_SIZE"
echo

mkdir -p /home/inf/.analysis
cat > /home/inf/.analysis/config << EOF
BINARY_PATH=$BINARY
TEXT_OFFSET=$TEXT_OFFSET
TEXT_SIZE=$TEXT_SIZE
EOF

echo "Generating pseudo-source..."
/home/inf/.cargo_target/release/synthesize_source > "$TMPDIR/output.c" 2>/dev/null

echo "Computing quality metrics..."
/home/inf/.cargo_target/release/quality_metrics "$TMPDIR/output.c"

echo
echo "=== SYNTHESIS RESULTS ==="
wc -l "$TMPDIR/output.c"
echo
echo "First 50 lines:"
head -50 "$TMPDIR/output.c"

cp "$TMPDIR/output.c" "reconstructed_$(basename $BINARY).c"
echo
echo "Full output: reconstructed_$(basename $BINARY).c"
