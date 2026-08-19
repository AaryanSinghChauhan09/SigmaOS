#!/usr/bin/env python3
"""
Advanced conflict resolution for SigmaOS: handles diff3 (|||||||) and standard merge markers.
Strategy: Extract ALL unique non-marker lines from conflict sections and synthesize them into one clean file,
safely ignoring stray/residual conflict markers.
"""

import os
import re
import sys

def extract_all_content(content):
    """
    Robustly parse standard git conflict blocks and stray markers.
    """
    lines = content.split('\n')
    out_lines = []
    i = 0
    n = len(lines)

    while i < n:
        line = lines[i]
        
        if line.startswith('<<<<<<<'):
            current_section = []
            base_section = []
            incoming_section = []

            # 1. Collect current
            i += 1
            while i < n and not lines[i].startswith('|||||||') and not lines[i].startswith('=======') and not lines[i].startswith('>>>>>>>'):
                current_section.append(lines[i])
                i += 1

            # 2. Collect base (optional)
            if i < n and lines[i].startswith('|||||||'):
                i += 1
                while i < n and not lines[i].startswith('=======') and not lines[i].startswith('>>>>>>>'):
                    base_section.append(lines[i])
                    i += 1

            # 3. Collect incoming (optional)
            if i < n and lines[i].startswith('======='):
                i += 1
                while i < n and not lines[i].startswith('>>>>>>>'):
                    incoming_section.append(lines[i])
                    i += 1

            # 4. Skip closing >>>>>>>
            if i < n and lines[i].startswith('>>>>>>>'):
                i += 1

            # Combine cleanly, preserving order and removing exact duplicates
            seen = set()
            combined = []
            for l in current_section + incoming_section:
                if l not in seen or not l.strip():
                    combined.append(l)
                    if l.strip():
                        seen.add(l)

            out_lines.extend(combined)

        elif line.startswith('|||||||') or line.startswith('=======') or line.startswith('>>>>>>>'):
            # Stray marker - skip
            i += 1
        else:
            out_lines.append(line)
            i += 1

    return '\n'.join(out_lines)


def deduplicate_declarations(content):
    """Remove duplicate pub mod, pub use, and use declarations."""
    lines = content.split('\n')
    result = []
    
    seen_pub_mod = set()
    seen_use_blocks = set()
    
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        if re.match(r'^pub mod \w+;$', stripped):
            mod_name = re.match(r'^pub mod (\w+);$', stripped).group(1)
            if mod_name not in seen_pub_mod:
                seen_pub_mod.add(mod_name)
                result.append(line)
            i += 1
            
        elif re.match(r'^mod \w+;$', stripped):
            mod_name = re.match(r'^mod (\w+);$', stripped).group(1)
            key = f"mod_{mod_name}"
            if key not in seen_pub_mod:
                seen_pub_mod.add(key)
                result.append(line)
            i += 1
        
        elif re.match(r'^(pub )?use .+;$', stripped):
            key = stripped.rstrip(';').strip()
            if key not in seen_use_blocks:
                seen_use_blocks.add(key)
                result.append(line)
            i += 1
        
        elif re.match(r'^(pub )?use .+\{$', stripped):
            block_lines = [line]
            i += 1
            while i < len(lines) and not lines[i].strip().endswith('};'):
                block_lines.append(lines[i])
                i += 1
            if i < len(lines):
                block_lines.append(lines[i])
                i += 1
            
            use_key = re.match(r'^(pub )?use (.+)\{', stripped)
            if use_key:
                base = use_key.group(2).strip().rstrip('{').strip()
                if base not in seen_use_blocks:
                    seen_use_blocks.add(base)
                    result.extend(block_lines)
            else:
                result.extend(block_lines)
        
        else:
            result.append(line)
            i += 1
    
    return '\n'.join(result)


def fix_file(filepath, verbose=True):
    """Fix a single file with conflict markers."""
    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            original = f.read()
    except Exception as e:
        print(f"ERROR reading {filepath}: {e}")
        return False
    
    if '|||||||' not in original and '<<<<<<<' not in original and '=======' not in original and '>>>>>>>' not in original:
        return False
    
    cleaned = extract_all_content(original)
    cleaned = deduplicate_declarations(cleaned)
    
    # Strip any potential residual markers
    cleaned = re.sub(r'^<<<<<<<.*\n?', '', cleaned, flags=re.MULTILINE)
    cleaned = re.sub(r'^\|\|\|\|\|\|\|.*\n?', '', cleaned, flags=re.MULTILINE)
    cleaned = re.sub(r'^=======\s*\n?', '', cleaned, flags=re.MULTILINE)
    cleaned = re.sub(r'^>>>>>>>.*\n?', '', cleaned, flags=re.MULTILINE)
    cleaned = re.sub(r'\n{4,}', '\n\n\n', cleaned)
    
    try:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(cleaned)
        if verbose:
            print(f"FIXED: {filepath}")
        return True
    except Exception as e:
        print(f"ERROR writing {filepath}: {e}")
        return False


def find_and_fix_all(root_dir, extensions=('.rs', '.md', '.toml', '.c', '.h', '.py', '.sh')):
    """Find all files with conflict markers and fix them."""
    fixed = 0
    skipped = 0
    errors = 0
    
    skip_dirs = {'.git', 'target', 'node_modules', '.cargo'}
    
    for dirpath, dirnames, filenames in os.walk(root_dir):
        dirnames[:] = [d for d in dirnames if d not in skip_dirs]
        
        for filename in filenames:
            if not any(filename.endswith(ext) for ext in extensions):
                continue
            
            filepath = os.path.join(dirpath, filename)
            try:
                with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
                    content = f.read()
                
                has_diff3 = '|||||||' in content
                has_conflict = '<<<<<<<' in content
                has_sep = '=======' in content
                has_end = '>>>>>>>' in content
                
                if has_diff3 or has_conflict or has_sep or has_end:
                    result = fix_file(filepath)
                    if result:
                        fixed += 1
                    else:
                        errors += 1
                else:
                    skipped += 1
                    
            except Exception as e:
                print(f"Exception processing {filepath}: {e}")
                errors += 1
    
    return fixed, skipped, errors


if __name__ == '__main__':
    root = sys.argv[1] if len(sys.argv) > 1 else '.'
    print(f"Scanning {root} for conflict markers...")
    fixed, skipped, errors = find_and_fix_all(root)
    print(f"\nDone: {fixed} files fixed, {skipped} files skipped, {errors} errors")
