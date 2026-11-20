#!/usr/bin/env bash
# scripts/find_doc_todos.sh
# Scans documentation files and codebase for TODO/FIXME/PLANNED items to track backlog tasks.

set -e

echo "Scanning repository documentation and sources for TODOs, FIXMEs, and PLANNED tasks..."

SEARCH_PATHS=("docs" "wiki" "README.md" "src")
OUTPUT_FILE="build/doc_todos_summary.txt"
mkdir -p build

echo "=== SigmaOS Documentation & Codebase TODO Summary ===" > "$OUTPUT_FILE"
echo "Generated on: $(date)" >> "$OUTPUT_FILE"
echo "--------------------------------------------------------" >> "$OUTPUT_FILE"

TOTAL_ITEMS=0

for path in "${SEARCH_PATHS[@]}"; do
    if [ -e "$path" ]; then
        echo "Scanning $path..."
        HITS=$(grep -rnE "(TODO|FIXME|PLANNED|TBD|RFC):" "$path" 2>/dev/null || true)
        if [ -n "$HITS" ]; then
            COUNT=$(echo "$HITS" | wc -l)
            TOTAL_ITEMS=$((TOTAL_ITEMS + COUNT))
            echo "--- $path ($COUNT items) ---" >> "$OUTPUT_FILE"
            echo "$HITS" >> "$OUTPUT_FILE"
            echo "" >> "$OUTPUT_FILE"
        fi
    fi
done

echo "--------------------------------------------------------" >> "$OUTPUT_FILE"
echo "Total actionable items found: $TOTAL_ITEMS" >> "$OUTPUT_FILE"

cat "$OUTPUT_FILE"
echo "Summary written to $OUTPUT_FILE"
exit 0
