#!/usr/bin/env bash
# Host smoke test for kernel/net/sigma_net.c (no kernel headers required)
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cc -std=c11 -Wall -Wextra \
  "${ROOT}/tools/sigma_net_host_test.c" -o "${ROOT}/sigma_net_test"
"${ROOT}/sigma_net_test"
echo "sigma_net_host_test: OK"
