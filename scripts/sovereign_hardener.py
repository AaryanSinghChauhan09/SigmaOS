import os
import re

def get_rel_path(src, target_name):
    # Mapping of common headers to their absolute locations (relative to project root)
    registry = {
        "SigmaOOP.hpp": "include/SigmaOOP.hpp",
        "SovereignLibC.h": "include/SovereignLibC.h",
        "sigma_types.h": "include/sigma_types.h",
        "sigma_kernel_types.h": "include/sigma_kernel_types.h",
        "vfs.h": "include/vfs.h",
        "VfsShard.hpp": "kernel/core/VfsShard.hpp"
    }
    
    if target_name not in registry:
        return f'"{target_name}"'
    
    target_path = registry[target_name]
    src_dir = os.path.dirname(src)
    rel = os.path.relpath(target_path, src_dir).replace('\\', '/')
    return f'"{rel}"'

def harden_lattice():
    print("[HARDENER]: Initiating Deep Lattice Hardening (Relative Path Parity)...")
    
    targets = ["SigmaOOP.hpp", "SovereignLibC.h", "sigma_types.h", "sigma_kernel_types.h", "vfs.h"]
    
    count = 0
    for root, _, files in os.walk('.'):
        if 'obj' in root or '.git' in root: continue
        for file in files:
            if file.endswith(('.h', '.c', '.hpp', '.cpp')):
                path = os.path.join(root, file).replace('\\', '/')
                changed = False
                with open(path, 'r', encoding='utf-8', errors='ignore') as f:
                    content = f.read()
                
                new_content = content
                for target in targets:
                    pattern = rf'#include\s+[<"](?:\.\./)*{re.escape(target)}[">]'
                    rel_include = f'#include {get_rel_path(path, target)}'
                    new_content = re.sub(pattern, rel_include, new_content)
                
                # Fix common type errors if headers are missing
                if "sigma_printf" in new_content and "SovereignLibC.h" not in new_content:
                    # Insert include after guard or at top
                    if "#ifndef" in new_content:
                        new_content = re.sub(r'(#define.*\n)', r'\1\n#include ' + get_rel_path(path, "SovereignLibC.h") + '\n', new_content, count=1)
                    else:
                        new_content = "#include " + get_rel_path(path, "SovereignLibC.h") + "\n" + new_content

                if new_content != content:
                    with open(path, 'w', encoding='utf-8') as f:
                        f.write(new_content)
                    print(f"[REL-FIXED]: {path}")
                    count += 1
                    
    print(f"[LATTICE]: {count} files refactored for relative parity.")

if __name__ == '__main__':
    harden_lattice()
