import os
import shutil

# SigmaOS: Sovereign Legacy Purge
# Finalizes Phase 2 by removing redundant legacy directories.

ROOT = "C:/Users/Aaryan/.gemini/antigravity/scratch/SigmaOS"
PURGE_LIST = [
    "kernel/shards",
    "kernel/modules",
    "drivers",
    "fs",
    "arch",
    "tools",
    "sovereign_tools",
    "absorption",
    "kernel/core",
    "kernel/shards", # Double check
]

def purge():
    print("[PURGE]: Initiating Sovereign Legacy Purge...")
    for item in PURGE_LIST:
        path = os.path.join(ROOT, item)
        if os.path.exists(path):
            print(f"  [DEL] {item}")
            if os.path.isdir(path):
                shutil.rmtree(path)
            else:
                os.remove(path)
    print("[PURGE]: Legacy repositories neutralized.")

if __name__ == "__main__":
    purge()
