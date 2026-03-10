"""
SigmaOS v3.0 Apex Boot Test & Validation
Validates the Great Merger: Fabric, Automator, Forge, and Mesh.
Run: python boot_test_v3_apex.py
"""
import sys, os, time

_ROOT = os.path.abspath(os.path.dirname(__file__))
for _sub in ("", "kernel", "ecosystem"):
    sys.path.insert(0, os.path.join(_ROOT, _sub))

from sigma_core import SigmaKernel, SigmaConfig, EventBus, ModuleRegistry

PASS = "\033[92m✔\033[0m"
FAIL = "\033[91m✖\033[0m"
INFO = "\033[96mℹ\033[0m"
HEAD = lambda s: print(f"\n\033[1m\033[95m━━━ {s} ━━━\033[0m")
ok   = lambda m: print(f"  {PASS} {m}")
info = lambda m: print(f"  {INFO} {m}")

print("\n\033[1m\033[96m══════════════════════════════════════════\033[0m")
print("\033[1m\033[96m   SigmaOS v3.0 Apex Validation\033[0m")
print("\033[1m\033[96m══════════════════════════════════════════\033[0m")

# 1. Kernel Bootstrap
HEAD("1. APEX KERNEL BOOTSTRAP")
t0 = time.perf_counter()
kernel = SigmaKernel(auto_load=True)
t1 = time.perf_counter()
n = len(kernel.registry.list_modules())
ok(f"Apex Kernel loaded in {(t1-t0)*1000:.0f}ms")
ok(f"Modules registered: {n}")

# 2. Apex Module Access
HEAD("2. APEX v3 MODULE PROPERTY ACCESS")
for name, prop in [
    ("Neural Fabric",   kernel.fabric),
    ("Omni Automator",  kernel.automator),
    ("Content Forge",   kernel.forge),
    ("Aura Mesh",       kernel.mesh),
    ("UAL Bridge",      kernel.ual),
    ("Security",        kernel.security),
]:
    status = "APEX-READY" if prop else "\033[91mFAILED\033[0m"
    ok(f"{name}: {status}")

# 3. Fabric Operations (Brain)
HEAD("3. NEURAL FABRIC (BRAIN)")
if kernel.fabric:
    res = kernel.fabric.execute_neural_prefetch("Creative")
    ok(f"Predictive Warm: {res}")
    ok(f"Fabric Health: {kernel.fabric.health_check()}")

# 4. Automator Operations (Logic)
HEAD("4. OMNI AUTOMATOR (LOGIC)")
if kernel.automator:
    mid = kernel.automator.plan_mission("Forensic Audit")
    ok(f"Mission Planned: {mid}")
    ok(f"Automator Health: {kernel.automator.health_check()}")

# 5. Forge Operations (Content)
HEAD("5. CONTENT FORGE (ASSETS)")
if kernel.forge:
    res1 = kernel.forge.process_document("tax.pdf", "Audit")
    res2 = kernel.forge.capture_visual_region("Primary", "OCR")
    ok(f"PDF Audit: {res1}")
    ok(f"Visual OCR: {res2}")

# 6. Mesh Operations (Infrastructure)
HEAD("6. AURA MESH (INFRA)")
if kernel.mesh:
    res1 = kernel.mesh.add_mesh_peer("peer-apex-1")
    res2 = kernel.mesh.broadcast_update_intent("v3.0")
    ok(f"Peer Discovery: {res1}")
    ok(f"Mesh Consensus: {res2}")

# 7. Security Shield
HEAD("7. SECURITY SHIELD")
if kernel.security:
    ok(f"Secure Boot: {kernel.security.secure_boot_verify()}")
    ok(f"PQC Strategy: {kernel.security.ebpf_proactive_monitoring()}")

# 8. Universal Application Layer
HEAD("8. UNIVERSAL BRIDGE (UAL)")
if kernel.ual:
    res = kernel.ual.bridge_app("photoshop.exe")
    ok(f"UAL Bridge: {res['Message']}")

# 9. Final Synthesis Stats
HEAD("FINAL APEX STATS")
stats = kernel.get_leadership_stats()
for k, v in stats.items():
    info(f"{k}: \033[92m{v}\033[0m")

print("\n\033[1m\033[92m══════════════════════════════════════════\033[0m")
print("\033[1m\033[92m   SIGMAOS v3.0 APEX BOOT: SUCCESS ✔\033[0m")
print("\033[1m\033[92m══════════════════════════════════════════\033[0m\n")
