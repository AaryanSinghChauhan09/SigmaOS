#!/usr/bin/env bash
# automation/scripts/release.sh — Semantic versioning + changelog generation
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

CYAN='\033[36m'; GREEN='\033[32m'; NC='\033[0m'
info() { echo -e "${CYAN}[RELEASE]${NC} $*"; }
ok()   { echo -e "${GREEN}[OK]${NC}     $*"; }

# ── Determine next version ──────────────────────────────────────────────────
PREV_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0")
IFS='.' read -r major minor patch <<< "${PREV_TAG#v}"

# Bump patch by default; bump minor if feat: commit exists; major if BREAKING:
COMMITS=$(git log "${PREV_TAG}..HEAD" --oneline 2>/dev/null || git log --oneline -20)

if echo "$COMMITS" | grep -q "^.*BREAKING"; then
    major=$((major+1)); minor=0; patch=0
elif echo "$COMMITS" | grep -q "^.*feat:"; then
    minor=$((minor+1)); patch=0
else
    patch=$((patch+1))
fi

NEXT_TAG="v${major}.${minor}.${patch}"
info "Previous: $PREV_TAG  →  Next: $NEXT_TAG"

# ── Generate CHANGELOG entry ────────────────────────────────────────────────
CHANGELOG_ENTRY="## $NEXT_TAG — $(date +%Y-%m-%d)\n\n"
while IFS= read -r line; do
    CHANGELOG_ENTRY+="- $line\n"
done <<< "$COMMITS"

# Prepend to CHANGELOG.md
{
    echo -e "$CHANGELOG_ENTRY"
    cat CHANGELOG.md 2>/dev/null || true
} > /tmp/sigma_changelog_tmp.md
mv /tmp/sigma_changelog_tmp.md CHANGELOG.md
ok "CHANGELOG.md updated."

# ── Tag and push ──────────────────────────────────────────────────────────
git config user.name  "sigma-bot"
git config user.email "bot@sigmaos.dev"
git add CHANGELOG.md
git diff --cached --quiet || git commit -m "chore(release): bump to $NEXT_TAG [skip ci]"
git tag -a "$NEXT_TAG" -m "Release $NEXT_TAG"
git push origin main --tags
ok "Tagged and pushed: $NEXT_TAG"
