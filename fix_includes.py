import os
import re

def fix_includes(filepath):
    with open(filepath, 'r', encoding='utf-8', errors='ignore') as f:
        content = f.read()

    # Fix depth-aware includes
    # If file is in kernel/core/ (depth 2)
    # If file is in kernel/core/hal/ (depth 3)
    
    parts = os.path.normpath(filepath).split(os.sep)
    # SigmaOS/kernel/core/...
    # 0       1      2
    
    depth = 0
    if 'kernel' in parts:
        kernel_idx = parts.index('kernel')
        depth = len(parts) - kernel_idx - 1
        
    prefix = "../" * depth + "include/"

    # Replace absolute-ish paths with depth-aware relative paths
    content = content.replace('#include "hal/sigma_hal.h"', f'#include "{prefix}hal/sigma_hal.h"')
    content = content.replace('#include "libc/SovereignLibC.h"', f'#include "{prefix}SovereignLibC.h"')
    content = content.replace('#include "core/sigma_types.h"', f'#include "{prefix}sigma_types.h"')
    content = content.replace('#include "core/SigmaOOP.hpp"', f'#include "{prefix}SigmaOOP.hpp"')
    
    # Fix incorrect depth (../../../ when it should be ../../ etc)
    # Common error: ../../../include/ in a depth 2 file
    if depth == 2:
        content = content.replace('#include "../../../include/', '#include "../../include/')
    elif depth == 3:
        content = content.replace('#include "../../include/', '#include "../../../include/')
        
    # Standardize sigma_hal.h to hal/sigma_hal.h if the C++ version is needed
    # But wait, sigma_hal.h (C) is in root include. hal/sigma_hal.h (C++) is in hal/
    # Most core files want the C++ one if they use SovereignHAL class.
    
    with open(filepath, 'w', encoding='utf-8') as f:
        f.write(content)

def walk_and_fix(root):
    for dirpath, _, filenames in os.walk(root):
        if '.git' in dirpath: continue
        for f in filenames:
            if f.endswith(('.cpp', '.hpp', '.h')):
                fix_includes(os.path.join(dirpath, f))

if __name__ == "__main__":
    walk_and_fix(".")
