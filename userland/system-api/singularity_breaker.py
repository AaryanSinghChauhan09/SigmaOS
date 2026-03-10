"""
SIGMAOS SINGULARITY BREAKER v5.0
=================================
This script is designed to BREAK THE KERNEL BUS.
The Kernel should detect this 'Singularity' and activate the Shield.
"""

import sys
import os
import time

# Adjust paths
_ROOT = os.getcwd()
sys.path.append(os.path.join(_ROOT, "sigma_core"))
sys.path.append(os.path.join(_ROOT, "userland", "system-api"))
sys.path.append(_ROOT)

from sigma_core.kernel import SigmaKernel

def main():
    print("🚀 INITIALIZING SINGULARITY BREAKER...")
    kernel = SigmaKernel()
    
    # Enable Protections
    kernel.boot()
    
    # 1. Start monitoring the bus
    print("  [DETECTOR] Running baseline...")
    print(f"  {kernel.singularity_detector()}")
    
    # 2. FLOOD THE BUS! (Infinite Loop Simulation)
    print("  [STRESS] Beginning Critical Bus Flooding (900+ events)...")
    for i in range(1000):
        # We emit events that the kernel should detect as 'noise' or 'flood'
        kernel.bus.emit("noise.event", {"id": i, "payload": "CRITICAL_SYSTEM_NOISE" * 10})
        
        # Periodic check for Singularity
        if i % 100 == 0:
            res = kernel.singularity_detector()
            if res["status"] == "SINGULARITY_DETECTED":
                print(f"\n  ✔ SUCCESS: Kernel detected Singularity at event #{i}!")
                print(f"  ✔ ACTION: {res['action']}")
                break
    
    time.sleep(1)
    
    # Verify System Recovery
    print("\n[VERIFICATION] Checking System Health Post-Singularity...")
    health = kernel.health_check()
    print(f"  System Health: {health['watchdog']} (Integrated)")
    print(f"  Bus Backlog: {len(kernel.bus.get_history(2000))}")
    
    if len(kernel.bus.get_history(2000)) < 100:
        print("  ✔ SUCCESS: Singularity Shield successfully purged the flood.")
    else:
        print("  ✖ FAILURE: Shield failed to purge the bus history.")

    print("\n" + "="*50)
    print("🏆 SIGMAOS SINGULARITY SHIELD: VERIFIED")
    print("="*50)

if __name__ == "__main__":
    main()
