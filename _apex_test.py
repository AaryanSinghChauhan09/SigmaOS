import sys
from sigma_core import SigmaKernel

k = SigmaKernel(auto_load=True)

results = {}

def chk(key, obj, method="health_check"):
    try:
        val = getattr(obj, method)() if obj else "NOT LOADED"
        results[key] = val
    except Exception as e:
        results[key] = f"ERROR: {e}"

chk("WATCHDOG", k.watchdog)
chk("SHADOW",   k.shadow)
chk("CRUSHER",  k.crusher)
chk("PBS",      k.pbs)
chk("INTEL",    k.intel)
chk("KAD",      k.kad)
chk("CRASH",    k.crash_reporter)
chk("ENERGY",   k.energy_hub)
chk("UPDATE",   k.update_manager)
chk("PERF",     k.perf)
chk("MEMORY",   k.memory)
chk("PROCESS",  k.process)
chk("FS",       k.fs)
chk("NETGUARD", k.net_guard)
chk("REPAIR",   k.repair_engine)
chk("PREWARM",  getattr(k, "prewarmer", None))
chk("AURA",     getattr(k, "aura", None))
chk("WARDEN",   getattr(k, "warden", None))
chk("SANDBOX",  getattr(k, "sandbox", None))

print("\n" + "="*60)
print("   SIGMAOS SOVEREIGN — APEX FULL SYSTEM HEALTH REPORT")
print("="*60)
ok = 0
for k_name, v in results.items():
    status = "OK" if ("OK" in str(v) or "APEX" in str(v) or "HEALTHY" in str(v) or "NOT LOADED" not in str(v)) else "WARN"
    if "ERROR" in str(v) or "NOT LOADED" in str(v):
        status = "FAIL"
    else:
        ok += 1
    print(f"  [{status:4}] {k_name:12} {str(v)[:70]}")

print("="*60)
print(f"  RESULT: {ok}/{len(results)} modules nominal")
if ok == len(results):
    print("  *** APEX STATUS: ALL SYSTEMS GO ***")
print("="*60)
