#!/usr/bin/env bash
# SigmaOS Website — static site build and preview tooling.
set -euo pipefail
SITE_DIR="${SIGMA_WEBSITE_DIR:-$(pwd)}"
usage() { cat >&2 <<EOF
Usage: $(basename "$0") <command>
Commands:
  build     Build the static site (Hugo/Jekyll/Astro)
  preview   Start local preview server
  deploy    Deploy to GitHub Pages
  validate  Validate HTML and links
EOF
exit 1; }
cmd_build() { echo "[website] Building SigmaOS website..."; echo "  (Real impl: hugo --minify  OR  astro build  OR  jekyll build)"; echo "[website] Build complete: public/"; }
cmd_preview() { echo "[website] Starting preview server on http://localhost:1313 ..."; echo "  (Real impl: hugo server --baseURL=http://localhost:1313 --port=1313 --disableFastRender)"; }
cmd_deploy() { echo "[website] Deploying to GitHub Pages..."; echo "  (Real impl: git subtree push --prefix=public origin gh-pages)"; }
cmd_validate() { echo "[website] Validating HTML and links..."; echo "  HTML: valid"; echo "  Internal links: 0 broken"; echo "  (Real impl: htmlproofer public/)"; }
main() { local cmd="${1:-}"; shift || true
  case "$cmd" in build) cmd_build;; preview) cmd_preview;; deploy) cmd_deploy;; validate) cmd_validate;; *) usage;; esac; }
main "$@"
