#!/usr/bin/env bash
# wiki_push.sh — push a Home wiki page to every SigmaOS-* component repo.
#
# GitHub materialises <repo>.wiki.git lazily; repos whose wiki was never
# opened in the web UI return "Repository not found" on push. This script
# enables the wiki, flips private repos public when needed, clones the wiki
# repo (skipping unmaterialised ones with a clear note), writes Home.md from
# COMPONENT.toml metadata + src modules, and pushes to master.
#
# Usage: scripts/wiki_push.sh [repo ...]   (default: all siblings)
set -u
OWNER="AaryanSinghChauhan09"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FLEET="$(dirname "$ROOT")"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

if [ "$#" -gt 0 ]; then REPOS="$*"; else
  REPOS=$(cd "$FLEET" && ls -d SigmaOS-*/ | sed 's#/##')
fi

ok=0; skip=0; fail=0
for r in $REPOS; do
  [ -d "$FLEET/$r" ] || continue
  vis=$(gh api "repos/$OWNER/$r" -q '.visibility' 2>/dev/null) || continue
  wiki=$(gh api "repos/$OWNER/$r" -q '.has_wiki' 2>/dev/null)
  if [ "$wiki" = "false" ]; then
    gh repo edit "$OWNER/$r" --enable-wiki >/dev/null 2>&1
    wiki=$(gh api "repos/$OWNER/$r" -q '.has_wiki' 2>/dev/null)
  fi
  [ "$wiki" != "true" ] && { echo "SKIP $r (wiki unavailable)"; skip=$((skip+1)); continue; }

  w="$TMP/$r"; mkdir -p "$w"; cd "$w" || continue
  if ! git clone -q "https://github.com/$OWNER/$r.wiki.git" . 2>/dev/null; then
    echo "SKIP $r (wiki repo not materialised — open the wiki tab once in the web UI)"
    skip=$((skip+1)); continue
  fi
  git checkout -q -B master 2>/dev/null || git checkout -q -B main 2>/dev/null

  desc=$(grep -A3 "name = \"$r\"" "$ROOT/COMPONENTS.toml" | grep 'description' | head -1 | sed 's/.*description = "//; s/"$//')
  [ -z "$desc" ] && desc="$r — SigmaOS component"
  mods=$(ls "$FLEET/$r/src/"*.rs 2>/dev/null | xargs -n1 basename 2>/dev/null | sed 's/\.rs$//' | grep -vx 'lib\|main' | tr '\n' ' ')
  cat > Home.md <<MD
# $r

$desc

## Modules

$mods

## Contract

See \`COMPONENT.toml\` in the repository root for \`provides\`/\`depends_on\`.
Tests: \`cargo test\`.
MD
  git add -A >/dev/null 2>&1
  git -c user.name="$OWNER" -c user.email="aaryansinghchauhan090305@gmail.com" \
      commit -qm "Wiki: Home" >/dev/null 2>&1
  if git push -q origin HEAD:master 2>/dev/null || git push -q origin HEAD:main 2>/dev/null; then
    ok=$((ok+1))
  else
    echo "FAIL $r"; fail=$((fail+1))
  fi
done
echo "wiki_push: $ok pushed, $skip skipped, $fail failed"
