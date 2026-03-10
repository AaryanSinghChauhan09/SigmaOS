"""Test kernel loading - captures all errors and prints a clear summary."""
import sys, traceback
sys.path.insert(0, ".")
sys.path.insert(0, "kernel")
sys.path.insert(0, "sigma_core")

print("=== SigmaOS Kernel Boot Test ===\n")
errs = []
import builtins
orig_print = builtins.print
captured = []

def capturing_print(*args, **kwargs):
    line = " ".join(str(a) for a in args)
    captured.append(line)
    orig_print(*args, **kwargs)

builtins.print = capturing_print
try:
    from sigma_core.kernel import SigmaKernel
    k = SigmaKernel(auto_load=True)
    print("\n[SUCCESS] Kernel booted.\n")
    r = k.registry.all() if hasattr(k.registry, 'all') else k.registry._modules
    print(f"Loaded Modules: {list(r.keys())}")
except Exception as e:
    print(f"\n[FATAL] Kernel boot failed: {e}")
    traceback.print_exc()
finally:
    builtins.print = orig_print

err_lines = [l for l in captured if "ERROR" in l or "Failed" in l or "Error" in l]
if err_lines:
    print("\n=== MODULE LOAD ERRORS ===")
    for l in err_lines:
        print(l)
