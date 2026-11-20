#!/usr/bin/env bash
# SigmaOS JS Interpreter — JavaScript runtime scaffold (Node.js / mozjs).
set -euo pipefail
usage() { cat >&2 <<EOF
Usage: $(basename "$0") <command> [options]
Commands:
  run <file>   Execute a JavaScript file
  repl         Start interactive JavaScript REPL
  version      Show JS runtime version
  install      Install Node.js or mozjs runtime
EOF
exit 1; }
detect_runtime() { which node 2>/dev/null && echo "node" || which deno 2>/dev/null && echo "deno" || echo ""; }
cmd_run() { local f="${1:-}"; local rt=$(detect_runtime); [[ -z "$rt" ]] && { echo "error: no JS runtime found. Run 'sigma-js install'" >&2; return 1; }
  echo "[js] Running $f with $rt"; "$rt" "$f"; }
cmd_repl() { local rt=$(detect_runtime); [[ -z "$rt" ]] && { echo "error: no JS runtime" >&2; return 1; }; exec "$rt"; }
cmd_version() { local rt=$(detect_runtime); [[ -n "$rt" ]] && "$rt" --version || echo "No JS runtime installed."; }
cmd_install() { echo "Installing Node.js runtime..."; echo "  (Real impl: sigma-pkg install nodejs)"; }
main() { local cmd="${1:-}"; shift || true
  case "$cmd" in run) cmd_run "$@";; repl) cmd_repl;; version) cmd_version;; install) cmd_install;; *) usage;; esac; }
main "$@"
