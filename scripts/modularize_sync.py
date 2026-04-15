import os
import re

# Define moves
moves = [
    ("include/SigmaC11.h", "include/suites/S01_Genesis/shards/SigmaC11.h"),
    ("include/SigmaOOP.h", "include/suites/S03_Orchestrator/shards/SigmaOOP.h"),
    ("include/SovereignPurity.h", "include/suites/S25_ZeroKernel/shards/SovereignPurity.h"),
    ("include/Sovereign_API_v1.h", "include/suites/S03_Orchestrator/shards/Sovereign_API_v1.h"),
    ("kernel/suites/S01_Genesis/shards/SovereignBootMaster.c", "kernel/suites/S02_Boot/shards/SovereignBootMaster.c"),
    ("kernel/suites/S01_Genesis/shards/SovereignIDE.c", "kernel/suites/S15_DevNexus/shards/SovereignIDE.c"),
    ("kernel/suites/S01_Genesis/shards/SovereignInterruptHandler.c", "kernel/suites/S04_HAL/shards/SovereignInterruptHandler.c"),
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
    "SigmaC11.h": "suites/S01_Genesis/shards/SigmaC11.h",
    "SigmaOOP.h": "suites/S03_Orchestrator/shards/SigmaOOP.h",
    "SovereignPurity.h": "suites/S25_ZeroKernel/shards/SovereignPurity.h",
    "Sovereign_API_v1.h": "suites/S03_Orchestrator/shards/Sovereign_API_v1.h",
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
                        # Match #include "old_h" or #include <old_h>
                        pattern = rf'#include\s+["<]{old_h}[">]'
                        replacement = f'#include "{new_h}"'
                        new_content = re.sub(pattern, replacement, new_content)
                    
                    if new_content != content:
                        with open(path, "w", encoding="utf-8") as f:
                            f.write(new_content)
                        print(f"Updated includes in {path}")
                except Exception as e:
                    print(f"Error processing {path}: {e}")

# Update includes in include and kernel directories
update_includes("include")
update_includes("kernel")

# Update sigma_module_registry.h
registry_h = "include/sigma_module_registry.h"
if os.path.exists(registry_h):
    with open(registry_h, "r") as f:
        content = f.read()
    content = content.replace("S01_Genesis/SovereignBootMaster.c", "S02_Boot/SovereignBootMaster.c")
    content = content.replace("S01_Genesis/SovereignIDE.c", "S15_DevNexus/SovereignIDE.c")
    content = content.replace("S01_Genesis/SovereignInterruptHandler.c", "S04_HAL/SovereignInterruptHandler.c")
    with open(registry_h, "w") as f:
        f.write(content)
    print(f"Updated comments in {registry_h}")
