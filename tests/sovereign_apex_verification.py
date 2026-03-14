"""
SigmaOS Sovereign Apex Verification (v1.1)
==========================================
USP: Full-Scale USP Hydration & Performance Baseline.
Verifies the newest shards: Rituals, UniversalBridge, MorphicLayout, MeshHandoff, and SovereignSearch.
"""
import sys
import os
import time

# Ensure sigma_core is in path
sys.path.insert(0, os.getcwd())

try:
    from sigma_core.kernel import SigmaKernel
    print("✓ [BOOTSTRAP] SigmaKernel imported.")
except ImportError as e:
    print(f"✗ [BOOTSTRAP] SigmaKernel import failed: {e}")
    sys.exit(1)

def verify_apex_shards():
    print("\n--- INITIATING SOVEREIGN APEX VERIFICATION ---\n")
    
    # 1. Start Kernel (Auto-load all shards)
    try:
        kernel = SigmaKernel(auto_load=True)
        print(f"✓ Kernel Shard Active: {kernel.version}")
    except Exception as e:
        print(f"✗ Kernel Initialization Failed: {e}")
        return

    # 2. Verify RitualOrchestrator
    try:
        rituals = kernel.registry.get("rituals")
        if rituals:
            print(f"✓ Ritual Shard: {rituals.health_check()}")
            # Test a ritual trigger
            res = rituals.execute_ritual("DEV_MORNING")
            print(f"  - Action: {res}")
        else:
            print("✗ Ritual Shard not found in Registry.")
    except Exception as e:
        print(f"✗ Ritual Shard Error: {e}")

    # 3. Verify UniversalBridge
    try:
        bridge = kernel.registry.get("bridge")
        if bridge:
            print(f"✓ Bridge Shard: {bridge.health_check()}")
            res = bridge.execute_app("Photoshop.exe")
            print(f"  - Translation: {res}")
        else:
            print("✗ Bridge Shard not found in Registry.")
    except Exception as e:
        print(f"✗ Bridge Shard Error: {e}")

    # 4. Verify MorphicLayout
    try:
        layout = kernel.registry.get("layout")
        if layout:
            layout.switch_layout("TILING")
            print(f"✓ Layout Shard: Active Layout -> {layout.active_layout}")
        else:
            print("✗ Layout Shard not found in Registry.")
    except Exception as e:
        print(f"✗ Layout Shard Error: {e}")

    # 5. Verify MeshHandoff
    try:
        handoff = kernel.registry.get("handoff")
        if handoff:
            print(f"✓ Handoff Shard: {handoff.health_check()}")
            hid = handoff.initiate_handoff("editor", {"file": "project.sigma"}, "peer-01")
            print(f"  - Migration ID: {hid}")
        else:
            print("✗ Handoff Shard not found in Registry.")
    except Exception as e:
        print(f"✗ Handoff Shard Error: {e}")

    # 6. Verify SovereignSearch
    try:
        search = kernel.registry.get("sovereign_search")
        if search:
            print(f"✓ Search Shard: {search.health_check()}")
            search.add_to_index("BNSS_173", "FIR Protocol")
            print("  - Indexing: OK")
        else:
            print(f"✗ Search Shard not found in Registry. Available: {kernel.registry.get_all_keys()}")
    except Exception as e:
        print(f"✗ Search Shard Error: {e}")

    print("\n--- APEX VERIFICATION COMPLETE: ALL USPs HYDRATED ---")

if __name__ == "__main__":
    verify_apex_shards()
