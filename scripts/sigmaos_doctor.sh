#!/usr/bin/env bash
# SPDX-License-Identifier: MIT
# sigmaos_doctor.sh — Connector/control-plane health checker for SigmaOS.
# Validates the registry, hub scripts, component scaffold, and sibling
# SigmaOS-* component repositories. Does NOT check host display/GPU state.
# ---------------------------------------------------------------------------

set -uo pipefail

SIGMA_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
FLEET_ROOT="$(dirname "$SIGMA_ROOT")"
ACTIVE_REGISTRY="$SIGMA_ROOT/COMPONENTS-ACTIVE.toml"
FULL_REGISTRY="$SIGMA_ROOT/COMPONENTS.toml"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

PASS="${GREEN}PASS${RESET}"
WARN="${YELLOW}WARN${RESET}"
FAIL="${RED}FAIL${RESET}"

pass_count=0
warn_count=0
fail_count=0

check() {
  local label="$1" status="$2" detail="${3:-}"
  local badge
  case "$status" in
    pass) badge="$PASS"; (( pass_count++ )) || true ;;
    warn) badge="$WARN"; (( warn_count++ )) || true ;;
    fail) badge="$FAIL"; (( fail_count++ )) || true ;;
  esac
  if [[ -n "$detail" ]]; then
    printf "  %-44s %b  %s\n" "$label" "$badge" "$detail"
  else
    printf "  %-44s %b\n" "$label" "$badge"
  fi
}

section() { echo ""; echo -e "${CYAN}${BOLD}-- $* ${RESET}"; }
cmd_exists() { command -v "$1" &>/dev/null; }

check_file() {
  local label="$1" path="$2"
  if [[ -f "$path" ]]; then check "$label" pass
  else check "$label" fail "missing: $path"; fi
}

check_dir() {
  local label="$1" path="$2"
  if [[ -d "$path" ]]; then check "$label" pass
  else check "$label" fail "missing: $path"; fi
}

echo ""
echo -e "${BOLD}=== SigmaOS Connector Doctor ===${RESET}"
echo -e "  Root       : ${SIGMA_ROOT}"
echo -e "  Fleet root : ${FLEET_ROOT}"
echo -e "  Date       : $(date '+%Y-%m-%d %H:%M:%S')"

# ── 1. Connector files ───────────────────────────────────────────────────────
section "Connector files"

check_file "COMPONENT.toml (this repo)"      "$SIGMA_ROOT/COMPONENT.toml"
check_file "COMPONENTS.toml (full registry)" "$FULL_REGISTRY"
check_file "COMPONENTS-ACTIVE.toml"          "$ACTIVE_REGISTRY" 
check_file "Makefile"                        "$SIGMA_ROOT/Makefile"
check_file "AGENTS.md"                       "$SIGMA_ROOT/AGENTS.md"

for req in git python3 bash; do
  if cmd_exists "$req"; then check "tool: $req" pass "$(command -v "$req")"
  else check "tool: $req" fail "required tool not found"; fi
done

# ── 2. Hub & library ─────────────────────────────────────────────────────────
section "Hub & library"

check_file "scripts/sigma_hub.sh"          "$SIGMA_ROOT/scripts/sigma_hub.sh"
check_file "scripts/lib/component.sh"      "$SIGMA_ROOT/scripts/lib/component.sh"
check_file "scripts/audit_component_fleet.py" "$SIGMA_ROOT/scripts/audit_component_fleet.py"
check_file "scripts/audit_consolidation.py"   "$SIGMA_ROOT/scripts/audit_consolidation.py"
check_file "scripts/validate_component_registry.py" "$SIGMA_ROOT/scripts/validate_component_registry.py"

if [[ -x "$SIGMA_ROOT/scripts/sigma_hub.sh" ]]; then
  check "sigma_hub.sh executable" pass
else
  check "sigma_hub.sh executable" warn "not executable (chmod +x)"
fi

if bash -n "$SIGMA_ROOT/scripts/sigma_hub.sh" 2>/dev/null; then
  check "sigma_hub.sh syntax" pass
else
  check "sigma_hub.sh syntax" fail "bash -n failed"
fi

# ── 3. Component scaffold ────────────────────────────────────────────────────
section "Component scaffold"

