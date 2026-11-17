#!/usr/bin/env bash
# sigma_hub.sh — SigmaOS component-repository hub.
#
# The SigmaOS integration repo is the connector for the component fleet:
# it owns the registry (COMPONENTS.toml + components/*.toml), the scaffold
# template, and this shell interface used to inventory, connect, create,
# upgrade, sync, and publish component repositories.
#
# Usage:
#   sigma_hub.sh list [--category C] [--local] [--remote] [--missing]
#   sigma_hub.sh status [name]
#   sigma_hub.sh audit [--github]
#   sigma_hub.sh connect <name> [--submodule]
#   sigma_hub.sh create <name> [--desc D] [--category C] [--tier N]
#                         [--provides a,b] [--depends-on x,y] [--dry-run]
#   sigma_hub.sh upgrade <name|--all> [--dry-run]
#   sigma_hub.sh sync [--pull|--push] [name ...]
#   sigma_hub.sh push [name|--all]
#   sigma_hub.sh search <regex>        # pacman -Ss / apt search
#   sigma_hub.sh info <name>           # apt show / pacman -Qi
#   sigma_hub.sh depends <name>        # apt-cache depends
#
# PRINCIPLES: MODULARITY, SINGLE RESPONSIBILITY, DRY, KISS, SEPARATION OF
# CONCERNS, UNIFORM INFORMATION, OPEN/CLOSED, DEFENSIVE PROGRAMMING.

set -euo pipefail

HUB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/component.sh
source "${HUB_DIR}/lib/component.sh"

DRY_RUN=0
SUBMODULE=0
ONLY_LOCAL=0
ONLY_REMOTE=0
ONLY_MISSING=0
CATEGORY_FILTER=""
DO_PULL=1
DO_PUSH=0
# create-specific options
OPT_DESC="" OPT_CAT="" OPT_TIER="" OPT_PROVIDES="" OPT_DEPENDS="" OPT_ENGINE=""
TARGETS=()

# --- arg parsing -------------------------------------------------------------
parse_common() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            --dry-run) DRY_RUN=1; shift ;;
            --submodule) SUBMODULE=1; shift ;;
            --local) ONLY_LOCAL=1; shift ;;
            --remote) ONLY_REMOTE=1; shift ;;
            --missing) ONLY_MISSING=1; shift ;;
            --category|--cat) CATEGORY_FILTER="${2:-}"; shift 2 ;;
            --desc) OPT_DESC="${2:-}"; shift 2 ;;
            --cat) OPT_CAT="${2:-}"; shift 2 ;;
            --tier) OPT_TIER="${2:-}"; shift 2 ;;
            --provides) OPT_PROVIDES="${2:-}"; shift 2 ;;
            --depends-on) OPT_DEPENDS="${2:-}"; shift 2 ;;
            --engine) OPT_ENGINE="${2:-}"; shift 2 ;;
            --pull) DO_PULL=1; DO_PUSH=0; shift ;;
            --push) DO_PUSH=1; DO_PULL=0; shift ;;
            -h|--help) usage; exit 0 ;;
            *) TARGETS+=("$1"); shift ;;
        esac
    done
}

usage() {
    sed -n '2,22p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
}

# --- helpers -----------------------------------------------------------------
# Iterate declared components, honoring filters. Calls: emit <name> <repo> <cat> <lang>
iter_components() {
    components_all | while IFS=$'\t' read -r name repo engine cat lang vis deps; do
        [[ -z "$name" ]] && continue
        if [[ -n "$CATEGORY_FILTER" && "$cat" != "$CATEGORY_FILTER" ]]; then continue; fi
        if [[ $ONLY_LOCAL -eq 1 ]]; then
            component_local_exists "$name" || continue
        fi
        if [[ $ONLY_MISSING -eq 1 ]]; then
            component_local_exists "$name" && continue
        fi
        printf '%s\t%s\t%s\t%s\n' "$name" "$repo" "$cat" "$lang"
    done
}

local_status() {  # name -> one-word state
    local name="$1"
    if ! component_local_exists "$name"; then echo "absent"; return; fi
    if ! component_local_clean "$name"; then echo "dirty"; return; fi
    local up; up="$(component_unpushed "$name")"
    if [[ "$up" == "0" ]]; then echo "clean"; else echo "ahead+$up"; fi
}

