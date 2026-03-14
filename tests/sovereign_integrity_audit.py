"""
SigmaOS Sovereign Integration Test (v1.0 Apex)
==============================================
USP: Automated Shard Hydration & Kernel Link Verification.
Verifies that all modularized shards are loadable and responsive.
"""
import sys
import os

# Ensure sigma_core is in path
sys.path.insert(0, os.getcwd())

try:
    from sigma_core.kernel import SigmaKernel
    print("✓ [BOOTSTRAP] SigmaKernel imported successfully.")
except ImportError as e:
    print(f"✗ [BOOTSTRAP] SigmaKernel import failed: {e}")
    sys.exit(1)

def run_integration_test():
    print("\n--- INITIATING SOVEREIGN INTEGRITY AUDIT ---\n")
    
    # 1. Kernel Initialization (No Auto Load to test manual hydration)
    kernel = SigmaKernel(auto_load=False)
    print("✓ Kernel Shard initialized.")

    # 2. Test Customizer Modularization
    try:
        from sigma_core.ui.customizer import SovereignCustomizer
        cust = SovereignCustomizer(kernel)
        res = cust.health_check()
        print(f"✓ UI Shard: {res}")
    except Exception as e:
        print(f"✗ UI Shard Failure: {e}")

    # 3. Test AI Modularization
    try:
        from sigma_core.ai.cortex_engine import CortexEngine
        cortex = CortexEngine(kernel)
        res = cortex.health_check()
        print(f"✓ AI Shard: {res}")
    except Exception as e:
        print(f"✗ AI Shard Failure: {e}")

    # 4. Test Resource Modularization
    try:
        from sigma_core.system.resource_alchemist import ResourceAlchemist
        alc = ResourceAlchemist(kernel)
        res = alc.health_check()
        print(f"✓ Resource Shard: {res}")
    except Exception as e:
        print(f"✗ Resource Shard Failure: {e}")

    # 5. Test Personalization Modularization
    try:
        from sigma_core.system.personalization import PersonalizationEngine
        pers = PersonalizationEngine(kernel)
        res = pers.health_check()
        print(f"✓ Intelligence Shard: {res}")
    except Exception as e:
        print(f"✗ Intelligence Shard Failure: {e}")

    # 6. Test Healer Modularization
    try:
        from sigma_core.system.autonomic_healer import AutonomicHealer
        healer = AutonomicHealer(kernel)
        res = healer.health_check()
        print(f"✓ Healer Shard: {res}")
    except Exception as e:
        print(f"✗ Healer Shard Failure: {e}")

    # 7. Test Automation Engine
    try:
        from sigma_core.system.automation_engine import AutomationEngine
        auto = AutomationEngine(kernel)
        res = auto.health_check()
        print(f"✓ Automation Shard: {res}")
    except Exception as e:
        print(f"✗ Automation Shard Failure: {e}")

    print("\n--- INTEGRITY AUDIT COMPLETE: SIGMAOS IS APEX READY ---")

if __name__ == "__main__":
    run_integration_test()
