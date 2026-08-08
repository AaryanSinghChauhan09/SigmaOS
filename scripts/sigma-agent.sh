#!/usr/bin/env bash
# Host-side sigma-agent wrapper for development (builds cpp_host tests binary patterns).
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

usage() {
  cat <<EOF
sigma-agent — Sovereign Copilot CLI (host wrapper)

Usage:
  ./scripts/sigma-agent.sh chat <prompt>
  ./scripts/sigma-agent.sh gui <command>
  ./scripts/sigma-agent.sh tools
  ./scripts/sigma-agent.sh test
  ./scripts/sigma-agent.sh validate

Examples:
  ./scripts/sigma-agent.sh gui status
  ./scripts/sigma-agent.sh gui layout tile
  ./scripts/sigma-agent.sh gui launch settings
  ./scripts/sigma-agent.sh chat "open browser and go to sigmaos.local"
EOF
}

cmd_validate() {
  "${ROOT}/scripts/validate_agent_skills.sh"
  for f in \
    "${ROOT}/profiles/ai_agent/system_prompt.md" \
    "${ROOT}/profiles/ai_agent/tools.json" \
    "${ROOT}/skills/zenith-gui/SKILL.md" \
    "${ROOT}/skills/sigma-copilot/SKILL.md"; do
    if [[ -f "$f" ]]; then
      echo "OK  $f"
    else
      echo "MISSING $f"
      exit 1
    fi
  done
}

cmd_test() {
  cmake -S "${ROOT}/tests/cpp_host" -B "${ROOT}/build/cpp_host"
  cmake --build "${ROOT}/build/cpp_host"
  ctest --test-dir "${ROOT}/build/cpp_host" --output-on-failure -R "SigmaCopilot"
}

cmd_gui() {
  shift
  local cmd="$*"
  echo "[sigma-agent gui] $cmd"
  case "$cmd" in
    status)     echo '{"running":true,"layout":"tile","theme":"obsidian","workspace":0}' ;;
    layout\ *)  echo "Layout set to ${cmd#layout }" ;;
    theme\ *)   echo "Theme set to ${cmd#theme }" ;;
    launch\ *)  echo "Launched ${cmd#launch }" ;;
    apps)       echo "zenith-panel (Zenith Panel) ws=0 running";;
    settings\ *) echo "Settings: $cmd" ;;
    files\ *)   echo "Files: $cmd" ;;
    browser\ *) echo "Navigated: $cmd" ;;
    dashboard*) echo "CPU 42% | MEM 6.1/16G | GPU 18%" ;;
    *)          echo "Unknown gui command: $cmd"; exit 1 ;;
  esac
}

cmd_chat() {
  shift
  local prompt="$*"
  echo "[sigma-agent] Plan: parse intent → ZenithGUI tool"
  if echo "$prompt" | grep -qiE 'theme|layout|settings|browser|files|dashboard|workspace'; then
    cmd_gui status
  else
    echo "[LLM] $prompt"
  fi
}

cmd_tools() {
  echo "Read          Read file contents from SemanticFS"
  echo "Write         Create or overwrite a file"
  echo "Edit          Apply targeted string replacement"
  echo "Bash          Run sigma-sh command"
  echo "Grep          Ripgrep search"
  echo "Glob          Find files by glob"
  echo "ZenithGUI     Control Zenith Desktop via CLI"
  echo "Skill         Dispatch agent skill"
  echo "Memory        Layered agent memory L0-L3"
  echo "ComputerUse   Accessibility GUI automation"
  echo "Pkg           sigma-pkg package manager"
  echo "Net           sigma-net diagnostics"
  echo "Sec           sigma-sec audit"
  echo "LLM           xLLM inference"
  echo "FastContext   Repo exploration with citations"
}

main() {
  local cmd="${1:-help}"
  case "$cmd" in
    chat)     cmd_chat "$@" ;;
    gui)      cmd_gui "$@" ;;
    tools)    cmd_tools ;;
    test)     cmd_test ;;
    validate) cmd_validate ;;
    help|--help|-h) usage ;;
    *) usage; exit 1 ;;
  esac
}

main "$@"
