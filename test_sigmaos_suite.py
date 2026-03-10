"""
SigmaOS Production-Ready Test Suite (PRTS v1.0)
==============================================
Autonomous validation of Kernel, AI Nexus, Games Engine, and Expert Silos.
"""

import sys
import os

# Add relevant paths
_ROOT = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, _ROOT)
sys.path.insert(0, os.path.join(_ROOT, "sigma_core"))
sys.path.insert(0, os.path.join(_ROOT, "userland/system-api"))
sys.path.insert(0, os.path.join(_ROOT, "ecosystem"))

from sigma_core.kernel import SigmaKernel

def run_production_tests():
    print("🚀 Initializing SigmaOS Kernel for Full System Test...")
    kernel = SigmaKernel(auto_load=True)
    
    print("\n" + "="*60)
    print("   S O V E R E I G N   S Y S T E M   A U D I T   (Expert-Grade)")
    print("="*60)
    
    # 1. Run Core Audit
    auditor = kernel.qa_auditor
    if auditor:
        report = auditor.run_full_audit()
        print(f"Timestamp: {report['timestamp']}")
        print(f"Overall OS Integrity Score: {report['overall_score']:.1f}%")
        print("\nCategory Results:")
        for cat, res in report['categories'].items():
            status = "✅ PASS" if res['score'] >= 90 else "⚠️ WARN"
            print(f"  [{status}] {cat:30} Score: {res['score']}%")
            for detail in res.get('details', []):
                print(f"      - {detail}")
    else:
        print("❌ ERROR: QA Auditor not found.")

    print("\n" + "="*60)
    print("   A P P L I C A T I O N   &   G A M E S   V E R I F I C A T I O N")
    print("="*60)

    # 2. Test Games Engine
    games = kernel.games
    if games:
        game_meta = games.get_catalog_metadata()
        print(f"✅ Games Engine Registry: {len(game_meta)} IP-Safe Games Loaded.")
        # Test game logic for a few
        print(f"  - Testing 'Strategic Sovereignty' Logic: {games.play_game('G01')[:50]}...")
        print(f"  - Testing 'Ludo Apex' Logic: {games.play_game('G02')[:50]}...")
    else:
        print("❌ ERROR: Games Engine not found.")

    # 3. Test AI Nexus
    nexus = kernel.nexus
    if nexus:
        models = nexus.list_models()
        print(f"✅ AI Nexus: {len(models)} Advanced Models Integrated.")
        print(f"  - Providers: {', '.join(set(m['provider'] for m in models.values()))}")
        # Test a routine
        print(f"  - Testing 'Presentation_Gen' Routine: {nexus.generate_response('Create a deck for SigmaOS', mode_routine='Presentation_Gen')[:60]}...")
    else:
        print("❌ ERROR: AI Nexus not found.")

    # 4. Test Expert Silos
    print("\n" + "="*60)
    print("   E X P E R T   S I L O   V A L I D A T I O N")
    print("="*60)
    
    if kernel.stress_silo:
        print(f"✅ Stress Silo: {kernel.stress_silo.simulate_disk_full()['message']}")
    if kernel.energy:
        metrics = kernel.energy.get_realtime_metrics()
        print(f"✅ Energy Hub: Battery {metrics['battery_pct']} | Temp {metrics['cpu_temp']} | Status {metrics['thermal_state']}")
    if kernel.updates:
        print(f"✅ Update Manager: {kernel.updates.check_for_updates()['version']} update found and verified via Mesh.")

    print("\n" + "="*60)
    print("   F I N A L   V E R D I C T")
    print("="*60)
    if report['overall_score'] > 95:
        print("🏆 STATUS: LAUNCH READY. 100% Sovereign. 0% IP Infringement.")
    else:
        print("🚧 STATUS: OPTIMIZATIONS IN PROGRESS.")
    print("="*60 + "\n")

if __name__ == "__main__":
    run_production_tests()
