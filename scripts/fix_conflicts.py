#!/usr/bin/env python3
"""
Fix unresolved git conflict markers (||||||| style from diff3) in SigmaOS source files.
Strategy: Remove all conflict markers and merge all sections together (deduplicating).
"""

import os
import re
import sys

def parse_conflict_sections(content):
    """Parse content with ||||||| style markers into clean sections."""
    lines = content.split('\n')
    result_lines = []
    seen_blocks = []
    
    i = 0
    while i < len(lines):
        line = lines[i]
        # Detect conflict marker (||||||| base version)
        if line.startswith('|||||||'):
            # We're inside a conflict block (the base version starts here)
            # Skip everything until we find the end (next non-marker content)
            # Actually, find the pattern: ||||||| ... lines ... ======= (optional) ... >>>>>>> (optional)
            # In diff3 style: current | base | incoming
            # We just collect all non-marker lines
            i += 1
            while i < len(lines) and not lines[i].startswith('|||||||') and not lines[i].startswith('>>>>>>>'):
                # These are base-version lines, skip them (we already have current version above)
                i += 1
            # Skip any >>>>>>> marker
            if i < len(lines) and lines[i].startswith('>>>>>>>'):
                i += 1
        elif line.startswith('>>>>>>>'):
            # End of incoming section without base marker seen
            i += 1
        elif line.startswith('<<<<<<<'):
            # Start of conflict - skip this marker line
            i += 1
        elif line.startswith('======='):
            # separator between current and incoming in 2-way conflicts
            # skip to end of incoming section
            i += 1
            while i < len(lines) and not lines[i].startswith('>>>>>>>'):
                i += 1
            if i < len(lines) and lines[i].startswith('>>>>>>>'):
                i += 1
        else:
            result_lines.append(line)
            i += 1
    
    return '\n'.join(result_lines)


def deduplicate_mod_declarations(content):
    """Remove duplicate pub mod and pub use declarations."""
    lines = content.split('\n')
    seen_mod = set()
    seen_use = set()
    result = []
    
    for line in lines:
        stripped = line.strip()
        
        # Handle pub mod declarations
        if stripped.startswith('pub mod ') and stripped.endswith(';'):
            mod_name = stripped[8:-1].strip()
            if mod_name not in seen_mod:
                seen_mod.add(mod_name)
                result.append(line)
            # else skip duplicate
        
        # Handle simple pub use - harder to deduplicate (multi-line), just keep
        else:
            result.append(line)
    
    return '\n'.join(result)


def fix_file(filepath):
    """Fix a single file with conflict markers."""
    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            original = f.read()
    except Exception as e:
        print(f"ERROR reading {filepath}: {e}")
        return False
    
    if '|||||||' not in original:
        return False  # No conflict markers
    
    # Parse and clean conflict markers
    cleaned = parse_conflict_sections(original)
    
    # Deduplicate module declarations
    cleaned = deduplicate_mod_declarations(cleaned)
    
    # Remove any remaining conflict markers that might have been missed
    cleaned = re.sub(r'^<<<<<<<.*$', '', cleaned, flags=re.MULTILINE)
    cleaned = re.sub(r'^=======\s*$', '', cleaned, flags=re.MULTILINE)
    cleaned = re.sub(r'^>>>>>>>.*$', '', cleaned, flags=re.MULTILINE)
    cleaned = re.sub(r'^\|\|\|\|\|\|\|.*$', '', cleaned, flags=re.MULTILINE)
    
    # Clean up excessive blank lines (more than 2 consecutive)
    cleaned = re.sub(r'\n{4,}', '\n\n\n', cleaned)
    
    try:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(cleaned)
        print(f"FIXED: {filepath}")
        return True
    except Exception as e:
        print(f"ERROR writing {filepath}: {e}")
        return False


def find_and_fix_all(root_dir):
    """Find all files with conflict markers and fix them."""
    fixed = 0
    errors = 0
    
    for dirpath, dirnames, filenames in os.walk(root_dir):
        # Skip hidden dirs and node_modules
        dirnames[:] = [d for d in dirnames if not d.startswith('.') and d != 'node_modules' and d != 'target']
        
        for filename in filenames:
            if filename.endswith(('.rs', '.md', '.toml', '.c', '.h', '.asm', '.s', '.py', '.sh')):
                filepath = os.path.join(dirpath, filename)
                try:
                    with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
                        content = f.read()
                    if '|||||||' in content:
                        if fix_file(filepath):
                            fixed += 1
                        else:
                            errors += 1
                except Exception as e:
                    pass
    
    return fixed, errors


if __name__ == '__main__':
    root = sys.argv[1] if len(sys.argv) > 1 else '/home/aaryansinghchauhan/SigmaOS/src'
    print(f"Scanning {root} for conflict markers...")
    fixed, errors = find_and_fix_all(root)
    print(f"\nDone: {fixed} files fixed, {errors} errors")
