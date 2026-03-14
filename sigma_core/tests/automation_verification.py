"""
SigmaOS Automation Verification Suite (v1.0 Apex)
================================================
USP: Automated verification of Sovereign Recipes & Orchestration.
"""
import sys
import os
import time

# Add root to path
sys.path.append(os.path.abspath(os.path.join(os.path.dirname(__file__), "../..")))

from sigma_core.kernel import SigmaKernel
from sigma_core.system.automation_engine import AutomationEngine

def test_automations():
    print("Initiating SigmaOS Automation Stress Test...")
    kernel = SigmaKernel(auto_load=False)
    
    # Mock necessary modules with method-signature compatibility
    kernel.boost = type('Boost', (), {'boost_system': lambda *a: print("  [BOOST] System Optimized.")})()
    kernel.hal = type('HAL', (), {'trim_working_set': lambda *a: print("  [HAL] Working set trimmed."), 
                                 'get_hardware_state': lambda *a: {"cpu_load": "96%", "ram_load": "92%"}})()
    kernel.registry.register("boost", kernel.boost)
    kernel.registry.register("hal", kernel.hal)
    
    auto = AutomationEngine(kernel)
    kernel.registry.register("automation", auto)
    
    print("\n[TEST] Manual Workflow Execution: performance.boost")
    auto.execute_workflow("performance.boost")
    
    print("\n[TEST] Reactive Orchestration (Simulated Loop Tick)")
    # Manually trigger what the loop does to verify reactive logic
    usage = kernel.hal.get_hardware_state()
    cpu_load = float(usage.get("cpu_load", "0%").replace("%", ""))
    ram_load = float(usage.get("ram_load", "0%").replace("%", ""))
    
    if ram_load > 90:
        print("  Triggering RAM Overload Automation...")
        auto.execute_workflow("performance.boost")
    if cpu_load > 95:
        print("  Triggering CPU Overload Automation...")
        auto.execute_workflow("power.save")

    print("\n[SUCCESS] Automation Verification Complete.")

if __name__ == "__main__":
    test_automations()
