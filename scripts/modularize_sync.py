import os
import re

# Define include mapping (basename -> suite path)
include_mapping = {
    "sigma_types.h": "suites/S01_Genesis/shards/sigma_types.h",
    "sigma_libc.h": "suites/S01_Genesis/shards/sigma_libc.h",
    "sigma_base.h": "suites/S01_Genesis/shards/sigma_base.h",
    "SovereignCommon.h": "suites/S01_Genesis/shards/SovereignCommon.h",
    "sigma_math.h": "suites/S01_Genesis/shards/sigma_math.h",
    "sigma_kernel.h": "suites/S01_Genesis/shards/sigma_kernel.h",
}

def update_includes(directory):
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith((".c", ".h")):
                path = os.path.join(root, file)
                try:
                    with open(path, "r", encoding="utf-8", errors="ignore") as f:
                        content = f.read()
                    
                    new_content = content
                    for old_h, new_h in include_mapping.items():
                        pattern = rf'#include\s+["<](?:[^">]*/)?{old_h}[">]'
                        replacement = f'#include "{new_h}"'
                        new_content = re.sub(pattern, replacement, new_content)
                    
                    if new_content != content:
                        with open(path, "w", encoding="utf-8") as f:
                            f.write(new_content)
                        print(f"Updated: {path}")
                except Exception as e:
                    print(f"Error: {path} - {e}")

# Apply to tests
update_includes("tests")
print("Test modularization sync complete.")
