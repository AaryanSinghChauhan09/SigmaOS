#!/usr/bin/env bash
# Sovereign merge script — resolves all remote branches into main
# Strategy: --ours on sacred files, --theirs on enhancement files, Python combine on mod.rs
set -euo pipefail

REPO="/home/aaryansinghchauhan/SigmaOS"
cd "$REPO"

OURS_FILES=(
  "Cargo.toml"
  "src/lib.rs"
  "src/package/universal.rs"
  "src/sigpkg/universal_adapter.rs"
  "src/sigpkg/universal_engine.rs"
  "src/distro/linux_bsd_distro_gaps.rs"
  "src/distro/omarchy.rs"
  "src/distro/void_runit.rs"
  "src/filesystem/manager.rs"
  "src/klib/btreemap.rs"
  "src/package/bsd_linux_package_innovations.rs"
  "src/package/sigma_pkg.rs"
)

THEIRS_FILES=(
  "src/sigpkg/universal_oop_system.rs"
  "tests/test_universal_adapter.rs"
  "src/open_source_obsoletion.rs"
  "src/open_source_os_gap_closure.rs"
)

merge_branch() {
  local branch="$1"
  echo "━━━ Merging: $branch ━━━"
  
  # Attempt merge
  if git merge --no-ff -m "merge: integrate $branch with sovereign OS advancements" \
    "origin/$branch" 2>&1; then
    echo "✓ Clean merge: $branch"
    return 0
  fi

  echo "  → Conflicts detected, applying sovereign strategy..."

  # Apply --ours to sacred files that exist and are conflicted
  for f in "${OURS_FILES[@]}"; do
    if [ -f "$f" ] && git diff --name-only --diff-filter=U | grep -qF "$f"; then
      git checkout --ours "$f"
      git add "$f"
    fi
  done

  # Apply --theirs to enhancement files that exist and are conflicted
  for f in "${THEIRS_FILES[@]}"; do
    if [ -f "$f" ] && git diff --name-only --diff-filter=U | grep -qF "$f"; then
      git checkout --theirs "$f"
      git add "$f"
    fi
  done

  # Combine strategy for src/distro/mod.rs
  if git diff --name-only --diff-filter=U | grep -qF "src/distro/mod.rs"; then
    python3 - <<'PYEOF'
with open("src/distro/mod.rs", "r") as f:
    content = f.read()

if '<<<<<<<' not in content:
    print("  mod.rs: no conflict markers, skipping")
    exit(0)

lines = content.split('\n')
result = []
in_head = True
in_theirs = False

for line in lines:
    if line.startswith('<<<<<<<'):
        in_head = True
        in_theirs = False
    elif line.strip() == '=======' and in_head:
        in_head = False
        in_theirs = True
    elif line.startswith('>>>>>>>') and in_theirs:
        in_theirs = False
    else:
        result.append(line)

with open("src/distro/mod.rs", "w") as f:
    f.write('\n'.join(result))
print("  mod.rs combined successfully, lines:", len(result))
PYEOF
    git add "src/distro/mod.rs"
  fi

  # For any remaining conflicts — use --ours (safe default)
  remaining=$(git diff --name-only --diff-filter=U 2>/dev/null || true)
  if [ -n "$remaining" ]; then
    echo "  → Remaining conflicts (using --ours): $remaining"
    echo "$remaining" | xargs -I{} git checkout --ours {}
    echo "$remaining" | xargs git add
  fi

  git commit --no-edit 2>&1 || git commit -m "merge: integrate $branch with sovereign OS advancements (conflicts resolved)" 2>&1
  echo "✓ Merged (conflict-resolved): $branch"
}

# Get all remote branches except main
BRANCHES=$(git branch -r | grep -v "HEAD\|/main$" | sed 's|  origin/||' | sort)

echo "Branches to merge: $(echo "$BRANCHES" | wc -l)"
echo "$BRANCHES"
echo ""

for branch in $BRANCHES; do
  merge_branch "$branch"
done

echo ""
echo "All branches merged ✓"
