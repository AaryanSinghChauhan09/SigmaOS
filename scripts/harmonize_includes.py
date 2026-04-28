import os
import re

def harmonize():
    # Root-anchored include mapping & Type Standardization
    replacements = {
        # Includes
        r'#include\s+[<"](?:\.\./)*include/sigma_kernel_types\.h[">]': '#include "sigma_kernel_types.h"',
        r'#include\s+[<"](?:\.\./)*libc/sigma_libc\.h[">]': '#include "SovereignLibC.h"',
        r'#include\s+[<"](?:\.\./)*libc/sigma_types\.h[">]': '#include "sigma_types.h"',
        r'#include\s+[<"](?:\.\./)*SovereignLibC\.h[">]': '#include "SovereignLibC.h"',
        r'#include\s+[<"](?:\.\./)*SigmaOOP\.hpp[">]': '#include "SigmaOOP.hpp"',
        r'#include\s+[<"](?:\.\./)*SovereignOmniShard\.h[">]': '#include "SovereignLibC.h"',
        r'#include\s+[<"](?:\.\./)*include/vfs\.h[">]': '#include "vfs.h"',
        r'#include\s+[<"](?:\.\./)*include/legal_shards\.h[">]': '#include "legal_shards.h"',
        
        # Types (Standardizing to sigma_ prefix)
        r'\bu32\b': 'sigma_u32',
        r'\bu16\b': 'sigma_u16',
        r'\bu8\b':  'sigma_u8',
        r'\bu64\b': 'sigma_u64',
        r'\bi32\b': 'sigma_i32',
        r'\bi16\b': 'sigma_i16',
        r'\bi8\b':  'sigma_i8',
        r'\bi64\b': 'sigma_i64',
        r'\busize\b': 'sigma_usize',
        r'\bisize\b': 'sigma_isize',
        r'\bbool_t\b': 'sigma_bool',
        r'\bpaddr_t\b': 'sigma_paddr_t',
        r'\bvaddr_t\b': 'sigma_vaddr_t',
        r'\bk_status\b': 'sigma_status',
        r'\bTRUE\b':  'SIGMA_TRUE',
        r'\bFALSE\b': 'SIGMA_FALSE',
        r'\bNULL\b':  'SIGMA_NULL',
    }

    count = 0
    for root, dirs, files in os.walk("."):
        if ".git" in root or "obj" in root: continue
        for file in files:
            if file.endswith(('.c', '.h', '.cpp', '.hpp')):
                path = os.path.join(root, file)
                with open(path, 'r', errors='ignore') as f:
                    content = f.read()
                
                new_content = content
                for pattern, replacement in replacements.items():
                    new_content = re.sub(pattern, replacement, new_content)
                
                if new_content != content:
                    with open(path, 'w') as f:
                        f.write(new_content)
                    print(f"[HARMONIZED]: {path}")
                    count += 1
    print(f"Total files harmonized: {count}")

if __name__ == "__main__":
    harmonize()
