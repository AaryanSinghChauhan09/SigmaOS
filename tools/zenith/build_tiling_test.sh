#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT="${ROOT}/sigma_tiling_test"
c++ -std=c++17 -Wall -Wextra "${ROOT}/tools/zenith/sigma_tiling_test.cpp" -o "$OUT"
"$OUT"
echo "sigma_tiling_test: OK"
