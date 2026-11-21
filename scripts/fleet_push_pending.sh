#!/usr/bin/env bash
# fleet_push_pending.sh — create missing GitHub remotes for local SigmaOS-*
# checkouts and push them. Safe to re-run: skips repos that already exist
# and repos whose remote is reachable. Handles GitHub repo-creation rate
# limits by simply retrying on the next run.
set -u
FLEET_ROOT="${FLEET_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
ORG="${SIGMA_GITHUB_ORG:-AaryanSinghChauhan09}"

created=0; pushed=0; skipped=0; failed=0
for dir in "$FLEET_ROOT"/SigmaOS-*/; do
    name="$(basename "$dir")"
    [ -d "$dir/.git" ] || continue
    if git -C "$dir" ls-remote origin HEAD >/dev/null 2>&1; then
        skipped=$((skipped+1)); continue
    fi
    url="https://github.com/$ORG/$name.git"
    git -C "$dir" remote remove origin 2>/dev/null
    git -C "$dir" remote add origin "$url"
    if gh repo create "$ORG/$name" --public >/dev/null 2>&1 \
       || gh repo view "$ORG/$name" --json name >/dev/null 2>&1; then
        created=$((created+1))
    else
        failed=$((failed+1)); echo "CREATE-FAIL $name"; continue
    fi
    if git -C "$dir" push -q -u origin HEAD 2>/dev/null; then
        pushed=$((pushed+1))
    else
        echo "PUSH-FAIL $name"
    fi
done
echo "done: created=$created pushed=$pushed skipped=$skipped failed=$failed"
