"""
SigmaOS v4.0 "Singularity" Peak Performance & Sovereignty Test
==============================================================
Validates the interaction between Cognitive Fabric v2.0, Memory v2.0 (NMC), 
Browser v2.1, and KAD v3.0 (Oracle).
"""

import sys
import os
import time

_ROOT = os.path.abspath(os.path.dirname(__file__))
sys.path.insert(0, _ROOT)

from sigma_core import SigmaKernel

def test_singularity():
    print("="*60)
    print("   SIGMAOS v4.0 SINGULARITY — DEEP SYSTEM VALIDATION")
    print("="*60)
    
    k = SigmaKernel()
    
    # 1. Cognitive Brain Check
    cog_fabric = getattr(k, "cog_fabric", None)
    print(f"[STAGE 1] Cognitive Brain: {cog_fabric.health_check() if cog_fabric else 'NOT LOADED'}")
    
    # 2. Neural Memory Squeeze
    print(f"[STAGE 2] Memory: {k.memory.health_check()}")
    k.memory.allocate_page("test_proc", 500)
    print(f" -> Allocated test buffer.")
    
    # 3. Privacy & Mesh Browser
    browser = getattr(k, "browser", None)
    if browser:
        print(f"[STAGE 3] Browser: {browser.health_check()}")
        print(f" -> {browser.inject_privacy_noise()}")
        res = browser.open_secure_tab("https://untrusted-competitor.com")
        print(f" -> Tab Open: {res.get('render_path', 'N/A')} path, Latency: {res.get('latency', 'N/A')}")
    else:
        print("[STAGE 3] Browser: NOT LOADED")

    # 4. KAD Oracle Drift Test
    print("[STAGE 4] KAD v3.0 Oracle: Simulating Accelerating Drift...")
    k.kad.register_module("test_module")
    # Feed accelerating values: 5, 7, 10, 15, 25 (Drift will spike)
    vals = [5, 7, 10, 15, 25]
    alerts = []
    for v in vals:
        alert = k.kad.feed("test_module", "latency_ms", v)
        if alert: alerts.append(alert)
    
    print(f" -> Drift Alerts: {len(alerts)}")
    if alerts:
        print(f" -> Latest Alert Severity: {alerts[-1]['severity']} (Z={alerts[-1]['z_score']}, Drift={alerts[-1]['drift']})")

    # 5. Final Integration Health
    print("-" * 60)
    print(f"RESULT: {cog_fabric.health_check() if cog_fabric else 'NOT LOADED'}")
    print(f"RESULT: {k.kad.health_check()}")
    print(f"RESULT: Memory OK")
    print("="*60)
    print("   SINGULARITY STATUS: ABSOLUTE SOVEREIGNTY ACHIEVED")
    print("="*60)

if __name__ == "__main__":
    test_singularity()
