import os
import shutil
import re

# SigmaOS: Sovereign Modularization Phase 2 (REFINED & PURGING)
# Consolidation of 400+ shards into 10 Master Suites.

SUITES = {
    "S01_Genesis": ["kernel/core", "kernel/SigmaOS_Zenith_Monolith.c", "kernel/modules/core/kmain.c", "kernel/boot.asm"],
    "S02_ZenithUI": ["index.html", "index.js", "index.css", "kernel/shards/ui_ux", "kernel/modules/multimedia", "kernel/shards/interface"],
    "S03_Distros": ["absorption", "kernel/shards/distros", "kernel/shards/linux_usp", "kernel/shards/core_linux"],
    "S04_HAL": ["arch", "drivers", "kernel/shards/hal", "kernel/shards/arch", "kernel/modules/drivers"],
    "S05_Memory": ["kernel/shards/memory", "kernel/shards/mm", "kernel/modules/core/SovereignMemorySuite.c", "kernel/modules/core/SovereignMemoryCore.c"],
    "S06_Storage": ["fs", "kernel/shards/fs", "kernel/shards/io", "kernel/shards/vfs", "kernel/modules/storage"],
    "S07_Network": ["kernel/shards/net", "kernel/shards/network", "kernel/shards/ipc", "kernel/modules/net", "kernel/modules/distributed", "kernel/shards/binder"],
    "S08_Security": ["kernel/shards/security", "kernel/modules/security", "kernel/modules/tracing"],
    "S09_Tooling": ["scripts", "sovereign_tools", "tools", "build.ps1", "Makefile"],
    "S10_Orchestration": ["kernel/shards/scheduler", "kernel/shards/syscalls", "kernel/shards/system", "kernel/modules/sched", "kernel/modules/system", "kernel/modules/core/SovereignModuleRegistry.c"]
}

ROOT = "C:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS"
TARGET_BASE = os.path.join(ROOT, "kernel/suites")

def ensure_header_recursive(dir_path):
    for root, _, files in os.walk(dir_path):
        for file in files:
            if file.endswith(".c"):
                path = os.path.join(root, file)
                try:
                    with open(path, 'r', encoding='utf-8', errors='ignore') as f:
                        content = f.read()
                    
                    if "sigma_base.h" not in content and "sigma_kernel.h" not in content:
                        print(f"  [HEADER] Injecting header into {path}")
                        # Calculate depth to include
                        depth = path.replace(ROOT, "").count(os.sep)
                        prefix = "../" * (depth - 1)
                        header = f'#include "{prefix}include/sigma_base.h"\n\n'
                        with open(path, 'w', encoding='utf-8') as f:
                            f.write(header + content)
                except Exception as e:
                    print(f"  [ERROR] Failed to process {path}: {e}")

def modularize():
    if not os.path.exists(TARGET_BASE):
        os.makedirs(TARGET_BASE)
        
    for suite, sources in SUITES.items():
        suite_path = os.path.join(TARGET_BASE, suite)
        if not os.path.exists(suite_path):
            os.makedirs(suite_path)
            
        print(f"Processing {suite}...")
        
        for src in sources:
            src_full = os.path.join(ROOT, src)
            if not os.path.exists(src_full):
                continue
                
            dest = os.path.join(suite_path, os.path.basename(src))
            
            print(f"  [MOVE] {src} -> {suite}")
            if os.path.isdir(src_full):
                # Copy content first
                for item in os.listdir(src_full):
                    s = os.path.join(src_full, item)
                    d = os.path.join(suite_path, item)
                    if os.path.isdir(s):
                        if os.path.exists(d): shutil.rmtree(d)
                        shutil.copytree(s, d)
                    else:
                        shutil.copy2(s, d)
                # Cleanup original
                # shutil.rmtree(src_full) # UNCOMMENT LATER FOR FULL PURGE
            else:
                shutil.copy2(src_full, dest)
                # os.remove(src_full) # UNCOMMENT LATER FOR FULL PURGE
        
        # Ensure headers in the suite
        ensure_header_recursive(suite_path)

    print("\n[MODULARIZER]: Phase 2 Consolidation & Refinement COMPLETE.")

if __name__ == "__main__":
    modularize()
