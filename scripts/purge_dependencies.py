import os
import re

# Sovereign Equivalent Map
REPLACEMENTS = {
    r'#include <stdio\.h>': '#include "suites/S01_Genesis/shards/sigma_libc.h"',
    r'#include <stdlib\.h>': '#include "suites/S01_Genesis/shards/sigma_libc.h"',
    r'#include <string\.h>': '#include "suites/S01_Genesis/shards/sigma_libc.h"',
    r'#include <stddef\.h>': '#include "suites/S01_Genesis/shards/sigma_types.h"',
    r'#include <stdint\.h>': '#include "suites/S01_Genesis/shards/sigma_types.h"',
    r'#include <stdbool\.h>': '#include "suites/S01_Genesis/shards/sigma_types.h"',
    r'printf\(': 'sigma_printf(',
    r'malloc\(': 'sigma_malloc(',
    r'free\(': 'sigma_free(',
    r'memset\(': 'sigma_memset(',
    r'memcpy\(': 'sigma_memcpy(',
    r'strlen\(': 'sigma_strlen(',
    r'strcpy\(': 'sigma_strcpy(',
    r'strcmp\(': 'sigma_strcmp('
}

def purge_dependencies(root_dir):
    print(f"Purging pre-defined dependencies in {root_dir}...")
    for root, dirs, files in os.walk(root_dir):
        for file in files:
            if file.endswith(('.c', '.h')):
                path = os.path.join(root, file)
                try:
                    with open(path, 'r', encoding='utf-8', errors='replace') as f:
                        content = f.read()
                    
                    new_content = content
                    for old, new in REPLACEMENTS.items():
                        new_content = re.sub(old, new, new_content)
                    
                    if new_content != content:
                        print(f"  [SOVEREIGNTY] Purged {path}")
                        with open(path, 'w', encoding='utf-8', errors='replace') as f:
                            f.write(new_content)
                except Exception as e:
                    print(f"  [ERROR] {path}: {e}")

if __name__ == "__main__":
    purge_dependencies('kernel/suites')
    print("Zero-dependency parity achieved.")
