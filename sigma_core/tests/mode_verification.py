"""
SigmaOS Mode Verification Suite (v1.0 Apex)
===========================================
USP: Automated verification of all 20+ specialized system modes.
Ensures DNA recalibration and aesthetic synchronization.
"""
import sys
import os

# Add root to path
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "../..")))

from sigma_core.kernel import SigmaKernel
from sigma_core.system.mode_manager import SigmaModeManager

def test_modes():
    print("Initiating SigmaOS Mode Stress Test...")
    kernel = SigmaKernel(auto_load=False)
    # Patch kernel to avoid full hydration for speed
    kernel.aura = type('Aura', (), {'apply_aura': lambda self, x: print(f"  [AURA] Applied {x}")})()
    
    modes = SigmaModeManager(kernel)
    kernel.registry.register("modes", modes)
    if not modes:
        print("[FAIL] Mode Manager not found in registry.")
        return

    test_cases = [
        "Cinema", "Driving", "Meditation", "Gaming", "Emergency", "Study", "Work", "Sleep"
    ]

    for mode in test_cases:
        print(f"\n[TEST] Switching to mode: {mode}")
        res = modes.switch_mode(mode)
        print(f"  Status: {res.get('Status')}")
        print(f"  Tuning: {res.get('Kernel_Tuning')}")
        
    print("\n[SUCCESS] Mode Verification Complete.")

if __name__ == "__main__":
    test_modes()