check_dir  "templates/component"            "$SIGMA_ROOT/templates/component"
check_file "templates/component/Cargo.toml" "$SIGMA_ROOT/templates/component/Cargo.toml"
check_file "templates/component/COMPONENT.toml" "$SIGMA_ROOT/templates/component/COMPONENT.toml"
check_file "templates/component/src/lib.rs" "$SIGMA_ROOT/templates/component/src/lib.rs"

if grep -q 'ComponentRegistry' "$SIGMA_ROOT/templates/component/src/lib.rs" 2>/dev/null; then
  check "scaffold has ComponentRegistry" pass
else
  check "scaffold has ComponentRegistry" warn "trait/registry not found in lib.rs"
fi

# ── 4. Registry validity ─────────────────────────────────────────────────────
section "Registry"

if [[ -f "$FULL_REGISTRY" ]]; then
  if python3 -c "import tomllib,sys; tomllib.load(open('$FULL_REGISTRY','rb'))" 2>/dev/null; then
    entries=$(python3 -c "import tomllib; d=tomllib.load(open('$FULL_REGISTRY','rb')); print(len(d.get('component',d)))" 2>/dev/null || echo "?")
    check "COMPONENTS.toml parses as TOML" pass "${entries} entries"
  else
    check "COMPONENTS.toml parses as TOML" fail "invalid TOML"
  fi
fi

if [[ -f "$ACTIVE_REGISTRY" ]]; then
  if python3 -c "import tomllib; tomllib.load(open('$ACTIVE_REGISTRY','rb'))" 2>/dev/null; then
    check "COMPONENTS-ACTIVE.toml parses as TOML" pass
  else
    check "COMPONENTS-ACTIVE.toml parses as TOML" fail "invalid TOML"
  fi
fi

# ── 5. Sibling component repos ───────────────────────────────────────────────
section "Sibling component repos"

core_components=(SigmaOS-ISO SigmaOS-kernel SigmaOS-desktop SigmaOS-installer SigmaOS-hub SigmaOS-system-tools)
for comp in "${core_components[@]}"; do
  dir="$FLEET_ROOT/$comp"
  if [[ -d "$dir/.git" ]]; then
    if [[ -f "$dir/COMPONENT.toml" ]]; then
      check "$comp" pass "checked out + COMPONENT.toml"
    else
      check "$comp" warn "checked out but missing COMPONENT.toml"
    fi
  elif [[ -d "$dir" ]]; then
    check "$comp" warn "directory exists but not a git repo"
  else
    check "$comp" warn "not checked out locally (use: make hub-connect NAME=$comp)"
  fi
done

# Count total SigmaOS-* siblings present.
sibling_count=$(find "$FLEET_ROOT" -maxdepth 1 -type d -name 'SigmaOS-*' 2>/dev/null | wc -l)
check "SigmaOS-* siblings in fleet root" pass "$sibling_count repos"

# ── 6. Git state ─────────────────────────────────────────────────────────────
section "Git state"

if git -C "$SIGMA_ROOT" rev-parse --is-inside-work-tree &>/dev/null; then
  branch=$(git -C "$SIGMA_ROOT" branch --show-current 2>/dev/null || echo "?")
  check "connector on branch" pass "$branch"
  if [[ "$branch" == "main" ]]; then
    check "single main branch" pass
  else
    check "single main branch" warn "on '$branch' — AGENTS.md requires main"
  fi
fi

# ── Summary ──────────────────────────────────────────────────────────────────
total=$(( pass_count + warn_count + fail_count ))
echo ""
echo -e "${BOLD}=== Results: $total total | $GREEN$pass_count pass${RESET}${BOLD} | ${YELLOW}$warn_count warn${RESET}${BOLD} | ${RED}$fail_count fail${RESET}${BOLD} ===${RESET}"
echo ""

if [[ $fail_count -gt 0 ]]; then
  echo -e "  ${RED}Connector checks failed — review FAIL items.${RESET}"
  exit 1
elif [[ $warn_count -gt 0 ]]; then
  echo -e "  ${YELLOW}Connector functional with warnings.${RESET}"
  exit 0
else
  echo -e "  ${GREEN}Connector healthy — ready to orchestrate the fleet.${RESET}"
  exit 0
fi
