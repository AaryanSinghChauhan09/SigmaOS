import os
import re

# Define include mapping (basename -> suite path)
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
    "sigma_udf.h": "suites/S10_Registry/shards/sigma_udf.h",
    "sigma_math.h": "suites/S01_Genesis/shards/sigma_math.h",
    "sigma_kernel.h": "suites/S01_Genesis/shards/sigma_kernel.h",
    "sigma_libc.h": "suites/S01_Genesis/shards/sigma_libc.h",
    "sigma_base.h": "suites/S01_Genesis/shards/sigma_base.h",
    "SovereignCommon.h": "suites/S01_Genesis/shards/SovereignCommon.h",
    "SovereignModule.h": "suites/S01_Genesis/shards/SovereignModule.h",
    "Suites.h": "suites/S01_Genesis/shards/Suites.h",
    "sigma_module_registry.h": "suites/S01_Genesis/shards/sigma_module_registry.h",
    "sigma_types.h": "suites/S01_Genesis/shards/sigma_types.h",
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
                        # Also captures existing paths to replace them with the correct namespaced ones
                        pattern = rf'#include\s+["<](?:[^">]*/)?{old_h}[">]'
                        replacement = f'#include "{new_h}"'
                        new_content = re.sub(pattern, replacement, new_content)
                    
                    if new_content != content:
                        with open(path, "w", encoding="utf-8") as f:
                            f.write(new_content)
                        print(f"Updated: {path}")
                except Exception as e:
                    print(f"Error: {path} - {e}")

# Bulk update
update_includes("include")
update_includes("kernel")
update_includes("tests")
print("Absolute Final Sync Complete.")
