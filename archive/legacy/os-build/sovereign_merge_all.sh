#!/usr/bin/env bash
# Merge all remaining unmerged remote branches using sovereign strategy
set -euo pipefail
REPO="/home/aaryansinghchauhan/SigmaOS"
cd "$REPO"

# Branches already merged by script run (becb99f2 was just done)
# We'll merge: jules-14008722679885411662-c59076f7, jules-2710202376188178750-b9534c0e,
# jules-3908154892251844112-833ff418, jules-3966115662251218635-d7895a73,
# jules-5796990581539821167-ae6f5b6a (already merged in prev session but still listed),
# jules-9488094871273210021-eb4d9170, palette, sentinel, universal-enhancements, universal-improvements
# Ones previously merged: feat/tech-media, feat-universal, feature/ultra, feature-universal-pm,
# fix-gh-actions, jules-3908, jules-3966, jules-5796, jules-9488, jules-open-source
# New ones from this session: jules-14008, jules-2710, universal-enhancements, universal-improvements

LEGACY_WORKFLOWS=(
  ".github/workflows/alpine-abuild-apk-ci.yml"
  ".github/workflows/arch-namcap-aur-audit-ci.yml"
  ".github/workflows/cachyos-kernel-bore-ci.yml"
  ".github/workflows/debian-autopkgtest-ci.yml"
  ".github/workflows/fedora-dnf5-advisory-ci.yml"
  ".github/workflows/freebsd-poudriere-ports-ci.yml"
  ".github/workflows/freebsd-poudriere-vuxml-ci.yml"
  ".github/workflows/gentoo-catalyst-stage3-ci.yml"
  ".github/workflows/nixos-flake-devshell-ci.yml"
  ".github/workflows/nixos-hydra-eval-ci.yml"
  ".github/workflows/omarchy-quickshell-hyprland-ci.yml"
  ".github/workflows/openbsd-signify-pledge-audit-ci.yml"
  ".github/workflows/openbsd-syspatch-pledge-ci.yml"
  ".github/workflows/opensuse-obs-kiwi-ci.yml"
)

OURS_FILES=(
  "Cargo.toml" "src/lib.rs" "src/package/universal.rs"
  "src/sigpkg/universal_adapter.rs" "src/sigpkg/universal_engine.rs"
  "src/distro/linux_bsd_distro_gaps.rs" "src/distro/omarchy.rs"
  "src/distro/void_runit.rs" "src/filesystem/manager.rs" "src/klib/btreemap.rs"
  "src/package/bsd_linux_package_innovations.rs" "src/package/sigma_pkg.rs"
  ".github/workflows/06_build-optimization-matrix.yml"
  ".github/workflows/09_storage-performance-matrix.yml"
  ".github/workflows/sigma_multiarch_ci.yml"
)

THEIRS_FILES=(
  "src/sigpkg/universal_oop_system.rs"
  "tests/test_universal_adapter.rs"
  "src/open_source_obsoletion.rs"
  "src/open_source_os_gap_closure.rs"
  "src/compatibility/mod.rs"
  "src/security/mod.rs"
  "src/tools/editor.rs"
)

do_resolve() {
  echo "  → Resolving conflicts..."
  # Handle modify/delete conflicts (legacy workflows we intentionally deleted)
  git status --porcelain | grep "^DU\|^UD" | awk '{print $2}' | while read f; do
    git rm -f "$f" 2>/dev/null || true
    echo "  Removed (modify/delete): $f"
  done
  # Also remove any resurrected legacy workflows
  for f in "${LEGACY_WORKFLOWS[@]}"; do
    if git diff --name-only --diff-filter=U 2>/dev/null | grep -qF "$f"; then
      git checkout --ours "$f" 2>/dev/null || git rm -f "$f" 2>/dev/null || true
      git add "$f" 2>/dev/null || true
    fi
    if [ -f "$f" ]; then
      rm -f "$f"; git rm --cached "$f" 2>/dev/null || true
    fi
  done

  # Sacred files → ours
  for f in "${OURS_FILES[@]}"; do
    if git diff --name-only --diff-filter=U 2>/dev/null | grep -qF "$f"; then
      git checkout --ours "$f" && git add "$f"
      echo "  Kept ours: $f"
    fi
  done

  # Enhancement files → theirs
  for f in "${THEIRS_FILES[@]}"; do
    if git diff --name-only --diff-filter=U 2>/dev/null | grep -qF "$f"; then
      git checkout --theirs "$f" && git add "$f"
      echo "  Took theirs: $f"
    fi
  done

  # mod.rs → combine
  if git diff --name-only --diff-filter=U 2>/dev/null | grep -qF "src/distro/mod.rs"; then
    python3 - <<'PYEOF'
with open("src/distro/mod.rs", "r") as f:
    content = f.read()
if '<<<<<<<' in content:
    lines = content.split('\n')
    result, in_head, in_theirs = [], True, False
    for line in lines:
        if line.startswith('<<<<<<<'):
            in_head = True; in_theirs = False
        elif line.strip() == '=======' and in_head:
            in_head = False; in_theirs = True
        elif line.startswith('>>>>>>>') and in_theirs:
            in_theirs = False
        else:
            result.append(line)
    with open("src/distro/mod.rs", "w") as f:
        f.write('\n'.join(result))
    print("  mod.rs combined:", len(result), "lines")
PYEOF
    git add "src/distro/mod.rs"
  fi

  # Any remaining conflicts → ours
  REMAINING=$(git diff --name-only --diff-filter=U 2>/dev/null || true)
  if [ -n "$REMAINING" ]; then
    echo "  Remaining (ours): $REMAINING"
    echo "$REMAINING" | while read f; do
      git checkout --ours "$f" 2>/dev/null && git add "$f" 2>/dev/null || true
    done
  fi

  git add -A
  git commit --no-edit 2>&1 || git commit -m "merge: resolve sovereign conflicts" 2>&1
}

merge_one() {
  local branch="$1"
  echo ""
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  echo "Merging: origin/$branch"
  echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
  if git merge --no-ff -m "merge: integrate $branch with sovereign OS advancements" \
      "origin/$branch" 2>&1; then
    echo "✓ Clean merge"
  else
    do_resolve
    echo "✓ Merged (conflicts resolved)"
  fi
}

# List of branches to merge (skipping already-merged ones from prev sessions)
# We'll try all and git will skip already-merged ones (up-to-date)
BRANCHES=(
  "jules-14008722679885411662-c59076f7"
  "jules-2710202376188178750-b9534c0e"
  "jules-3908154892251844112-833ff418"
  "jules-3966115662251218635-d7895a73"
  "jules-5796990581539821167-ae6f5b6a"
  "jules-9488094871273210021-eb4d9170"
  "jules-open-source-parity-enhancements-6712125693013811941"
  "palette-escape-key-overlay-dismissal-8663269476301815275"
  "sentinel/fix-control-char-input-validation-9630806104785257808"
  "universal-package-system-enhancements-11366958273359734418"
  "universal-package-system-improvements-5325325354119513414"
  "feat/tech-media-inspirations-enhancement-9554947991337263290"
  "feat-universal-package-manager-distro-parity-3836969524529494079"
  "feature/ultra-encyclopedia-v26-9381808247070751897"
  "feature-universal-pm-distro-innovations-6712866605131190313"
  "fix-gh-actions-pinning-and-warnings-9579796699483565200"
  "jules-2585223580442800853-becb99f2"
)

for b in "${BRANCHES[@]}"; do
  merge_one "$b"
done

echo ""
echo "All branches processed ✓"
