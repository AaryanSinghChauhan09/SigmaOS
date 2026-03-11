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
chk("POLYGLOT", getattr(k, "polyglot_runtime", None))

print("\n" + "="*60)
print("   SIGMAOS SOVEREIGN — APEX FULL SYSTEM HEALTH REPORT")
print("="*60)
ok: int = 0
for k_name, v in results.items():
    v_str = str(v)
    status = "OK" if ("OK" in v_str or "APEX" in v_str or "HEALTHY" in v_str or "NOT LOADED" not in v_str) else "WARN"
    if "ERROR" in v_str or "NOT LOADED" in v_str:
        status = "FAIL"
    else:
        ok += 1
    
    safe_v = "".join([v_str[i] for i in range(min(70, len(v_str)))])
    print(f"  [{status:4}] {k_name:12} {safe_v}")

print("="*60)
print(f"  RESULT: {ok}/{len(results)} modules nominal")
if ok == len(results):
    print("  *** APEX STATUS: ALL SYSTEMS GO ***")
print("="*60)
