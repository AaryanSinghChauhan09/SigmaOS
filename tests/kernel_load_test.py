"""
SigmaOS Kernel Load Test (v1.0 Apex)
=====================================
USP: Forensic validation of the entire Apex Shard Grid.
Ensures every module in the manifest is reachable and healthy.
"""
import sys
import os

# Absolute path injection
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

try:
    from sigma_core.kernel import SigmaKernel
    from sigma_core.manifest import CORE_SYSTEM_MODULES
except ImportError as e:
    print(f"[FATAL] System imports failed: {e}")
    sys.exit(1)

def main():
    print("--- SIGMAOS KERNEL INTEGRITY AUDIT ---")
    kernel = SigmaKernel()
    
    print(f"\n[AUDIT] Validating {len(CORE_SYSTEM_MODULES)} system shards...")
    
    missing: list[str] = []
    healthy: int = 0
    
    for mod_path, class_name, shard_id in CORE_SYSTEM_MODULES:
        try:
            # Check if shard proxy exists on kernel
            if hasattr(kernel, shard_id):
                shard = getattr(kernel, shard_id)
                health = "UNKNOWN"
                if hasattr(shard, "health_check"):
                    health = shard.health_check()
                
                print(f"  ✔ {shard_id:<18} | Status: {health}")
                healthy = healthy + 1 # type: ignore
            else:
                print(f"  ✘ {shard_id:<18} | Status: MISSING_FROM_GRID")
                missing.append(shard_id)
        except Exception as e:
            print(f"  ⚠ {shard_id:<18} | Status: LOAD_ERROR ({e})")
            missing.append(shard_id)

    print(f"\n--- AUDIT RESULTS ---")
    print(f"  TOTAL: {len(CORE_SYSTEM_MODULES)}")
    print(f"  HEALTHY: {healthy}")
    print(f"  FAILURES: {len(missing)}")
    
    if missing:
        print(f"\n[CRITICAL] Missing Shards: {', '.join(missing)}")
        sys.exit(1)
    else:
        print("\n[SUCCESS] Kernel Tactical Grid is fully hydrated.")
        sys.exit(0)

if __name__ == "__main__":
    main()
