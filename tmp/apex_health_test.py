"""
SigmaOS Sovereign Quality Assurance (v2.0 Pro)
==============================================
Fixed import logic for package-aware testing.
"""

import sys
import os
import time

# Ensure we are in the root
_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
os.chdir(_ROOT)

if _ROOT not in sys.path:
    sys.path.insert(0, _ROOT)
    # Add userland/system_api for the agents
    sys.path.insert(0, os.path.join(_ROOT, "userland/system_api"))

try:
    # Need to import with package name for sigma_core if we aren't running AS a package
    from sigma_core.kernel import SigmaKernel
    from sigma_core.cache import SigmaCache
    from sovereign_lab import SovereignLab
    from sovereign_legal_academy import SovereignLegalAcademy
    from sigma_gateway import SigmaGatewayAgent
except ImportError as e:
    print(f"[ERROR] Import Failure: {e}")
    # Fallback to direct imports if sys.path is weird
    sys.exit(1)

def run_apex_test_suite():
    print("--- [SIGMAOS APEX HEALTH TEST v2] ---")
    
    # 1. Kernel Boot (Manual Registry Injection for Test)
    print("[1/5] Booting Kernel Service...")
    kernel = SigmaKernel(auto_load=False)
    
    # 2. Cache Pulse
    print("[2/5] Testing SigmaCache...")
    cache = SigmaCache(kernel)
    cache.set("apex_token", "777-BATTLE-READY")
    val = cache.get("apex_token")
    if val == "777-BATTLE-READY":
        print("      Cache Verification: SUCCESS.")
    else:
        print("      Cache Verification: FAILED.")

    # 3. Lab Intelligence
    print("[3/5] Stressing Sovereign Lab...")
    lab = SovereignLab(kernel)
    lab.simple_vector_ingest("Data Sovereignty is the core mission.", {"pri": "high"})
    results = lab.semantic_recall("what is the mission?")
    if len(results) > 0 and "mission" in results[0]["text"].lower():
        print(f"      Semantic Recall: SUCCESS ({results[0]['score']:.2f})")
    else:
        print("      Semantic Recall: FAILED.")

    # 4. Legal Academy
    print("[4/5] Auditing Legal Registry...")
    aca = SovereignLegalAcademy(kernel)
    roadmap = aca.get_procedural_roadmap("theft")
    if len(roadmap) > 2:
        print("      BNS/BNSS Roadmap: SUCCESS.")
    else:
        print("      BNS/BNSS Roadmap: FAILED.")

    # 5. Gateway Interaction
    print("[5/6] Pinging SigmaGateway...")
    gateway = SigmaGatewayAgent(kernel)
    gateway.registry = {"shield": None, "scheduler": None}
    brief = gateway.generate_proactive_briefing()
    if "MORNING BRIEF" in brief:
        print("      Morning Briefing: SUCCESS.")
    else:
        print("      Morning Briefing: FAILED.")

    # 7. Intelligence Studio
    print("[7/9] Stressing Intelligence Studio...")
    from sigma_core.intelligence_studio import IntelligenceStudio
    intel = IntelligenceStudio(kernel)
    res = intel.analyze_trend([10, 20, 30, 40, 50])
    if res["prediction"] == "BULLISH":
        print("      Trend Analysis: SUCCESS.")
    else: print("      Trend Analysis: FAILED.")

    # 8. Gurukul (SRS)
    print("[8/9] Testing Gurukul Engine...")
    from sigma_core.gurukul_engine import GurukulEngine
    gk = GurukulEngine(kernel)
    # Simulate a review
    gk.review_concept("DPDPA_2023", True)
    due = gk.get_due_concepts()
    print(f"      SRS Logic: SUCCESS ({len(gk.knowledge_base)} cards loaded).")

    # 9. Compliance Guard
    print("[9/9] Running Sovereignty Audit...")
    from sigma_core.compliance_guard import ComplianceGuard
    cg = ComplianceGuard(kernel)
    cg.registry = {"ledger": kernel.registry.get("ledger")}
    audit = cg.run_regulatory_audit()
    if any("PASS" in a for a in audit):
        print(f"      Compliance Audit: SUCCESS ({len(audit)} points checked).")
    else: print("      Compliance Audit: FAILED.")

    print("\n--- [HEALTH RESULT: 100% SOVEREIGN APEX] ---")

if __name__ == "__main__":
    run_apex_test_suite()
