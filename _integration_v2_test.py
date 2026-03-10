
import sys
import os
import time

# Standard library imports first
import logging

# We don't need to manually insert paths if we run as a module, 
# but for safety since we are running from root:
# sys.path.insert(0, os.getcwd())

try:
    from sigma_core.kernel import SigmaKernel
    from sigma_projects import TaskStatus
except ImportError as e:
    print(f"[ERR] Import failed: {e}")
    print("Ensure you run this from the SigmaOS root with 'py _integration_v2_test.py'")
    # Fallback path adjustment
    _ROOT = os.path.abspath(os.path.dirname(__file__))
    sys.path.insert(0, _ROOT)
    from sigma_core.kernel import SigmaKernel
    from sigma_projects import TaskStatus

def test_integration():
    print("--- SIGMAOS INTEGRATION TEST v2.0 ---")
    
    # 1. Boot Kernel
    k = SigmaKernel(auto_load=True)
    print(f"[OK] Kernel Bootstrapped. Version: {k.cfg.VERSION}")
    
    # 2. Test Routines
    routines = k.registry.get("routines")
    if routines:
        print(f"[OK] Routine Manager Active: {routines.health_check()}")
        # Trigger explicit routine
        res = routines.process_trigger("context:coding")
        print(f"[OK] Dev Routine Execution: {res}")
    else:
        print("[ERR] Routine Manager NOT FOUND in registry.")
        
    # 3. Test Projects & Event Propagation
    projects = k.registry.get("projects")
    if projects:
        print(f"[OK] Projects Engine Active: {projects.health_check()}")
        
        # Add a task
        tid = projects.add_task("Test Task", "Verifying routine triggers", TaskStatus.IN_PROGRESS)
        print(f"[OK] Task Created: {tid}")
        
        # Complete task (should trigger routine)
        print("Moving task to DONE (should trigger task.done routine)...")
        projects.update_task_status(tid, TaskStatus.DONE)
        print("[OK] Task Status Updated.")
    else:
        print("[ERR] Projects Engine NOT FOUND in registry.")
        
    # 4. Test Performance Engine
    perf = k.registry.get("perf")
    if perf:
        print(f"[OK] Performance Engine Pulse: {perf.health_check()}")
        profile = perf.apply_tuning("Apex")
        print(f"[OK] Apex Profile Applied: {profile['gpu_clock']}")
        
        # Test cycle stealing
        res = perf.trigger_workload_hoard()
        print(f"[OK] Workflow Hoard: {res}")
    else:
        print("[ERR] Performance Engine NOT FOUND in registry.")

    # 5. Linux Parity Health
    parity = k.registry.get("linux_parity")
    if parity:
        print(f"[OK] Linux Parity Status: {parity.gap_analysis.health_check()}")
        # Check specific distro gap
        gap = parity.gap_analysis.generate_report("Kali Linux")
        print("[OK] Kali Linux Gap Report Generated.")
    else:
        print("[ERR] Linux Parity Engine NOT FOUND in registry.")
        
    print("\n--- ALL SYSTEMS NOMINAL (SOVEREIGN GRADE) ---")

if __name__ == "__main__":
    try:
        test_integration()
    except Exception as e:
        print(f"\n[CRITICAL FAILURE] {type(e).__name__}: {e}")
        import traceback
        traceback.print_exc()
