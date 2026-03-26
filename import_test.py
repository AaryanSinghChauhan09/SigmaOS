import sys
import os

# --- SIGMA-OS SMOKE TEST v4.4 ---

sys.path.insert(0, os.path.abspath("."))

def test() -> bool:
    print("[TEST] Verifying Omni-Sovereign Modularity...")
    try:
        from sigma_core.security.proof_ledger import ProofLedger  # type: ignore
        pl = ProofLedger()
        print("✅ ProofLedger: SUCCESS")
    except Exception as e:
        print(f"❌ ProofLedger: FAILED ({e})")
        return False

    try:
        from sigma_core.interfaces.base_sovereign import SovereignModule  # type: ignore
        print("✅ SovereignModule: SUCCESS")
    except Exception as e:
        print(f"❌ SovereignModule: FAILED ({e})")
        return False

    return True

if __name__ == "__main__":
    if test():
        print("\n🏆 SYSTEM STABLE: Sibling dependencies resolved.")
    else:
        sys.exit(1)
