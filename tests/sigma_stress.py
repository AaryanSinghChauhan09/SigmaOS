"""
SigmaOS Operational Stress Test (Apex v1.0)
===========================================
Simulates high-pressure missions to verify AutomationEngine reactivity.
"""
import time
import threading
import sys
import os

# Root injection
sys.path.insert(0, os.path.abspath(os.path.join(os.path.dirname(__file__), "..")))

from sigma_core.kernel import SigmaKernel

def cpu_spike():
    print("[STRESS] Initiating CPU Thermal Spike...")
    end = time.time() + 15
    while time.time() < end:
        _ = 2**1000

def ram_bloat(kernel):
    print("[STRESS] Bloating Process Table...")
    # Access via shard proxy 'process' instead of 'process_manager'
    for i in range(50):
        kernel.process.spawn(f"zombie_shard_{i}", cgroup="system.slice")
    print("[STRESS] 50 Shards injected.")

def main():
    print("--- SIGMAOS KERNEL STRESS TEST ---")
    # Kernel hydrates on init by default (auto_load=True)
    kernel = SigmaKernel()
    
    time.sleep(2)
    
    # Trigger RAM pressure
    ram_bloat(kernel)
    
    # Trigger CPU pressure in background
    t = threading.Thread(target=cpu_spike)
    t.start()
    
    print("[STRESS] Monitoring AutomationEngine for reactive boost...")
    for _ in range(20):
        state = kernel.hal.get_hardware_state()
        print(f"TELEMETRY: CPU={state['cpu_load']} | RAM={state['ram_load']} | BUS={state['bus_status']}")
        time.sleep(1)
    
    t.join()
    print("[STRESS] Test sequence complete.")

if __name__ == "__main__":
    main()
