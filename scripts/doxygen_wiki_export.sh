#!/usr/bin/env bash
# Generate Doxygen HTML then mirror a subsystem index into wiki_repo/.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WIKI="${ROOT}/wiki_repo"
API="${ROOT}/docs/api/html"

if command -v doxygen >/dev/null 2>&1; then
  doxygen "${ROOT}/Doxyfile"
else
  echo "[doxygen-wiki] doxygen not installed; skipping HTML generation"
fi

mkdir -p "$WIKI"
cat > "${WIKI}/API-Reference.md" <<'EOF'
# API Reference (Doxygen)

Generated from `kernel/` and `include/` sources.

## Build locally

```bash
doxygen Doxyfile
# Output: docs/api/html/index.html
```

## Subsystem entry points

| Area | Headers |
|------|---------|
| Networking | `include/sigma_net.h`, `kernel/include/sigma_socket_abi.h` |
| Pods | `include/sigma_pod_spec.h` |
| Boot | `include/sigma_boot.h` |
| Zenith | `include/sigma_theme.h` |

EOF

echo "[doxygen-wiki] wrote ${WIKI}/API-Reference.md"
if [[ -f "${API}/index.html" ]]; then
  echo "[doxygen-wiki] HTML at docs/api/html/index.html"
fi
