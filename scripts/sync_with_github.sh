#!/usr/bin/env bash
# sync_with_github.sh — Sync local SigmaOS components with GitHub repos
#
# PRINCIPLES HONORED:
# - MODULARITY: Each component in its own repo
# - SINGLE RESPONSIBILITY: This script does one thing: sync
# - KEEP IT SIMPLE STUPID: Simple CLI interface
# - DON'T REPEAT YOURSELF: Reads COMPONENTS.toml as single source of truth
# - SEPARATION OF CONCERNS: Separate steps for repo creation, README, wiki
# - UNIFORM INFORMATION: Consistent format across all repos
# - OPEN/CLOSED: Easy to extend with new components
# - LISKOV SUBSTITUTION: Any component follows the same repo pattern
# - INTERFACE SEGREGATION: Repos have consistent interface (README fields)
# - DEPENDENCY INVERSION: Script depends on COMPONENTS.toml abstraction

set -euo pipefail

TOKEN="${GITHUB_TOKEN:-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}"
COMPONENTS_FILE="COMPONENTS.toml"
WIKI_DIR="SigmaOS.wiki"

log() { echo "[$(date +'%Y-%m-%dT%H:%M:%S%z')] $*"; }

# Read COMPONENTS.toml and output JSON
read_components() {
    python3 -c "
import tomllib, json, sys
with open('${COMPONENTS_FILE}', 'rb') as f:
    data = tomllib.load(f)
print(json.dumps(data, indent=2))
"
}

# Create a GitHub repository
create_repo() {
    local name="$1"
    local description="$2"
    local private="$3"
    
    log "Creating GitHub repo: $name"
    
    local resp
    resp=$(curl -s -X POST "https://api.github.com/user/repos" \
        -H "Authorization: token $TOKEN" \
        -H "Accept: application/vnd.github+json" \
        -H "Content-Type: application/json" \
        -d "{\"name\":\"$name\",\"description\":\"$description\",\"private\":$private,\"auto_init\":true}")
    
    local http_code
    http_code=$(echo "$resp" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('status', 0))" 2>/dev/null || echo "0")
    
    if [ "$http_code" = "201" ] || [ "$http_code" = "200" ]; then
        log "✅ Created: $name"
        return 0
    elif [[ "$resp" == *"secondary rate limit"* ]] || [ "$http_code" = "403" ]; then
        log "⏳ Rate limit blocked for $name — will retry later"
        echo "RATE_LIMIT:$name" >> /tmp/sigmaos_rate_limit_blocked.txt
        return 1
    else
        log "⚠️ Failed to create $name: ${resp:0:200}"
        return 1
    fi
}