# --- subcommands -------------------------------------------------------------
cmd_list() {
    require_cmd python3
    printf '%-38s %-14s %-22s %s\n' "COMPONENT" "CATEGORY" "REPO" "LOCAL"
    iter_components | while IFS=$'\t' read -r name repo cat lang; do
        local st="-"
        component_local_exists "$name" && st="$(local_status "$name")"
        printf '%-38s %-14s %-22s %s\n' "$name" "$cat" "$(basename "$repo" .git)" "$st"
    done
}

cmd_status() {
    if [[ ${#TARGETS[@]} -eq 0 ]]; then
        # fleet summary — local-only (no per-repo network calls)
        local total=0 local_n=0 dirty_n=0 ahead_n=0 absent_meta=0
        while IFS=$'\t' read -r name repo cat lang; do
            total=$((total+1))
            if component_local_exists "$name"; then
                local_n=$((local_n+1))
                local st; st="$(local_status "$name")"
                [[ "$st" == "dirty"* || "$st" == "ahead+"* ]] && dirty_n=$((dirty_n+1))
                [[ "$st" == ahead+* ]] && ahead_n=$((ahead_n+1))
                [[ ! -f "$(component_local_path "$name")/COMPONENT.toml" ]] && absent_meta=$((absent_meta+1))
            fi
        done < <(iter_components)
        hub_ok "declared=$total local=$local_n unpushed_or_dirty=$dirty_n (ahead=$ahead_n) missing_COMPONENT.toml=$absent_meta"
        hub_info "fleet_dir=$SIGMA_FLEET_DIR  (pass a name for per-repo detail + github check)"
        return
    fi
    for name in "${TARGETS[@]}"; do
        local row; row="$(component_get "$name")"
        if [[ -z "$row" ]]; then hub_err "$name: not in COMPONENTS.toml"; continue; fi
        IFS=$'\t' read -r cname repo engine cat lang vis deps <<<"$row"
        local path; path="$(component_local_path "$name")"
        printf '\n%s\n' "$name"
        printf '  repo:    %s\n' "$repo"
        printf '  path:    %s\n' "$path"
        printf '  local:   %s\n' "$(component_local_exists "$name" && echo yes || echo no)"
        [[ -d "$path/.git" ]] && {
            printf '  branch:  %s\n' "$(git -C "$path" rev-parse --abbrev-ref HEAD 2>/dev/null)"
            printf '  status:  %s\n' "$(local_status "$name")"
            printf '  remote:  %s\n' "$(git -C "$path" remote get-url origin 2>/dev/null || echo none)"
        }
        printf '  github:  %s\n' "$(github_repo_exists "$(basename "$repo" .git)" && echo exists || echo missing)"
        printf '  meta:    %s\n' "$([[ -f "$path/COMPONENT.toml" ]] && echo COMPONENT.toml || echo MISSING)"
    done
}

cmd_audit() {
    require_cmd python3
    local total=0 with_meta=0 without_meta=0
    local arr=()
    while IFS=$'\t' read -r name repo cat lang; do
        total=$((total+1))
        local path; path="$(component_local_path "$name")"
        if [[ -f "$path/COMPONENT.toml" ]]; then with_meta=$((with_meta+1)); else without_meta=$((without_meta+1)); arr+=("$name"); fi
    done < <(iter_components)
    hub_ok "declared=$total with_COMPONENT.toml=$with_meta missing=$without_meta"
    if [[ ${#arr[@]} -gt 0 ]]; then
        hub_info "missing COMPONENT.toml (upgrade candidates):"
        printf '  %s\n' "${arr[@]:0:40}"
        [[ ${#arr[@]} -gt 40 ]] && hub_info "  ... and $(( ${#arr[@]} - 40 )) more"
    fi
}

cmd_connect() {
    [[ ${#TARGETS[@]} -eq 0 ]] && hub_die "connect requires a component name"
    require_cmd git
    for name in "${TARGETS[@]}"; do
        local row; row="$(component_get "$name")"
        [[ -z "$row" ]] && { hub_err "$name: not declared"; continue; }
        IFS=$'\t' read -r cname repo engine cat lang vis deps <<<"$row"
        local path; path="$(component_local_path "$name")"
        if component_local_exists "$name"; then
            hub_warn "$name already present at $path"
        else
            hub_info "cloning $repo -> $path"
            if [[ $DRY_RUN -eq 0 ]]; then
                git clone --quiet "$repo" "$path" || { hub_err "clone failed: $repo"; continue; }
                hub_ok "cloned $name"
            fi
        fi
        if [[ $SUBMODULE -eq 1 && $DRY_RUN -eq 0 && -d "$SIGMA_HUB_ROOT/.git" ]]; then
            local sub="src_modules/$name"
            mkdir -p "$SIGMA_HUB_ROOT/src_modules"
            if ! git -C "$SIGMA_HUB_ROOT" config -f .gitmodules "submodule.$sub.url" "$repo" 2>/dev/null; then
                git -C "$SIGMA_HUB_ROOT" submodule add "$repo" "$sub" || true
            fi
        fi
    done
}

# Render the component template into a directory.
render_template() {
    local dest="$1" name="$2" desc="$3" cat="$4" tier="$5" provides="$6" depends="$7" engine="$8"
    local crate_lib; crate_lib="$(printf '%s' "$name" | tr 'A-Z-' 'a-z_')"
    local year; year="$(date +%Y)"
    # TOML-quoted variants for the [contract] arrays; plain for README.
    local provides_q depends_q
    provides_q="$(printf '%s' "$provides" | tr ',' '\n' | sed 's/^ *//;s/ *$//' | sed '/^$/d' | sed 's/.*/"&"/' | paste -sd', ' -)"
    depends_q="$(printf '%s' "$depends" | tr ',' '\n' | sed 's/^ *//;s/ *$//' | sed '/^$/d' | sed 's/.*/"&"/' | paste -sd', ' -)"
    [[ -z "$provides_q" ]] && provides_q='"component"'
    [[ -d "$SIGMA_TEMPLATE_DIR" ]] || hub_die "template dir missing: $SIGMA_TEMPLATE_DIR"
    # escape sed replacement metacharacters (&, /, \) in every substituted value
    local e_name e_desc e_cat e_tier e_prov e_dep e_eng e_provlist
    e_name="$(printf '%s' "$name" | sed 's/[&/\\]/\\&/g')"
    e_desc="$(printf '%s' "$desc" | sed 's/[&/\\]/\\&/g')"
    e_cat="$(printf '%s' "$cat" | sed 's/[&/\\]/\\&/g')"
    e_tier="$(printf '%s' "$tier" | sed 's/[&/\\]/\\&/g')"
    e_prov="$(printf '%s' "$provides_q" | sed 's/[&/\\]/\\&/g')"
    e_dep="$(printf '%s' "$depends_q" | sed 's/[&/\\]/\\&/g')"
    e_eng="$(printf '%s' "$engine" | sed 's/[&/\\]/\\&/g')"
    e_provlist="$(printf '%s' "$provides" | sed 's/[&/\\]/\\&/g')"
    ( cd "$SIGMA_TEMPLATE_DIR" && find . -type f ) | while read -r f; do
        local rel="${f#./}"
        local out="$dest/$rel"
        mkdir -p "$(dirname "$out")"
        sed -e "s/__COMPONENT_NAME__/$e_name/g" \
            -e "s/__COMPONENT_DESC__/$e_desc/g" \
            -e "s/__COMPONENT_CATEGORY__/$e_cat/g" \
            -e "s/__COMPONENT_TIER__/$e_tier/g" \
            -e "s/__COMPONENT_PROVIDES__/$e_prov/g" \
            -e "s/__COMPONENT_DEPENDS__/$e_dep/g" \
            -e "s/__COMPONENT_ENGINE__/$e_eng/g" \
            -e "s/__COMPONENT_PROVIDES_LIST__/$e_provlist/g" \
            -e "s/__COMPONENT_CRATE__/$crate_lib/g" \
            -e "s/__COMPONENT_CRATE_LIB__/${crate_lib}_lib/g" \
            -e "s/__GITHUB_ORG__/$SIGMA_GITHUB_ORG/g" \
            -e "s/__COMPONENT_YEAR__/$year/g" \
            "$SIGMA_TEMPLATE_DIR/$f" > "$out"
    done
}

# Append a [[component]] block to COMPONENTS.toml if absent.
registry_append() {
    local name="$1" desc="$2" cat="$3" lang="$4" engine="$5" repo="$6" provides="$7" depends="$8"
    python3 - "$SIGMA_COMPONENTS_TOML" "$name" "$desc" "$cat" "$lang" "$engine" "$repo" "$provides" "$depends" <<'PY'
import sys, re
toml, name, desc, cat, lang, engine, repo, provides, depends = sys.argv[1:10]
src = open(toml).read()
if re.search(rf'^name\s*=\s*"{re.escape(name)}"', src, re.M):
    print("exists"); sys.exit(0)
prov = ", ".join(f'"{p.strip()}"' for p in provides.split(",") if p.strip()) or '"component"'
dep  = ", ".join(f'"{p.strip()}"' for p in depends.split(",") if p.strip())
block = f'''
[[component]]
name = "{name}"
description = "{desc}"
repo = "{repo}"
engine = "{engine}"
category = "{cat}"
language = "{lang}"
inspirations = ["native"]
visibility = "private"
depends_on = [{dep}]
provides = [{prov}]
'''
open(toml, "a").write(block)
print("added")
PY
}

cmd_create() {
    [[ ${#TARGETS[@]} -eq 0 ]] && hub_die "create requires a component name"
    require_cmd python3 git sed date
    local name="${TARGETS[0]}"
    local desc="${OPT_DESC:-SigmaOS $name component}"
    local cat="${OPT_CAT:-${CATEGORY_FILTER:-System Services}}"
    local tier="${OPT_TIER:-2}"
    local provides="${OPT_PROVIDES:-component}"
    local depends="${OPT_DEPENDS:-}"
    local engine="${OPT_ENGINE:-${name#SigmaOS-}}"
    local repo="https://github.com/${SIGMA_GITHUB_ORG}/${name}.git"
    local path; path="$(component_local_path "$name")"

    if component_exists "$name"; then hub_warn "$name already declared in COMPONENTS.toml"; fi
    if component_local_exists "$name"; then hub_warn "$name already exists at $path"; fi

    hub_info "create $name (tier=$tier cat=$cat engine=$engine)"
    hub_info "  repo=$repo"
    hub_info "  path=$path"

    if [[ $DRY_RUN -eq 1 ]]; then
        hub_ok "dry-run: would scaffold template + register + init git"
        return
    fi

    mkdir -p "$path"
    render_template "$path" "$name" "$desc" "$cat" "$tier" "$provides" "$depends" "$engine"
    ( cd "$path" && git init -q -b main && git add -A && \
      git -c user.name="SigmaOS Bot" -c user.email="sigmaos@localhost" \
          commit -qm "chore: scaffold $name component" )
    git -C "$path" remote add origin "$repo" 2>/dev/null || git -C "$path" remote set-url origin "$repo"
    hub_ok "scaffolded $name at $path"

    local reg; reg="$(registry_append "$name" "$desc" "$cat" rust "$engine" "$repo" "$provides" "$depends")"
    [[ "$reg" == "added" ]] && hub_ok "registered $name in COMPONENTS.toml" || hub_info "$name already in COMPONENTS.toml"

    if github_auth_ok; then
        hub_info "creating GitHub repo ${SIGMA_GITHUB_ORG}/${name}"
        gh repo create "${SIGMA_GITHUB_ORG}/${name}" --private --source="$path" --remote=origin --push 2>/dev/null \
            && hub_ok "pushed $name to GitHub" || hub_warn "gh repo create failed (rate limit / perms) — push manually"
    else
        hub_warn "gh not authed — repo created locally; push with: git -C $path push -u origin main"
    fi
}

cmd_upgrade() {
    require_cmd python3 git sed date
    local targets=("${TARGETS[@]}")
    [[ ${#targets[@]} -eq 0 || "${targets[0]}" == "--all" ]] && {
        targets=()
        while IFS=$'\t' read -r name repo cat lang; do targets+=("$name"); done < <(iter_components)
    }
    local upgraded=0 skipped=0
    for name in "${targets[@]}"; do
        local path; path="$(component_local_path "$name")"
        [[ -d "$path" ]] || { hub_warn "$name: no local checkout, skip"; skipped=$((skipped+1)); continue; }
        # resolve declared language (registry) — fall back to repo contents
        local row lang="rust" cat="System Services"
        row="$(component_get "$name")"
        if [[ -n "$row" ]]; then IFS=$'\t' read -r _ _ _ cat lang _ _ <<<"$row"; fi
        local is_rust=0
        [[ "$lang" == "rust" || -f "$path/Cargo.toml" ]] && is_rust=1
        local missing=()
        [[ -f "$path/COMPONENT.toml" ]] || missing+=("COMPONENT.toml")
        [[ -f "$path/README.md" ]]    || missing+=("README.md")
        [[ -f "$path/LICENSE" ]]      || missing+=("LICENSE")
        compgen -G "$path/.github/workflows/*.yml" >/dev/null || missing+=("CI")
        if [[ $is_rust -eq 1 ]]; then
            [[ -f "$path/Cargo.toml" ]] || missing+=("Cargo.toml")
            compgen -G "$path/tests/*.rs" >/dev/null || missing+=("tests")
        else
            [[ -d "$path/tests" || -d "$path/test" ]] || missing+=("tests")
        fi
        if [[ ${#missing[@]} -eq 0 ]]; then skipped=$((skipped+1)); continue; fi
        hub_info "$name ($lang): missing ${missing[*]}"
        if [[ $DRY_RUN -eq 1 ]]; then upgraded=$((upgraded+1)); continue; fi
        # scaffold only the missing pieces from the template (non-destructive)
        local desc engine tier provides depends
        desc="$(component_file_field "$path/COMPONENT.toml" component description 2>/dev/null)"; desc="${desc:-${name#SigmaOS-} component}"
        engine="$(component_file_field "$path/COMPONENT.toml" component name 2>/dev/null)"; engine="${engine:-${name#SigmaOS-}}"
        tier="$(component_file_field "$path/COMPONENT.toml" component tier 2>/dev/null)"; tier="${tier:-2}"
        provides="$(component_file_field "$path/COMPONENT.toml" contract provides 2>/dev/null)"; provides="${provides:-component}"
        depends="$(component_file_field "$path/COMPONENT.toml" contract depends_on 2>/dev/null)"
        # render full template to a temp dir then copy only missing files
        local tmp; tmp="$(mktemp -d)"
        render_template "$tmp" "$name" "$desc" "$cat" "$tier" "$provides" "$depends" "$engine"
        local files=(COMPONENT.toml README.md LICENSE .github/workflows/component-ci.yml)
        [[ $is_rust -eq 1 ]] && files+=(.gitignore Cargo.toml src/lib.rs src/main.rs tests/integration_test.rs)
        for f in "${files[@]}"; do
            if [[ ! -e "$path/$f" && -e "$tmp/$f" ]]; then
                mkdir -p "$path/$(dirname "$f")"; cp "$tmp/$f" "$path/$f"
            fi
        done
        # non-rust components still get a generic tests/ dir marker
        if [[ $is_rust -eq 0 && ! -d "$path/tests" && ! -d "$path/test" ]]; then
            mkdir -p "$path/tests"; printf '// Integration tests for %s\n' "$name" > "$path/tests/integration_test.rs"
        fi
        rm -rf "$tmp"
        if [[ -d "$path/.git" ]]; then
            git -C "$path" add -A
            git -C "$path" -c user.name="SigmaOS Bot" -c user.email="sigmaos@localhost" \
                commit -qm "chore: upgrade $name to component standard" || true
        fi
        upgraded=$((upgraded+1)); hub_ok "$name upgraded"
    done
    hub_ok "upgrade complete: $upgraded upgraded, $skipped already-standard/absent"
}

# families — cluster local SigmaOS-* repos by name-prefix family and report
# scaffold vs real-code counts. Supports CONSOLIDATE-GITHUB-REPOS-OF-SIMILAR-
# COMPONENTS: identical-prefix scaffold fleets are consolidation candidates,
# not candidates for blind per-repo metadata upgrades.
cmd_families() {
    require_cmd python3
    python3 - "$SIGMA_FLEET_DIR" <<'PY'
import os, sys, re, collections
root = sys.argv[1]
fam = collections.defaultdict(lambda: [0,0])  # family -> [repos, with_real_code]
for d in os.listdir(root):
    if not d.startswith("SigmaOS-") or not os.path.isdir(os.path.join(root,d,".git")):
        continue
    rest = d[len("SigmaOS-"):]
    # family = leading alpha prefix up to first digit or '-' boundary groups
    m = re.match(r"^([a-zA-Z]+)", rest)
    key = (m.group(1).lower() if m else rest.split("-")[0].lower())
    # refine: keep known multi-word families
    for pref in ("archlinux","system-feature","omarchy","mint","arch-"):
        if rest.lower().startswith(pref): key = pref.rstrip("-"); break
    src = os.path.join(root,d,"src")
    size = 0
    if os.path.isdir(src):
        for r,_,fs in os.walk(src):
            for f in fs:
                if f.endswith(".rs"): size += os.path.getsize(os.path.join(r,f))
    fam[key][0]+=1
    if size>20000: fam[key][1]+=1
print(f"{'FAMILY':<22} {'REPOS':>6} {'REALCODE':>8}")
for k,(n,real) in sorted(fam.items(), key=lambda x:-x[1][0]):
    print(f"{k:<22} {n:>6} {real:>8}")
print(f"\nTotal families={len(fam)} repos={sum(v[0] for v in fam.values())} realcode={sum(v[1] for v in fam.values())}")
PY
}

# consolidate [family ...] — dry-run merge plan for scaffold fleets.
# For each prefix family, picks the canonical parent (exact-name repo or the
# family root, e.g. SigmaOS-omarchy) and lists scaffold members that would be
# merged into it. NON-DESTRUCTIVE: prints the plan only; apply with --apply.
cmd_consolidate() {
    require_cmd python3
    python3 - "$SIGMA_FLEET_DIR" "$@" <<'PY'
import os, sys, re, collections
root, args = sys.argv[1], sys.argv[2:]
want = set(a.lower() for a in args)
fam = collections.defaultdict(list)
for d in sorted(os.listdir(root)):
    if not d.startswith("SigmaOS-") or not os.path.isdir(os.path.join(root,d,".git")): continue
    rest = d[len("SigmaOS-"):]
    key=None
    for pref in ("archlinux","system-feature","omarchy","mint","arch-"):
        if rest.lower().startswith(pref): key=pref.rstrip("-"); break
    if key is None:
        m=re.match(r"^([a-zA-Z]+)",rest); key=(m.group(1).lower() if m else rest.split("-")[0].lower())
    if want and key not in want: continue
    src=os.path.join(root,d,"src"); size=0
    if os.path.isdir(src):
        for r,_,fs in os.walk(src):
            for f in fs:
                if f.endswith(".rs"): size+=os.path.getsize(os.path.join(r,f))
    fam[key].append((d,size))
print("CONSOLIDATION PLAN (dry-run — nothing deleted)\n")
for key in sorted(fam, key=lambda k:-len(fam[k])):
    members=fam[key]
    if len(members)<3: continue
    # canonical = exact family-name repo if present else largest real-code repo
    canon=None
    for pref in (f"SigmaOS-{key}", f"SigmaOS-{key}-"):
        for d,s in members:
            if d==pref or d.startswith(pref) and s>20000: canon=(d,s); break
        if canon: break
    if not canon:
        canon=max(members,key=lambda x:x[1])
    scaffolds=[d for d,s in members if d!=canon[0]]
    print(f"[{key}] canonical={canon[0]} ({canon[1]}B)  merge {len(scaffolds)} scaffold(s):")
    for d in scaffolds[:8]: print(f"    <- {d}")
    if len(scaffolds)>8: print(f"    ... +{len(scaffolds)-8} more")
PY
}

# search <regex> — pacman -Ss / apt search over the component registry.
cmd_search() {
    [[ ${#TARGETS[@]} -eq 0 ]] && hub_die "search requires a pattern"
    local pat="${TARGETS[0]}"
    printf '%-38s %-22s %-10s %s\n' COMPONENT CATEGORY LANG DESCRIPTION
    components_all | while IFS=$'\t' read -r name repo engine cat lang vis dep desc; do
        [[ -z "$name" ]] && continue
        [[ -n "$CATEGORY_FILTER" && "$cat" != "$CATEGORY_FILTER" ]] && continue
        [[ "$name$desc$cat" =~ $pat ]] || continue
        printf '%-38s %-22s %-10s %s\n' "$name" "$cat" "$lang" "${desc:0:60}"
    done
}

# info <name> — apt show / pacman -Qi style full metadata for one component.
cmd_info() {
    [[ ${#TARGETS[@]} -eq 0 ]] && hub_die "info requires a component name"
    local name="${TARGETS[0]}" row
    row="$(component_get "$name")"
    [[ -z "$row" ]] && hub_die "$name not in COMPONENTS.toml"
    local repo engine cat lang vis dep desc
    IFS=$'\t' read -r _ repo engine cat lang vis dep desc <<<"$row"
    local path; path="$(component_local_path "$name")"
    echo "Name:        $name"
    echo "Repository:  $repo"
    echo "Engine:      $engine"
    echo "Category:    $cat"
    echo "Language:    $lang"
    echo "Visibility:  $vis"
    [[ "$dep" == "-" ]] && dep=""
    echo "Depends on:  ${dep:- (none)}"
    echo "Local path:  $path $([[ -d "$path" ]] && echo '[present]' || echo '[absent]')"
    [[ -n "$desc" ]] && echo "Description: $desc"
    if [[ -f "$path/COMPONENT.toml" ]]; then
        echo "--- COMPONENT.toml ---"
        cat "$path/COMPONENT.toml"
    fi
}

# depends <name> — apt-cache depends: show declared dependency closure (1 level + count).
cmd_depends() {
    [[ ${#TARGETS[@]} -eq 0 ]] && hub_die "depends requires a component name"
    local name="${TARGETS[0]}" row dep
    row="$(component_get "$name")"
    [[ -z "$row" ]] && hub_die "$name not in COMPONENTS.toml"
    IFS=$'\t' read -r _ _ _ _ _ _ dep _ <<<"$row"
    [[ "$dep" == "-" || -z "$dep" ]] && { echo "$name depends on: (none)"; return; }
    echo "$name depends on:"
    local d
    IFS=',' read -ra arr <<<"$dep"
    for d in "${arr[@]}"; do
        d="${d// /}"
        local st="?"; component_exists "$d" && st="ok"
        local p; p="$(component_local_path "$d")"; [[ -d "$p" ]] && st="$st+local"
        printf '  %-40s %s\n' "$d" "$st"
    done
}

cmd_sync() {
    require_cmd git
    local targets=("${TARGETS[@]}")
    [[ ${#targets[@]} -eq 0 ]] && {
        targets=()
        while IFS=$'\t' read -r name repo cat lang; do targets+=("$name"); done < <(iter_components)
    }
    local pulled=0 pushed=0 failed=0
    for name in "${targets[@]}"; do
        local path; path="$(component_local_path "$name")"
        [[ -d "$path/.git" ]] || continue
        if [[ $DO_PULL -eq 1 ]]; then
            if git -C "$path" pull --ff-only -q 2>/dev/null; then pulled=$((pulled+1)); else failed=$((failed+1)); fi
        fi
        if [[ $DO_PUSH -eq 1 ]]; then
            local up; up="$(component_unpushed "$name")"
            if [[ "$up" != "0" && "$up" != "-1" ]]; then
                git -C "$path" push -q 2>/dev/null && pushed=$((pushed+1)) || failed=$((failed+1))
            fi
        fi
    done
    hub_ok "sync done: pulled=$pulled pushed=$pushed failed=$failed"
}

# --- main --------------------------------------------------------------------
cmd="${1:-help}"; shift || true
case "$cmd" in
    list|status|audit|connect|create|upgrade|sync|search|info|depends|families|consolidate)
        parse_common "$@"
        "cmd_${cmd}"
        ;;
    push)
        parse_common "$@"; cmd="sync"; DO_PULL=0; DO_PUSH=1; TARGETS=("${TARGETS[@]}"); cmd_sync ;;
    help|-h|--help) usage ;;
    *) hub_die "unknown command: $cmd (try 'sigma_hub.sh help')" ;;
esac
