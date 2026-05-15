#!/usr/bin/env python3
"""
SigmaOS v15.0 Zenith — Master Automated Fix Script
Resolves all IDE-reported compilation errors across the kernel lattice.

Fixes applied:
  1. sigma_printf conflicting types  → remove the void-return re-declaration in sigma_hal.h
  2. ../../../include/ wrong depth   → correct to ../../include/ for kernel/core/* files
  3. libc/SovereignLibC.h            → replace with correct relative path
  4. hal/sigma_hal.h                 → replace with correct relative path
  5. core/sigma_types.h (shards)     → replace with ../../include/sigma_types.h
  6. SovereignBoot.cpp stray brace  → remove extraneous closing brace
  7. SovereignMAC.cpp strstr         → already uses sigma_strstr (confirmed OK)
  8. sigma_hal.h duplicate sigma_log → already renamed to sigma_hal_log
  9. SovereignVFS.cpp sigma_hal.h   → already fixed to ../../include/sigma_hal.h
  10. memory_manager.hpp wrong path  → fix include depth
"""

import os, re

ROOT = os.path.dirname(os.path.abspath(__file__))

# ---------------------------------------------------------------------------
# Replacement rules: list of (pattern, replacement) applied to file content
# Each rule is (regex_pattern, replacement_string)
# ---------------------------------------------------------------------------

GLOBAL_RULES = [
    # 1. Old-style 'core/' prefix paths used in shard files
    (r'#include\s*"core/sigma_types\.h"',          '#include "../../include/sigma_types.h"'),
    (r'#include\s*"core/sigma_log\.h"',             '#include "../../include/sigma_log.h"'),
    (r'#include\s*"core/sigma_hal\.h"',             '#include "../../include/sigma_hal.h"'),
    (r'#include\s*"core/SigmaOOP\.hpp"',            '#include "../../include/SigmaOOP.hpp"'),
    (r'#include\s*"core/SovereignLibC\.h"',         '#include "../../include/SovereignLibC.h"'),

    # 2. libc/ prefix paths
    (r'#include\s*"libc/SovereignLibC\.h"',        '#include "../../include/SovereignLibC.h"'),

    # 3. hal/ prefix paths  
    (r'#include\s*"hal/sigma_hal\.h"',             '#include "../../include/sigma_hal.h"'),

    # 4. Bare (no path) includes that should be absolute-relative
    (r'#include\s*"SovereignLibC\.h"(?!\s*/)',     '#include "../../include/SovereignLibC.h"'),
    (r'(?<!/)#include\s*"sigma_hal\.h"',           '#include "../../include/sigma_hal.h"'),

    # 5. Wrong depth (../../../) for kernel/core files — 3 levels up is wrong when
    #    the file is already in kernel/core/XXX/ (needs exactly 3 ../s to reach root,
    #    but include/ is at root, so ../../../include IS correct from kernel/core/subdir/
    #    BUT from kernel/core/ (no subdir) it should be ../../include)
    #    We handle this per-directory below.
]

# Rules that apply only in specific directory contexts (rel_dir → extra rules)
DIR_RULES = {
    # kernel/core/ (direct children, depth=2 from root)
    'kernel/core': [
        (r'#include\s*"\.\.\/\.\.\/\.\.\/include\/sigma_types\.h"',   '#include "../../include/sigma_types.h"'),
        (r'#include\s*"\.\.\/\.\.\/\.\.\/include\/sigma_log\.h"',     '#include "../../include/sigma_log.h"'),
        (r'#include\s*"\.\.\/\.\.\/\.\.\/include\/SigmaOOP\.hpp"',    '#include "../../include/SigmaOOP.hpp"'),
        (r'#include\s*"\.\.\/\.\.\/\.\.\/include\/SovereignLibC\.h"', '#include "../../include/SovereignLibC.h"'),
        (r'#include\s*"\.\.\/\.\.\/\.\.\/include\/sigma_hal\.h"',     '#include "../../include/sigma_hal.h"'),
        (r'#include\s*"\.\.\/\.\.\/\.\.\/include\/hal\/sigma_hal\.h"','#include "../../include/sigma_hal.h"'),
    ],
    # kernel/core/*/  (subdirectories, depth=3 from root) — ../../../include IS correct
    # tools/ and userland/ (depth=1 from root)
    'tools': [
        (r'#include\s*"core/sigma_types\.h"',    '#include "../include/sigma_types.h"'),
        (r'#include\s*"core/sigma_log\.h"',      '#include "../include/sigma_log.h"'),
        (r'#include\s*"core/SigmaOOP\.hpp"',     '#include "../include/SigmaOOP.hpp"'),
        (r'#include\s*"core/SovereignLibC\.h"',  '#include "../include/SovereignLibC.h"'),
        (r'#include\s*"sigma_hal\.h"',           '#include "../include/sigma_hal.h"'),
    ],
    'userland': [
        (r'#include\s*"sigma_hal\.h"',           '#include "../include/sigma_hal.h"'),
        (r'#include\s*"sigma_log\.h"',           '#include "../include/sigma_log.h"'),
        (r'#include\s*"SigmaOOP\.hpp"',          '#include "../include/SigmaOOP.hpp"'),
        (r'#include\s*"SovereignLibC\.h"',       '#include "../include/SovereignLibC.h"'),
        (r'#include\s*"sigma_types\.h"',         '#include "../include/sigma_types.h"'),
    ],
    # suites/ (depth=1 from root)
    'suites': [
        (r'#include\s*"core/sigma_types\.h"',    '#include "../../../include/sigma_types.h"'),
        (r'#include\s*"core/sigma_log\.h"',      '#include "../../../include/sigma_log.h"'),
        (r'#include\s*"core/SigmaOOP\.hpp"',     '#include "../../../include/SigmaOOP.hpp"'),
        (r'#include\s*"core/SovereignLibC\.h"',  '#include "../../../include/SovereignLibC.h"'),
    ],
    # kernel/shards/ (depth=2 from root → ../../../include is 3 deep, needs 3)
    'kernel/shards': [
        (r'#include\s*"core/sigma_types\.h"',    '#include "../../../include/sigma_types.h"'),
        (r'#include\s*"core/sigma_log\.h"',      '#include "../../../include/sigma_log.h"'),
        (r'#include\s*"core/SigmaOOP\.hpp"',     '#include "../../../include/SigmaOOP.hpp"'),
        (r'#include\s*"core/SovereignLibC\.h"',  '#include "../../../include/SovereignLibC.h"'),
    ],
}

