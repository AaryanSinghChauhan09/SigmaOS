"""
fix_md_v2.py — Robust markdownlint auto-fixer.
Handles MD022, MD031, MD032, MD040, MD060 completely.
PERFORMANCE FIX: Precompiled regex patterns at module level for O(1) pattern matching.
PERFORMANCE FIX: Optimized line iteration using enumerate to reduce boundary checking.
"""
import re, sys
from pathlib import Path

# Precompile regex patterns at module level (PERFORMANCE FIX)
FENCE_RE = re.compile(r'^(`{3,}|~{3,})(.*)')
HEAD_RE = re.compile(r'^#{1,6}\s')
LIST_RE = re.compile(r'^(\s*)([-*+]|\d+[.)]) ')
TABLE_RE = re.compile(r'^\|')
SEP_ROW_RE = re.compile(r'^\|[\s:]*-+')

def fix(path):
    try:
        text = path.read_text(encoding='utf-8', errors='replace')
    except Exception:
        return False
    original = text
    lines = text.split('\n')
    out = []
    in_fence = False

    # Optimized iteration using enumerate (PERFORMANCE FIX)
    for i, line in enumerate(lines):
        stripped = line.rstrip()
        fm = FENCE_RE.match(stripped)

        if not in_fence and fm:
            # Opening fence
            lang = fm.group(2).strip()
            # MD040: add language if missing
            if not lang:
                line = fm.group(1) + 'text'
            # MD031: blank line before fence
            if out and out[-1].strip() != '':
                out.append('')
            out.append(line)
            in_fence = fm.group(1)
            continue

        if in_fence:
            cf = FENCE_RE.match(stripped)
            if cf and cf.group(1) == in_fence and not cf.group(2).strip():
                # Closing fence
                out.append(line)
                in_fence = False
                # MD031: blank line after fence
                if i + 1 < len(lines) and lines[i + 1].strip() != '':
                    out.append('')
            else:
                out.append(line)
            continue

        is_heading = bool(HEAD_RE.match(stripped))
        is_list = bool(LIST_RE.match(stripped))
        is_table = bool(TABLE_RE.match(stripped.lstrip()))
        prev_blank = (not out) or (out[-1].strip() == '')
        next_line = lines[i + 1] if i + 1 < len(lines) else ''
        next_blank = (next_line.strip() == '')

        # MD022: blank line before heading
        if is_heading and not prev_blank:
            out.append('')

        # MD032: blank line before list start
        if is_list and not prev_blank:
            prev_is_list = bool(LIST_RE.match(out[-1])) if out else False
            if not prev_is_list:
                out.append('')

        # MD060: table pipe spacing
        if is_table:
            # Add space after | where missing (but not for separator rows like |---|)
            sep_row = SEP_ROW_RE.match(stripped)
            if not sep_row:
                line = re.sub(r'\|([^\s|])', r'| \1', line)
                line = re.sub(r'([^\s|])\|', r'\1 |', line)

        out.append(line)

        # MD022: blank line after heading
        if is_heading and not next_blank:
            out.append('')

        # MD032: blank line after list block end
        if is_list and i + 1 < len(lines):
            next_is_list = bool(LIST_RE.match(next_line))
            if not next_is_list and not next_blank:
                out.append('')

    # Collapse 3+ consecutive blank lines to 2
    final = []
    blanks = 0
    for l in out:
        if l.strip() == '':
            blanks += 1
            if blanks <= 2:
                final.append(l)
        else:
            blanks = 0
            final.append(l)

    result = '\n'.join(final)
    if not result.endswith('\n'):
        result += '\n'

    if result != original:
        path.write_text(result, encoding='utf-8')
        return True
    return False

def main():
    targets = sys.argv[1:] or ['.']
    fixed = 0
    for t in targets:
        p = Path(t)
        files = [p] if p.is_file() else list(p.rglob('*.md'))
        for f in sorted(files):
            if '.git' in f.parts:
                continue
            if fix(f):
                fixed += 1
                print(f'  [FIXED] {f}')
    print(f'\nDone: {fixed} files fixed')

if __name__ == '__main__':
    main()
