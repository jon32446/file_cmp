#!/bin/bash
#
# Performance comparison between file_cmp and cmp
#
# This script compares the performance of file_cmp against the standard cmp utility.
# It tests both file comparison and various scenarios.
#
# Usage: ./benchmark.sh
#

set -e

# Check if file_cmp binary exists
if [ ! -f target/release/file_cmp ]; then
    echo "Building file_cmp in release mode..."
    cargo build --release
fi

# Create temporary directory for test files
TMPDIR=$(mktemp -d)
trap "rm -rf $TMPDIR" EXIT

echo "=== file_cmp Performance Benchmark ==="
echo "Test directory: $TMPDIR"
echo ""

# Create test files of various sizes
echo "Creating test files..."

# 1MB identical files
dd if=/dev/urandom of="$TMPDIR/file1_1mb" bs=1M count=1 2>/dev/null
cp "$TMPDIR/file1_1mb" "$TMPDIR/file2_1mb"

# 10MB identical files
dd if=/dev/urandom of="$TMPDIR/file1_10mb" bs=1M count=10 2>/dev/null
cp "$TMPDIR/file1_10mb" "$TMPDIR/file2_10mb"

# 100MB identical files
dd if=/dev/urandom of="$TMPDIR/file1_100mb" bs=1M count=100 2>/dev/null
cp "$TMPDIR/file1_100mb" "$TMPDIR/file2_100mb"

# 10MB files that differ at the beginning
cp "$TMPDIR/file1_10mb" "$TMPDIR/file3_10mb"
echo "X" | dd of="$TMPDIR/file3_10mb" bs=1 count=1 conv=notrunc 2>/dev/null

# 10MB files that differ at the end
cp "$TMPDIR/file1_10mb" "$TMPDIR/file4_10mb"
echo "X" | dd of="$TMPDIR/file4_10mb" bs=1 count=1 seek=10485759 conv=notrunc 2>/dev/null

echo ""
echo "=== Test 1: 1MB Identical Files ==="
echo "cmp:"
time cmp -s "$TMPDIR/file1_1mb" "$TMPDIR/file2_1mb"
echo ""
echo "file_cmp:"
time ./target/release/file_cmp "$TMPDIR/file1_1mb" "$TMPDIR/file2_1mb" > /dev/null
echo ""
echo "file_cmp (quick mode):"
time ./target/release/file_cmp -q "$TMPDIR/file1_1mb" "$TMPDIR/file2_1mb" > /dev/null
echo ""

echo "=== Test 2: 10MB Identical Files ==="
echo "cmp:"
time cmp -s "$TMPDIR/file1_10mb" "$TMPDIR/file2_10mb"
echo ""
echo "file_cmp:"
time ./target/release/file_cmp "$TMPDIR/file1_10mb" "$TMPDIR/file2_10mb" > /dev/null
echo ""
echo "file_cmp (quick mode):"
time ./target/release/file_cmp -q "$TMPDIR/file1_10mb" "$TMPDIR/file2_10mb" > /dev/null
echo ""

echo "=== Test 3: 100MB Identical Files ==="
echo "cmp:"
time cmp -s "$TMPDIR/file1_100mb" "$TMPDIR/file2_100mb"
echo ""
echo "file_cmp:"
time ./target/release/file_cmp "$TMPDIR/file1_100mb" "$TMPDIR/file2_100mb" > /dev/null
echo ""
echo "file_cmp (quick mode):"
time ./target/release/file_cmp -q "$TMPDIR/file1_100mb" "$TMPDIR/file2_100mb" > /dev/null
echo ""

echo "=== Test 4: 10MB Files - Differ at Beginning ==="
echo "cmp:"
time cmp -s "$TMPDIR/file1_10mb" "$TMPDIR/file3_10mb" || true
echo ""
echo "file_cmp:"
time ./target/release/file_cmp "$TMPDIR/file1_10mb" "$TMPDIR/file3_10mb" > /dev/null || true
echo ""
echo "file_cmp (quick mode):"
time ./target/release/file_cmp -q "$TMPDIR/file1_10mb" "$TMPDIR/file3_10mb" > /dev/null || true
echo ""

echo "=== Test 5: 10MB Files - Differ at End ==="
echo "cmp:"
time cmp -s "$TMPDIR/file1_10mb" "$TMPDIR/file4_10mb" || true
echo ""
echo "file_cmp:"
time ./target/release/file_cmp "$TMPDIR/file1_10mb" "$TMPDIR/file4_10mb" > /dev/null || true
echo ""
echo "file_cmp (quick mode):"
time ./target/release/file_cmp -q "$TMPDIR/file1_10mb" "$TMPDIR/file4_10mb" > /dev/null || true
echo ""

echo "=== Test 6: Chunk Size Comparison (100MB) ==="
echo "file_cmp (4K chunks):"
time ./target/release/file_cmp -c 4k "$TMPDIR/file1_100mb" "$TMPDIR/file2_100mb" > /dev/null
echo ""
echo "file_cmp (64K chunks - default):"
time ./target/release/file_cmp "$TMPDIR/file1_100mb" "$TMPDIR/file2_100mb" > /dev/null
echo ""
echo "file_cmp (1M chunks):"
time ./target/release/file_cmp -c 1m "$TMPDIR/file1_100mb" "$TMPDIR/file2_100mb" > /dev/null
echo ""

echo "=== Benchmark Complete ==="
