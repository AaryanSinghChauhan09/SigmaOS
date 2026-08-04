#!/usr/bin/env python3
"""
Advanced conflict resolution for SigmaOS: handles diff3 (|||||||) merge markers.
Strategy: Extract ALL unique non-marker lines from all sections (current, base, incoming)
and synthesize them into one clean file, deduplicating identical declarations.
"""

import os
import re
import sys
from collections import OrderedDict


def extract_all_content(content):
    """
    Parse diff3-style conflict markers and extract all unique content.
    In diff3: 
      <<<<<<< current
      ... current version ...
      ||||||| base commit hash
      ... base version ...
      ======= 
      ... incoming version ...
      >>>>>>> incoming
    
    Strategy: collect ALL non-marker lines from ALL sections, deduplicate.
    """
    lines = content.split('\n')
    collected = []
    
    i = 0
    while i < len(lines):
        line = lines[i]
        
        # Skip conflict start markers
        if line.startswith('<<<<<<<'):
            i += 1
            # Collect current section lines
            while i < len(lines) and not lines[i].startswith('|||||||') and not lines[i].startswith('=======') and not lines[i].startswith('>>>>>>>'):
                collected.append(lines[i])
                i += 1
        elif line.startswith('|||||||'):
            i += 1
            # Collect base section lines (skip them as they're the common ancestor)
            # We skip these since we have both current and incoming
            while i < len(lines) and not lines[i].startswith('=======') and not lines[i].startswith('>>>>>>>'):
                i += 1
        elif line.startswith('======='):
            i += 1
            # Collect incoming section lines
            while i < len(lines) and not lines[i].startswith('>>>>>>>'):
                collected.append(lines[i])
                i += 1
        elif line.startswith('>>>>>>>'):
            i += 1  # skip end marker
        else:
            collected.append(line)
            i += 1
    
    return '\n'.join(collected)


def deduplicate_declarations(content):
    """Remove duplicate pub mod, pub use, and use declarations."""
    lines = content.split('\n')
    result = []
    
    # Track seen simple one-liner declarations
    seen_pub_mod = set()
    seen_use_blocks = set()
    
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        # Handle `pub mod X;` declarations
        if re.match(r'^pub mod \w+;$', stripped):
            mod_name = re.match(r'^pub mod (\w+);$', stripped).group(1)
            if mod_name not in seen_pub_mod:
                seen_pub_mod.add(mod_name)
                result.append(line)
            # else: skip duplicate
            i += 1
            
        # Handle `mod X;` declarations
        elif re.match(r'^mod \w+;$', stripped):
            mod_name = re.match(r'^mod (\w+);$', stripped).group(1)
            key = f"mod_{mod_name}"
            if key not in seen_pub_mod:
                seen_pub_mod.add(key)
                result.append(line)
            i += 1
        
        # Handle single-line `pub use X::Y;` or `use X::Y;`
        elif re.match(r'^(pub )?use .+;$', stripped):
            key = stripped.rstrip(';').strip()
            if key not in seen_use_blocks:
                seen_use_blocks.add(key)
                result.append(line)
            i += 1
        
        # Handle multi-line use blocks: `pub use X::{`
        elif re.match(r'^(pub )?use .+\{$', stripped):
            # Collect the whole block
            block_lines = [line]
            i += 1
            while i < len(lines) and not lines[i].strip().endswith('};'):
                block_lines.append(lines[i])
                i += 1
            if i < len(lines):
                block_lines.append(lines[i])
                i += 1
            
            # Create a key from the use statement base
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
    
    if '|||||||' not in original and '<<<<<<' not in original:
        return False  # No conflict markers
    
    # Step 1: Extract all content from conflict sections
    cleaned = extract_all_content(original)
    
    # Step 2: Deduplicate declarations
    cleaned = deduplicate_declarations(cleaned)
    
    # Step 3: Safety net - remove any remaining conflict markers
    cleaned = re.sub(r'^<<<<<<<.*\n?', '', cleaned, flags=re.MULTILINE)
    cleaned = re.sub(r'^\|\|\|\|\|\|\|.*\n?', '', cleaned, flags=re.MULTILINE)
    cleaned = re.sub(r'^=======\s*\n?', '', cleaned, flags=re.MULTILINE)
    cleaned = re.sub(r'^>>>>>>>.*\n?', '', cleaned, flags=re.MULTILINE)
    
    # Step 4: Clean up excessive blank lines (more than 2 consecutive)
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
                
                if has_diff3 or has_conflict:
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
    root = sys.argv[1] if len(sys.argv) > 1 else '/home/aaryansinghchauhan/SigmaOS/src'
    print(f"Scanning {root} for conflict markers...")
    fixed, skipped, errors = find_and_fix_all(root)
    print(f"\nDone: {fixed} files fixed, {skipped} files skipped, {errors} errors")
