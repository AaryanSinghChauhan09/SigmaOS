#!/usr/bin/env sh
# SPDX-License-Identifier: MIT
# SigmaOS Benchmark Report Generator (Native POSIX Shell Edition)

set -e

BOOT_FILE=""
MEMORY_FILE=""
OUTPUT_FILE=""

while [ "$#" -gt 0 ]; do
    case "$1" in
        --boot)
            BOOT_FILE="$2"
            shift 2
            ;;
        --memory)
            MEMORY_FILE="$2"
            shift 2
            ;;
        --output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        *)
            shift
            ;;
    esac
done

if [ -z "$OUTPUT_FILE" ]; then
    echo "Error: --output parameter required."
    exit 1
fi

extract_json_val() {
    file="$1"
    key="$2"
    if [ -f "$file" ]; then
        grep -o "\"${key}\":[0-9.]*" "$file" | cut -d':' -f2 || echo "N/A"
    else
        echo "N/A"
    fi
}

BOOT_TIME=$(extract_json_val "$BOOT_FILE" "boot_time_ms")
KERNEL_INIT=$(extract_json_val "$BOOT_FILE" "kernel_init_time_ms")
USERLAND_INIT=$(extract_json_val "$BOOT_FILE" "userland_init_time_ms")

PEAK_MEM=$(extract_json_val "$MEMORY_FILE" "peak_memory_mb")
KERNEL_HEAP=$(extract_json_val "$MEMORY_FILE" "kernel_heap_mb")
USERLAND_RSS=$(extract_json_val "$MEMORY_FILE" "userland_rss_mb")

cat <<EOF > "$OUTPUT_FILE"
# Performance Benchmark Report

## Boot Performance
- **Boot Time:** ${BOOT_TIME:-N/A} ms
- **Kernel Init:** ${KERNEL_INIT:-N/A} ms
- **Userland Init:** ${USERLAND_INIT:-N/A} ms

## Memory Usage
- **Peak Memory:** ${PEAK_MEM:-N/A} MB
- **Kernel Heap:** ${KERNEL_HEAP:-N/A} MB
- **Userland RSS:** ${USERLAND_RSS:-N/A} MB
EOF

echo "Report generated at ${OUTPUT_FILE}"
