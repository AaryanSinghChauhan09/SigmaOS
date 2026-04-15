import os
import re

# Define moves
moves = [
    ("include/SovereignRegistry.h", "include/suites/S10_Registry/shards/SovereignRegistry.h"),
    ("include/SovereignLatticeRegistry.h", "include/suites/S10_Registry/shards/SovereignLatticeRegistry.h"),
    ("include/SovereignGCD.h", "include/suites/S19_Parallelism/shards/SovereignGCD.h"),
    ("include/SovereignInterconnect.h", "include/suites/S20_Interconnect/shards/SovereignInterconnect.h"),
    ("include/SovereignUDF.h", "include/suites/S10_Registry/shards/SovereignUDF.h"),
    ("kernel/suites/S01_Genesis/shards/SovereignRegistry.c", "kernel/suites/S10_Registry/shards/SovereignRegistry.c"),
    ("kernel/suites/S01_Genesis/shards/SovereignIPCRegistry.c", "kernel/suites/S10_Registry/shards/SovereignIPCRegistry.c"),
    ("kernel/suites/S01_Genesis/shards/SovereignInitRegistry.c", "kernel/suites/S10_Registry/shards/SovereignInitRegistry.c"),
    ("kernel/suites/S01_Genesis/shards/SovereignSyscallRegistry.c", "kernel/suites/S10_Registry/shards/SovereignSyscallRegistry.c"),
]

# Create directories and execute moves
for src, dst in moves:
    if os.path.exists(src):
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        try:
            os.replace(src, dst)
            print(f"Moved {src} to {dst}")
        except Exception as e:
            print(f"Failed to move {src}: {e}")

# Define include mapping (basename -> full suite path)
include_mapping = {
    "SovereignRegistry.h": "suites/S10_Registry/shards/SovereignRegistry.h",
    "SovereignLatticeRegistry.h": "suites/S10_Registry/shards/SovereignLatticeRegistry.h",
    "SovereignGCD.h": "suites/S19_Parallelism/shards/SovereignGCD.h",
    "SovereignInterconnect.h": "suites/S20_Interconnect/shards/SovereignInterconnect.h",
    "SovereignUDF.h": "suites/S10_Registry/shards/SovereignUDF.h",
}

def update_includes(directory):
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith((".c", ".h")):
                path = os.path.join(root, file)
                with open(path, "r", encoding="utf-8", errors="ignore") as f:
                    content = f.read()
                
                new_content = content
                for old_h, new_h in include_mapping.items():
                    # Match #include "old_h" or #include <old_h>
                    pattern = rf'#include\s+["<]{old_h}[">]'
                    replacement = f'#include "{new_h}"'
                    new_content = re.sub(pattern, replacement, new_content)
                
                if new_content != content:
                    with open(path, "w", encoding="utf-8") as f:
                        f.write(new_content)
                    print(f"Updated includes in {path}")

# Update includes in include and kernel directories
update_includes("include")
update_includes("kernel")

# Special update for sigma_module_registry.h comments
registry_h = "include/sigma_module_registry.h"
if os.path.exists(registry_h):
    with open(registry_h, "r") as f:
        content = f.read()
    content = content.replace("S01_Genesis/SovereignRegistry.c", "S10_Registry/SovereignRegistry.c")
    content = content.replace("S01_Genesis/SovereignIPCRegistry.c", "S10_Registry/SovereignIPCRegistry.c")
    content = content.replace("S01_Genesis/SovereignInitRegistry.c", "S10_Registry/SovereignInitRegistry.c")
    content = content.replace("S01_Genesis/SovereignSyscallRegistry.c", "S10_Registry/SovereignSyscallRegistry.c")
    with open(registry_h, "w") as f:
        f.write(content)
    print(f"Updated comments in {registry_h}")
