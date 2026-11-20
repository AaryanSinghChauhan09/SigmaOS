#!/usr/bin/env bash
# Collects system diagnostic report for SigmaOS failure analysis

set -euo pipefail

OUTPUT_DIR="build/diagnostics_report"
mkdir -p "$OUTPUT_DIR"

echo "Collecting SigmaOS System Diagnostics Report..."

# Capture git commit and build info if available
git rev-parse HEAD > "$OUTPUT_DIR/git_commit.txt" 2>/dev/null || echo "Unknown commit" > "$OUTPUT_DIR/git_commit.txt"
date -u +"%Y-%m-%dT%H:%M:%SZ" > "$OUTPUT_DIR/timestamp.txt"

# Check rust toolchain version
rustc --version > "$OUTPUT_DIR/rust_version.txt" 2>/dev/null || true

# Summary file
cat <<EOF > "$OUTPUT_DIR/summary.json"
{
  "system": "SigmaOS",
  "report_timestamp": "$(cat "$OUTPUT_DIR/timestamp.txt")",
  "commit": "$(cat "$OUTPUT_DIR/git_commit.txt")",
  "status": "Diagnostic collection complete"
}
EOF

echo "System diagnostic report generated at $OUTPUT_DIR/summary.json"
