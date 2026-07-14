#!/usr/bin/env python3
"""
fix_markdown_lint.py — Auto-fix common markdownlint issues across .md files.

Fixes:
- MD022: Headings should be surrounded by blank lines
- MD031: Fenced code blocks should be surrounded by blank lines
- MD032: Lists should be surrounded by blank lines
- MD040: Fenced code blocks should have a language specified
- MD060: Table pipe missing space (compact table style)
"""

import re
import sys
import os
from pathlib import Path

# Default language for unlabeled code blocks
DEFAULT_LANG = "text"


def fix_blanks_around_headings(lines):
    """MD022: Ensure blank line before and after headings."""
    result = []
    for i, line in enumerate(lines):
        if re.match(r'^#{1,6}\s', line):
            # Ensure blank line before heading
            if result and result[-1].rstrip() != '':
                result.append('')
            result.append(line)
            # Ensure blank line after heading (peek ahead)
            if i + 1 < len(lines) and lines[i + 1].rstrip() != '':
                result.append('')
        else:
            result.append(line)
    return result


def fix_blanks_around_fences(lines):
    """MD031: Fenced code blocks should be surrounded by blank lines."""
    result = []
    in_fence = False
    for i, line in enumerate(lines):
        stripped = line.rstrip()
        is_fence = re.match(r'^(`{3,}|~{3,})', stripped)
        if is_fence:
            if not in_fence:
                # Opening fence
                if result and result[-1].rstrip() != '':
                    result.append('')
                in_fence = True
            else:
                # Closing fence
                in_fence = False
                result.append(line)
                if i + 1 < len(lines) and lines[i + 1].rstrip() != '':
                    result.append('')
                continue
        result.append(line)
    return result


def fix_blanks_around_lists(lines):
    """MD032: Lists should be surrounded by blank lines."""
    result = []
    list_pattern = re.compile(r'^(\s*)([-*+]|\d+[.)]) ')
    for i, line in enumerate(lines):
        is_list_item = bool(list_pattern.match(line))
        prev_is_list = bool(list_pattern.match(lines[i - 1])) if i > 0 else False
        if is_list_item and not prev_is_list:
            # Start of a list block — ensure blank line before
            if result and result[-1].rstrip() != '':
                result.append('')
        if not is_list_item and prev_is_list:
            # End of a list block — ensure blank line after
            if result and result[-1].rstrip() != '':
                result.append('')
        result.append(line)
    return result


def fix_fenced_code_language(lines):
    """MD040: Add a language specifier to bare fenced code blocks."""
    result = []
    in_fence = False
    for line in lines:
        stripped = line.rstrip()
        # Match opening fence with NO language
        open_fence_bare = re.match(r'^(`{3,}|~{3,})$', stripped)
        # Match opening fence WITH language
        open_fence_lang = re.match(r'^(`{3,}|~{3,})\S', stripped)
        close_fence = re.match(r'^(`{3,}|~{3,})$', stripped)

        if not in_fence:
            if open_fence_bare and not open_fence_lang:
                # Bare opening fence — add default language
                fence_char = open_fence_bare.group(1)
                result.append(line.replace(fence_char, fence_char + DEFAULT_LANG, 1))
                in_fence = True
                continue
            elif open_fence_lang:
                in_fence = True
        else:
            if close_fence:
                in_fence = False

        result.append(line)
    return result


def fix_table_spacing(lines):
    """MD060: Add spaces around table cell content (pipe style)."""
    result = []
    table_row = re.compile(r'^\|')
    for line in lines:
        if table_row.match(line.strip()):
            # Add spaces after | and before | where missing
            # e.g. |foo|bar| -> | foo | bar |
            # Split carefully to preserve alignment
            fixed = re.sub(r'\|([^ |])', r'| \1', line)
            fixed = re.sub(r'([^ |])\|', r'\1 |', fixed)
            result.append(fixed)
        else:
            result.append(line)
    return result


def fix_file(path: Path, verbose: bool = True) -> bool:
    """Apply all fixes to a file. Returns True if the file was modified."""
    try:
        content = path.read_text(encoding='utf-8', errors='replace')
    except Exception as e:
        print(f"  [SKIP] {path.name}: {e}")
        return False

    original = content
    lines = content.splitlines(keepends=False)

    # Apply fixes in order
    lines = fix_fenced_code_language(lines)
    lines = fix_blanks_around_headings(lines)
    lines = fix_blanks_around_fences(lines)
    lines = fix_blanks_around_lists(lines)
    lines = fix_table_spacing(lines)

    # Remove excessive blank lines (max 2 consecutive)
    cleaned = []
    blank_count = 0
    for line in lines:
        if line.strip() == '':
            blank_count += 1
            if blank_count <= 2:
                cleaned.append(line)
        else:
            blank_count = 0
            cleaned.append(line)

    new_content = '\n'.join(cleaned)
    if not new_content.endswith('\n'):
        new_content += '\n'

    if new_content != original:
        path.write_text(new_content, encoding='utf-8')
        if verbose:
            print(f"  [FIXED] {path}")
        return True
    else:
        if verbose:
            print(f"  [OK]    {path}")
        return False


def main():
    targets = sys.argv[1:] if len(sys.argv) > 1 else ['.']
    total_fixed = 0
    total_skipped = 0

    for target_str in targets:
        target = Path(target_str)
        if target.is_file() and target.suffix == '.md':
            files = [target]
        elif target.is_dir():
            files = list(target.rglob('*.md'))
        else:
            print(f"[WARN] Not found: {target_str}")
            continue

        print(f"\n=== Fixing {len(files)} file(s) in {target_str} ===")
        for f in sorted(files):
            # Skip .git directories
            if '.git' in f.parts:
                continue
            fixed = fix_file(f)
            if fixed:
                total_fixed += 1
            else:
                total_skipped += 1

    print(f"\n=== Done: {total_fixed} fixed, {total_skipped} unchanged ===")


if __name__ == '__main__':
    main()
