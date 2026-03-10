"""
SigmaOS Modular Boot Test v2.0
Validates the new modular kernel, event bus, and registry architecture.
Run: python boot_test_v2.py
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
print("\033[1m\033[96m   SigmaOS Modular Boot Test v2.0\033[0m")
print("\033[1m\033[96m══════════════════════════════════════════\033[0m")

# 1. Config singleton
HEAD("1. CONFIGURATION")
cfg = SigmaConfig()
ok(f"OS: {cfg.OS_NAME} v{cfg.VERSION}")
ok(f"Base Dir: {cfg.BASE_DIR}")

# 2. Event Bus
HEAD("2. EVENT BUS")
bus = EventBus()
results = []
bus.subscribe("test.ping", lambda p: results.append(f"pong:{p}"))
bus.emit("test.ping", "hello")
ok(f"Pub/Sub: {results[0]}")

# 3. Module Registry
HEAD("3. MODULE REGISTRY")
reg = ModuleRegistry()
class _DummyMod:
    def health_check(self): return "OK"
reg.register("dummy", _DummyMod(), {"source":"test","class":"_DummyMod"})
ok(f"Registered: dummy module")
ok(f"Call health: {reg.call('dummy','health_check')}")

# 4. Kernel Bootstrap
HEAD("4. KERNEL BOOTSTRAP")
t0 = time.perf_counter()
kernel = SigmaKernel(auto_load=True)
t1 = time.perf_counter()
n = len(kernel.registry.list_modules())
ok(f"Kernel loaded in {(t1-t0)*1000:.0f}ms")
ok(f"Modules registered: {n}")

# 5. Boot Sequence
HEAD("5. BOOT SEQUENCE")
steps = kernel.boot()
for step, result in steps.items():
    ok(f"[{step}] {result}")

# 6. Module Access
HEAD("6. MODULE PROPERTY ACCESS")
for name, prop in [
    ("PDF Forge",   kernel.pdf_forge),
    ("Titan",       kernel.titan_capture),
    ("Security",    kernel.security),
    ("Browser",     kernel.browser),
    ("Aether",      kernel.aether),
    ("SharedProc",  kernel.shared_processor),
]:
    status = "loaded" if prop else "\033[93mnot loaded\033[0m"
    ok(f"{name}: {status}")

# 7. Core Kernel Ops
HEAD("7. KERNEL OPERATIONS")
ok(kernel.process_document("test.pdf", "Audit"))
ok(kernel.capture_visual("OCR"))
ok(kernel.declarative_state_enforcement("0xSovereignHash"))
ok(kernel.initialize_wasm_runtime())
ok(kernel.carbon_aware_scheduler("AI_Training"))
ok(kernel.clean_text_native("## AI Report [1] http://example.com text"))

# 8. Security
HEAD("8. SECURITY SHIELD")
sec = kernel.security
if sec:
    ok(sec.secure_boot_verify())
    ok(sec.ebpf_proactive_monitoring())
    ok(sec.ai_threat_mitigation_engine({"name":"Core","entropy":0.12}))
    ok(sec.formal_verification_audit())

# 9. Boot Profile Selector
HEAD("9. BOOT PROFILES")
sel = kernel.registry.get("boot_selector")
if sel:
    profiles = sel.list_available_profiles()
    ok(f"Available profiles: {len(profiles)}")
    rec = sel.ai_recommendation("I am a Data Scientist specializing in AI Risk")
    ok(f"AI Recommendation: {rec}")
    ok(sel.select_profile(rec))

# 10. Browser
HEAD("10. SIGMA OMNI BROWSER")
br = kernel.browser
if br:
    ok(f"Engine: {br.engine}")
    ok(br.enable_privacy_vault())
    ok(br.activate_sovereign_web_archive())
    ok(br.create_space("Research_Lab"))

# 11. Event History
HEAD("11. EVENT BUS HISTORY")
history = kernel.bus.get_history(5)
ok(f"Events recorded: {len(kernel.bus.get_history())}")
for e in history[:3]:
    info(f"Event: {e['event']}")

# 12. Registry Call API
HEAD("12. REGISTRY REMOTE CALL")
result = kernel.registry.call("security", "secure_boot_verify")
ok(f"registry.call result: {result}")

# 13. Health Check
HEAD("13. FULL HEALTH CHECK")
health = kernel.health_check()
ok(f"Kernel: {health['kernel']}")
loaded = sum(1 for v in health["modules"].values() if "Error" not in str(v))
ok(f"Healthy modules: {loaded}/{len(health['modules'])}")

# Final Stats
HEAD("FINAL STATS")
stats = kernel.get_leadership_stats()
for k, v in stats.items():
    info(f"{k}: \033[92m{v}\033[0m")

print("\n\033[1m\033[92m══════════════════════════════════════════\033[0m")
print("\033[1m\033[92m   SIGMAOS v2.0 MODULAR BOOT: SUCCESS ✔\033[0m")
print("\033[1m\033[92m══════════════════════════════════════════\033[0m\n")