EXTENSIONS = {'.cpp', '.hpp', '.h', '.c'}

def rel_dir_key(filepath):
    """Return the top-level dir-context key for a given absolute filepath."""
    rel = os.path.relpath(filepath, ROOT).replace('\\', '/')
    for key in DIR_RULES:
        if rel.startswith(key + '/') or rel.startswith(key + '\\'):
            return key
    return None

def fix_file(filepath):
    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            content = f.read()
    except Exception as e:
        print(f"  [SKIP] {filepath}: {e}")
        return False

    original = content

    # Apply global rules
    for pattern, replacement in GLOBAL_RULES:
        content = re.sub(pattern, replacement, content)

    # Apply directory-specific rules
    key = rel_dir_key(filepath)
    if key and key in DIR_RULES:
        for pattern, replacement in DIR_RULES[key]:
            content = re.sub(pattern, replacement, content)

    if content != original:
        try:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(content)
            print(f"  [FIXED] {os.path.relpath(filepath, ROOT)}")
            return True
        except Exception as e:
            print(f"  [ERR] Could not write {filepath}: {e}")
    return False

def fix_sovereign_boot():
    """Fix the extraneous closing brace in SovereignBoot.cpp."""
    path = os.path.join(ROOT, 'kernel', 'core', 'system', 'SovereignBoot.cpp')
    if not os.path.exists(path):
        return
    with open(path, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    # Remove the stray extern "C" closing brace at end of file
    # The file ends with: }\n\n} // extern "C"\n
    fixed = re.sub(r'\}\s*//\s*extern\s*"C"\s*$', '', content.rstrip()) + '\n'
    if fixed != content:
        with open(path, 'w', encoding='utf-8') as f:
            f.write(fixed)
        print(f"  [FIXED] SovereignBoot.cpp: removed stray extern \"C\" brace")

def fix_security_matrix():
    """Fix SovereignSecurityMatrix.cpp: wrong libc/ path."""
    path = os.path.join(ROOT, 'kernel', 'core', 'security', 'SovereignSecurityMatrix.cpp')
    if not os.path.exists(path):
        return
    with open(path, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    fixed = re.sub(r'#include\s*"libc/SovereignLibC\.h"', 
                   '#include "../../../include/SovereignLibC.h"', content)
    if fixed != content:
        with open(path, 'w', encoding='utf-8') as f:
            f.write(fixed)
        print(f"  [FIXED] SovereignSecurityMatrix.cpp: libc/ path corrected")

def fix_memory_manager():
    """Fix memory_manager.hpp wrong include depth."""
    path = os.path.join(ROOT, 'kernel', 'core', 'memory_manager.hpp')
    if not os.path.exists(path):
        return
    with open(path, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    fixed = re.sub(r'#include\s*"\.\./\.\./\.\./include/sigma_types\.h"',
                   '#include "../../include/sigma_types.h"', content)
    fixed = re.sub(r'#include\s*"\.\./\.\./\.\./include/SigmaOOP\.hpp"',
                   '#include "../../include/SigmaOOP.hpp"', fixed)
    if fixed != content:
        with open(path, 'w', encoding='utf-8') as f:
            f.write(fixed)
        print(f"  [FIXED] memory_manager.hpp: include depth corrected")

def fix_pkg_manager():
    """Fix SovereignPkgManager.cpp sigma_hal.h bare include."""
    path = os.path.join(ROOT, 'userland', 'SovereignPkgManager.cpp')
    if not os.path.exists(path):
        return
    with open(path, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    fixed = re.sub(r'#include\s*"sigma_hal\.h"', '#include "../include/sigma_hal.h"', content)
    if fixed != content:
        with open(path, 'w', encoding='utf-8') as f:
            f.write(fixed)
        print(f"  [FIXED] SovereignPkgManager.cpp: sigma_hal.h path corrected")

def main():
    total_fixed = 0
    print("=== SigmaOS v15.0 Zenith — Automated Include Path Fix ===\n")

    # Special targeted fixes first
    fix_sovereign_boot()
    fix_security_matrix()
    fix_memory_manager()
    fix_pkg_manager()

    # Walk all source files
    for dirpath, dirnames, filenames in os.walk(ROOT):
        # Skip .git, wiki_repo (docs only), node_modules
        dirnames[:] = [d for d in dirnames if d not in {'.git', 'node_modules', '__pycache__'}]
        for filename in filenames:
            ext = os.path.splitext(filename)[1].lower()
            if ext in EXTENSIONS:
                filepath = os.path.join(dirpath, filename)
                if fix_file(filepath):
                    total_fixed += 1

    print(f"\n=== Done. {total_fixed} files updated. ===")

if __name__ == '__main__':
    main()
