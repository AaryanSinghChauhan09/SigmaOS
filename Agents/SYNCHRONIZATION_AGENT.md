# Synchronization Agent

## Role
The **Synchronization Agent** (`SigmaSyncAgent`) ensures seamless, continuous, bidirectional synchronization between the SigmaOS source repository (`https://github.com/AaryanSinghChauhan09/SigmaOS`) and the standalone GitHub Wiki repository (`https://github.com/AaryanSinghChauhan09/SigmaOS/wiki`).

## Mission & Purpose
- Keep the main codebase and documentation in exact lockstep.
- Ensure that every component implemented or updated in `src/` has its corresponding documentation migrated to the GitHub Wiki.
- Ensure that every `.md` file in `Agents/` has a corresponding single canonical Wiki page.
- Maintain consistent cross-links, navigation portals, and table of contents across both platforms.

## Synchronization Protocol
```
[ Feature / Bugfix in SigmaOS (src/) ]
                │
                ▼
[ Verified Standalone Unit Test ]
                │
                ▼
[ Agents/COMPONENT_AGENT.md Updated ]
                │
                ▼
[ GitHub Wiki COMPONENT-Component-Guide.md Updated ]
                │
                ▼
[ Atomic Commit & Push to origin/main & wiki origin/master ]
```

## Executable Synchronization Verification Script
```bash
#!/bin/bash
# Reusable Wiki-Codebase Sync Verifier
set -e
REPO_DIR="/home/aaryansinghchauhan/SigmaOS"
WIKI_DIR="/home/aaryansinghchauhan/SigmaOS.wiki"

echo "=== [Sync Agent] Checking Main Repo Status ==="
cd "$REPO_DIR"
git status -s

echo "=== [Sync Agent] Checking Wiki Repo Status ==="
cd "$WIKI_DIR"
git status -s

echo "=== [Sync Agent] Auditing Unlinked Agents ==="
for agent in "$REPO_DIR"/Agents/*_AGENT.md; do
    base=$(basename "$agent" _AGENT.md | tr '[:upper:]' '[:lower:]' | tr '_' '-')
    # Look for matching guide in wiki
    if ! ls "$WIKI_DIR"/*"$base"* 1>/dev/null 2>&1; then
        echo "Missing Wiki counterpart for: $agent"
    fi
done
```

## Repeatable AI Execution Checklist
- [ ] Verify local working tree is clean in both `SigmaOS` and `SigmaOS.wiki`.
- [ ] Transfer finalized `.md` documentation to the Wiki repository.
- [ ] Update wiki navigation portals (`Home.md`, `INDEX.md`, `Component-Status.md`).
- [ ] Push changes to both `SigmaOS` (`main`) and `SigmaOS.wiki` (`master`).
- [ ] Confirm both remotes report up-to-date status.
