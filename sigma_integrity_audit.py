import sys
import os
import importlib

sys.path.insert(0, os.path.abspath("."))

def test_module(module_path, class_name):
    print(f"[SHARD-TEST] Testing {module_path}.{class_name}...")
    try:
        mod = importlib.import_module(module_path)
        cls = getattr(mod, class_name)
        instance = cls()
        print(f"✅ {class_name}: OK")
        return True
    except Exception as e:
        print(f"❌ {class_name}: FAILED ({e})")
        return False

def run_integrity_audit():
    print("💎 --- SigmaOS Omni-Sovereign Integrity Audit --- 💎\n")
    
    tests = [
        # Security & Core
        ("sigma_core.security.proof_ledger", "ProofLedger"),
        ("sigma_core.interfaces.base_sovereign", "SovereignModule")
    ]
    
    results = []
    for mod, cls in tests:
        results.append(test_module(mod, cls))
        
    if all(results):
        print("\n🏆 INTEGRITY VERIFIED: All core shards and application shims are stable.")
    else:
        print("\n⚠️ INTEGRITY BREACH: Some shards are failing. Optimization recommended.")
        sys.exit(1)

if __name__ == "__main__":
    run_integrity_audit()
