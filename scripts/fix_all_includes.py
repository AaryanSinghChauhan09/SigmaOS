import os
import re

def fix_file(path, up_levels):
    with open(path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Replace #include "./include/..." with #include "{up_levels}include/..."
    # If up_levels is 0, it's "./"
    prefix = "../" * up_levels if up_levels > 0 else "./"
    new_content = re.sub(r'#include\s+"\./include/', f'#include "{prefix}include/', content)
    
    if content != new_content:
        with open(path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        print(f"Fixed: {path}")

# Fix userland (1 level up)
for file in os.listdir("userland"):
    if file.endswith(".cpp"):
        fix_file(os.path.join("userland", file), 1)

# Fix tools (1 level up)
for file in os.listdir("tools"):
    if file.endswith(".cpp"):
        fix_file(os.path.join("tools", file), 1)

# Fix kernel (2 levels up for core/...)
for root, dirs, files in os.walk("kernel"):
    for file in files:
        if file.endswith((".cpp", ".h")):
            path = os.path.join(root, file)
            rel = os.path.relpath(path, ".")
            levels = rel.count(os.sep)
            fix_file(path, levels)

# Fix include directory (self-referential)
for root, dirs, files in os.walk("include"):
    for file in files:
        if file.endswith((".h", ".hpp")):
            path = os.path.join(root, file)
            with open(path, 'r', encoding='utf-8') as f:
                content = f.read()
            # If in include/core/sigma_types.h, levels = 2 (include, core)
            # So root is ../..
            # #include "./include/sigma_kernel_types.h" -> #include "../sigma_kernel_types.h"
            
            # Special case for include/ tree:
            # We want to remove the "./include/" part and make it relative to the include root.
            # But the best way is to make it relative to the FILE.
            
            rel = os.path.relpath(path, "include")
            depth = rel.count(os.sep)
            # if depth is 0 (direct in include/), prefix is "./"
            # if depth is 1 (in include/core/), prefix is "../"
            prefix = "../" * depth if depth > 0 else "./"
            
            new_content = re.sub(r'#include\s+"\./include/', f'#include "{prefix}', content)
            
            if content != new_content:
                with open(path, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                print(f"Fixed Include: {path}")
