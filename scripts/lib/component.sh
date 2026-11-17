#!/usr/bin/env bash
# component.sh — shared library for the SigmaOS component hub.
#
# Single source of truth for fleet metadata is COMPONENTS.toml (component
# declarations) and components/SIGMAOS_COMPONENT_REGISTRY.toml (repo ownership).
# All hub subcommands source this file. Keep it POSIX-bash, no external deps
# beyond git, python3 (tomllib), and optionally gh.
#
# PRINCIPLES: MODULARITY (one lib, many commands), DRY (one TOML parser),
# SEPARATION OF CONCERNS (metadata vs git vs github), KISS.

# --- paths -------------------------------------------------------------------
SIGMA_HUB_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SIGMA_COMPONENTS_TOML="${SIGMA_HUB_ROOT}/COMPONENTS.toml"
SIGMA_REGISTRY_TOML="${SIGMA_HUB_ROOT}/components/SIGMAOS_COMPONENT_REGISTRY.toml"
SIGMA_OWNERSHIP_TOML="${SIGMA_HUB_ROOT}/components/MODULE_OWNERSHIP.toml"
SIGMA_GITMODULES="${SIGMA_HUB_ROOT}/.gitmodules"
# Component checkouts live as siblings of the main repo by convention.
SIGMA_FLEET_DIR="$(cd "${SIGMA_HUB_ROOT}/.." && pwd)"
SIGMA_GITHUB_ORG="${SIGMA_GITHUB_ORG:-AaryanSinghChauhan09}"
SIGMA_TEMPLATE_DIR="${SIGMA_HUB_ROOT}/templates/component"

# --- output ------------------------------------------------------------------
if [[ -t 1 ]]; then
    C_RED=$'\033[0;31m'; C_GRN=$'\033[0;32m'; C_YEL=$'\033[1;33m'
    C_BLU=$'\033[0;34m'; C_CYN=$'\033[0;36m'; C_DIM=$'\033[2m'; C_RST=$'\033[0m'
else
    C_RED= C_GRN= C_YEL= C_BLU= C_CYN= C_DIM= C_RST=
fi

hub_log()    { printf '%s[hub]%s %s\n' "$C_BLU" "$C_RST" "$*" >&2; }
hub_ok()     { printf '%s[ ok ]%s %s\n' "$C_GRN" "$C_RST" "$*" >&2; }
hub_warn()   { printf '%s[warn]%s %s\n' "$C_YEL" "$C_RST" "$*" >&2; }
hub_err()    { printf '%s[fail]%s %s\n' "$C_RED" "$C_RST" "$*" >&2; }
hub_info()   { printf '%s[info]%s %s\n' "$C_CYN" "$C_RST" "$*" >&2; }
hub_die()    { hub_err "$*"; exit 1; }

require_cmd() {
    command -v "$1" >/dev/null 2>&1 || hub_die "required command not found: $1"
}

# --- TOML parsing (python3 + tomllib; read-only) ------------------------------
# Emit "key\tvalue" for every [[component]] in COMPONENTS.toml.
# Fields: name repo engine category language visibility depends_on
components_all() {
    python3 - "$SIGMA_COMPONENTS_TOML" <<'PY'
import sys, tomllib
try:
    data = tomllib.load(open(sys.argv[1], "rb"))
except Exception as e:
    sys.stderr.write(f"components_all: {e}\n"); sys.exit(1)
for c in data.get("component", []):
    dep = ",".join(c.get("depends_on", [])) or "-"
    desc = str(c.get("description","")).replace("\t"," ").replace("\n"," ")
    print("\t".join([
        str(c.get("name","")),
        str(c.get("repo","")),
        str(c.get("engine","")),
        str(c.get("category","")),
        str(c.get("language","rust")),
        str(c.get("visibility","private")),
        dep,
        desc,
    ]))
PY
}

# Look up one component by exact name -> same tab-separated fields, or nothing.
# Consumes all input (no early awk exit) so the producer never sees SIGPIPE.
component_get() {
    local want="$1"
    components_all | awk -F'\t' -v w="$want" '$1==w {print}'
}

# True if a component name is declared in COMPONENTS.toml.
component_exists() { [[ -n "$(component_get "$1")" ]]; }

# --- local fleet -------------------------------------------------------------
# Local checkout path for a component name (SigmaOS-foo -> <fleet>/SigmaOS-foo).
component_local_path() { printf '%s/%s\n' "$SIGMA_FLEET_DIR" "$1"; }

component_local_exists() { [[ -d "$(component_local_path "$1")/.git" || -d "$(component_local_path "$1")" ]]; }

component_local_clean() {
    local p; p="$(component_local_path "$1")"
    [[ -d "$p/.git" ]] || return 1
    [[ -z "$(git -C "$p" status --porcelain 2>/dev/null)" ]]
}

# Does the local checkout's origin match the declared repo URL?
component_remote_ok() {
    local name="$1" want="$2" p have
    p="$(component_local_path "$name")"
    [[ -d "$p/.git" ]] || return 1
    have="$(git -C "$p" remote get-url origin 2>/dev/null || true)"
    [[ -n "$have" && "$have" == "$want" ]]
}

# --- git helpers -------------------------------------------------------------
repo_git() { git -C "$1" "${@:2}"; }

# Count commits the local branch is ahead of origin/<branch> (unpushed work).
component_unpushed() {
    local p; p="$(component_local_path "$1")"
    [[ -d "$p/.git" ]] || { echo -1; return; }
    local br; br="$(git -C "$p" rev-parse --abbrev-ref HEAD 2>/dev/null)"
    [[ -n "$br" ]] || { echo -1; return; }
    git -C "$p" rev-list --count "origin/$br..HEAD" 2>/dev/null || echo -1
}

# --- github helpers ----------------------------------------------------------
github_auth_ok() {
    command -v gh >/dev/null 2>&1 && gh auth status >/dev/null 2>&1
}

# Does a GitHub repo exist? Uses gh if authed, else unauthenticated API.
github_repo_exists() {
    local name="$1"
    if github_auth_ok; then
        gh repo view "${SIGMA_GITHUB_ORG}/${name}" >/dev/null 2>&1
    else
        curl -fsS -o /dev/null "https://api.github.com/repos/${SIGMA_GITHUB_ORG}/${name}" 2>/dev/null
    fi
}

# --- component metadata ------------------------------------------------------
# Read a field from a component's COMPONENT.toml (e.g. component_get_field name tier).
component_file_field() {
    local file="$1" section="$2" key="$3"
    python3 - "$file" "$section" "$key" <<'PY'
import sys, tomllib
try:
    d = tomllib.load(open(sys.argv[1], "rb"))
except Exception:
    sys.exit(0)
sec = sys.argv[2]; key = sys.argv[3]
node = d.get(sec, {})
if isinstance(node, dict) and key in node:
    v = node[key]
    if isinstance(v, list): print(",".join(map(str, v)))
    else: print(v)
PY
}