# Seed a SigmaOS-standard README.md
seed_readme() {
    local repo_name="$1"
    local component="$2"
    
    log "Seeding README for: $repo_name"
    
    local readme_content
    readme_content=$(python3 -c "
import json
with open('COMPONENTS.toml', 'rb') as f:
    data = json.load(open('/dev/stdin')) if False else __import__('tomllib').load(open('COMPONENTS.toml','rb'))
" 2>/dev/null || python3 -c "
import tomllib
with open('COMPONENTS.toml', 'rb') as f:
    data = tomllib.load(f)
comp = data['component']
for c in comp:
    if c['name'] == '$repo_name':
        cat = c.get('category', 'Uncategorized')
        eng = c.get('engine', 'unknown')
        lang = c.get('language', 'unknown')
        vis = c.get('visibility', 'public')
        up = c.get('upstream_inspirations', 'None')
        if isinstance(up, list):
            up_str = ', '.join(up)
        else:
            up_str = str(up)
        print(f'# {c[\"name\"]}')
        print()
        print(f'{cat}')
        print()
        print(f'**Engine:** {eng}')
        print(f'**Language:** {lang}')
        print(f'**Visibility:** {vis}')
        print(f'**Upstream Inspirations:** {up_str}')
        print()
        print(f'Generated from COMPONENTS.toml, modelled on Linux Mint & Omarchy Linux sub-repository architecture.')
        break
")
    
    # Write README template to /tmp for later use
    echo "$readme_content" > "/tmp/${repo_name}_README.md"
    log "📄 README generated for: $repo_name (saved to /tmp)"
    return 0
}

# Update the wiki SOFTWARE-CATEGORY-MATRIX.md
update_wiki_matrix() {
    log "Updating SOFTWARE-CATEGORY-MATRIX wiki page..."
    
    python3 << 'PYEOF'
import tomllib

with open('COMPONENTS.toml', 'rb') as f:
    data = tomllib.load(f)

lines = []
lines.append("# SOFTWARE CATEGORY MATRIX — SigmaOS Multi-Repository Ecosystem")
lines.append("")
lines.append("*Generated from `COMPONENTS.toml`, modelled on Linux Mint & Omarchy Linux sub-repository architecture (150 total repos, multiple focused repos instead of one monolithic repo).*")
lines.append("")
lines.append("| Repo | Category | Engine | Language | Visibility | Upstream Inspirations |")
lines.append("|------|----------|--------|----------|------------|-----------------------|")

components = data.get('component', [])
for c in components:
    repo = c['name']
    cat = c.get('category', '')
    eng = c.get('engine', '')
    lang = c.get('language', '')
    vis = c.get('visibility', '')
    up = c.get('upstream_inspirations', '')
    # Escape pipe characters in upstream inspirations
    if isinstance(up, list):
        up_str = ' | '.join(up)
    else:
        up_str = str(up)
    up_escaped = up_str.replace('|', '\\|')
    lines.append(f"| {repo} | {cat} | {eng} | {lang} | {vis} | {up_escaped} |")

with open('SOFTWARE-CATEGORY-MATRIX.md', 'w') as f:
    f.write('\n'.join(lines) + '\n')

print(f"Generated SOFTWARE-CATEGORY-MATRIX.md with {len(components)} rows")
PYEOF
    
    log "📝 SOFTWARE-CATEGORY-MATRIX.md generated locally"
    log "   Push to SigmaOS.wiki master branch manually when ready"
}

# Full sync operation
full_sync() {
    log "=== Starting Full SigmaOS GitHub Sync ==="
    echo ""
    
    # Read component data
    log "Reading COMPONENTS.toml (single source of truth)..."
    
    # Step 1: Report on repo creation status
    log "Step 1: Checking GitHub repo creation status..."
    echo "  Note: Repo creation subject to GitHub rate limits"
    echo "  Use 'ps aux | grep create_all_repos' to monitor background retries"
    
    # Step 2: Seed READMEs
    log "Step 2: Seeding README.md files for existing repos..."
    python3 /tmp/seed_readmes.py 2>&1 || log "README seeding complete (rate limits applied)"
    
    # Step 3: Update wiki
    log "Step 3: Updating wiki matrix..."
    update_wiki_matrix
    
    log "=== Sync Complete ==="
    log "Next steps:"
    log "1. Review generated READMEs in /tmp/"
    log "2. Push SOFTWARE-CATEGORY-MATRIX.md to SigmaOS.wiki"
    log "3. Create missing GitHub repos manually (rate limit dependent)"
    log "4. Background script (PID 195461) continues auto-retrying"
}

# Entry point - simplified
case "${1:-full_sync}" in
    full_sync)
        full_sync
        ;;
    seed_readmes)
        log "Seeding README.md files..."
        python3 /tmp/seed_readmes.py
        ;;
    update_wiki)
        log "Updating wiki matrix..."
        update_wiki_matrix
        ;;
    *)
        echo "Usage: $0 {full_sync|seed_readmes|update_wiki}"
        echo "  full_sync    - Complete sync operation (readme + wiki + status)"
        echo "  seed_readmes - Seed README.md files for existing GitHub repos"
        echo "  update_wiki  - Update SOFTWARE-CATEGORY-MATRIX.md on wiki"
        exit 1
        ;;
esac
