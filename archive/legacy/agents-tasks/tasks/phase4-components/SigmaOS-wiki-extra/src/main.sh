#!/usr/bin/env bash
# SigmaOS Wiki Extra — supplemental wiki pages CLI browser scaffold
# Upstream: Arch Wiki (https://wiki.archlinux.org/), MediaWiki
set -euo pipefail

WIKI_DIR="${SIGMA_WIKI_DIR:-.agents/tasks/phase5-finalize/wiki}"
WIKI_VERSION="0.5"

usage() {
    cat >&2 <<EOF
Usage: $(basename "$0") <subcommand> [options]

SigmaOS supplemental wiki browser

Subcommands:
  list                   List all wiki pages
  show    <page>         Display a wiki page
  search  <query>        Search page titles and content
  recent                 List recently updated pages
  toc     <page>         Show table of contents for a page
  check-links            Validate all internal wiki links
  export  <page> [<out>] Export a page to a file
  version                Print tool version

EOF
}

find_wiki_root() {
    # Try relative to script dir, then CWD
    local dirs=(
        "$WIKI_DIR"
        "$(dirname "$0")/../../../../phase5-finalize/wiki"
        "/home/aaryansinghchauhan/SigmaOS/.agents/tasks/phase5-finalize/wiki"
    )
    for d in "${dirs[@]}"; do
        [ -d "$d" ] && echo "$d" && return
    done
    echo ""
}

WIKI_ROOT=$(find_wiki_root)

cmd_list() {
    if [ -z "$WIKI_ROOT" ] || [ ! -d "$WIKI_ROOT" ]; then
        echo "[wiki] Wiki directory not found: $WIKI_DIR"
        echo "[wiki] Expected pages:"
        for p in Home Architecture Principles Roadmap Changelog "Getting-Started"; do
            echo "  $p.md"
        done
        return
    fi
    echo "Wiki pages:"
    for f in "$WIKI_ROOT"/*.md; do
        local name
        name=$(basename "${f%.md}")
        local words
        words=$(wc -w < "$f" 2>/dev/null || echo 0)
        printf "  %-30s %s words\n" "$name" "$words"
    done
}

cmd_show() {
    local page="${1:?'page name required'}"
    if [ -z "$WIKI_ROOT" ] || [ ! -d "$WIKI_ROOT" ]; then
        echo "[wiki] Wiki directory not found" >&2; exit 1
    fi
    local path="$WIKI_ROOT/${page}.md"
    # Try case-insensitive fallback
    if [ ! -f "$path" ]; then
        path=$(find "$WIKI_ROOT" -iname "${page}.md" 2>/dev/null | head -1)
    fi
    if [ -z "$path" ] || [ ! -f "$path" ]; then
        echo "[wiki] Page not found: $page" >&2; exit 1
    fi
    cat "$path"
}

cmd_search() {
    local query="${1:?'search query required'}"
    if [ -z "$WIKI_ROOT" ] || [ ! -d "$WIKI_ROOT" ]; then
        echo "[wiki] Wiki directory not found" >&2; exit 1
    fi
    echo "Search results for '$query':"
    local found=0
    for f in "$WIKI_ROOT"/*.md; do
        local name
        name=$(basename "${f%.md}")
        if grep -qi "$query" "$f" 2>/dev/null; then
            local ctx
            ctx=$(grep -im1 "$query" "$f" | head -c 80 | tr '\n' ' ')
            printf "  %-25s …%s…\n" "$name" "$ctx"
            found=$((found+1))
        fi
    done
    [ "$found" -eq 0 ] && echo "  No results found"
}

cmd_recent() {
    if [ -z "$WIKI_ROOT" ] || [ ! -d "$WIKI_ROOT" ]; then
        echo "[wiki] Wiki directory not found" >&2; exit 1
    fi
    echo "Recently modified wiki pages:"
    find "$WIKI_ROOT" -name "*.md" -printf "%T@ %f\n" 2>/dev/null | \
        sort -rn | head -10 | while read -r ts file; do
        local name="${file%.md}"
        printf "  %-30s\n" "$name"
    done
}

cmd_toc() {
    local page="${1:?'page name required'}"
    if [ -z "$WIKI_ROOT" ] || [ ! -d "$WIKI_ROOT" ]; then
        echo "[wiki] Wiki directory not found" >&2; exit 1
    fi
    local path="$WIKI_ROOT/${page}.md"
    [ ! -f "$path" ] && path=$(find "$WIKI_ROOT" -iname "${page}.md" 2>/dev/null | head -1)
    if [ -z "$path" ] || [ ! -f "$path" ]; then
        echo "[wiki] Page not found: $page" >&2; exit 1
    fi
    echo "Table of Contents: $page"
    grep -n "^#" "$path" | while IFS=: read -r lineno heading; do
        local depth
        depth=$(echo "$heading" | grep -o "^#*" | wc -c)
        local indent
        indent=$(printf '%*s' $((depth*2-2)) '')
        echo "  $indent${heading#"${heading%%[! ]*}"}"
    done
}

cmd_check_links() {
    if [ -z "$WIKI_ROOT" ] || [ ! -d "$WIKI_ROOT" ]; then
        echo "[wiki] Wiki directory not found" >&2; exit 1
    fi
    echo "Checking internal wiki links..."
    local broken=0
    for f in "$WIKI_ROOT"/*.md; do
        while IFS= read -r line; do
            local link_re='\[([^]]*)\]\(([^)]+\.md)([^)]*)\)'
            while [[ "$line" =~ $link_re ]]; do
                local target="${BASH_REMATCH[2]}"
                target="${target%#*}"
                if [ ! -f "$WIKI_ROOT/$target" ]; then
                    echo "  BROKEN: $(basename "$f") → $target"
                    broken=$((broken+1))
                fi
                line="${line#*"${BASH_REMATCH[0]}"}"
            done
        done < "$f"
    done
    if [ "$broken" -eq 0 ]; then
        echo "  All internal links OK"
    else
        echo "  $broken broken link(s) found"
    fi
}

cmd_export() {
    local page="${1:?'page name required'}"
    local outfile="${2:-${page}.md}"
    if [ -z "$WIKI_ROOT" ] || [ ! -d "$WIKI_ROOT" ]; then
        echo "[wiki] Wiki directory not found" >&2; exit 1
    fi
    local path="$WIKI_ROOT/${page}.md"
    [ ! -f "$path" ] && path=$(find "$WIKI_ROOT" -iname "${page}.md" 2>/dev/null | head -1)
    if [ -z "$path" ] || [ ! -f "$path" ]; then
        echo "[wiki] Page not found: $page" >&2; exit 1
    fi
    cp "$path" "$outfile"
    echo "[wiki] Exported: $page → $outfile"
}

case "${1:-}" in
    list)         cmd_list ;;
    show)         cmd_show "${2:-}" ;;
    search)       cmd_search "${2:-}" ;;
    recent)       cmd_recent ;;
    toc)          cmd_toc "${2:-}" ;;
    check-links)  cmd_check_links ;;
    export)       cmd_export "${2:-}" "${3:-}" ;;
    version)      echo "SigmaOS wiki-extra browser v$WIKI_VERSION" ;;
    -h|--help|"") usage ;;
    *) echo "Unknown subcommand: $1" >&2; usage; exit 1 ;;
esac
