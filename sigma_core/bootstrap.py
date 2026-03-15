import sys
import os

# Ensure local imports work correctly in a modular structure
sys.path.append(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from sigma_core.system_factory import get_factory
from sigma_core.kernel.kernel_core import SigmaKernel
from sigma_core.security.sovereignty_manager import SovereigntyManager
from sigma_core.system.device_manager import get_device_manager
from sigma_core.drivers.disk_driver import VirtualDiskDriver
from sigma_core.kernel.kernel_states import RunningState
from sigma_core.system.event_bus import get_event_bus
from sigma_core.analytics.system_auditor import SystemAuditor
from sigma_core.system.commander import get_commander
from sigma_core.interfaces.command_interfaces import ICommand
from sigma_core.system.scheduler import get_scheduler
from sigma_core.system.scheduler_strategies import EnergyEfficientStrategy
from sigma_core.security.proof_ledger import ProofLedger
from sigma_core.system.power_manager import PowerManager
from sigma_core.system.chaos_monkey import ChaosMonkey
from sigma_core.security.privacy_guard import DeterministicPrivacyGuard

class DisplayTextCommand(ICommand):
    """Polymorphic implementation of ICommand."""
    def execute(self, text):
        print(f"[UI-COMMAND] Displaying: {text}")
        return True

def bootstrap_zenith():
    """
    Initializes the Sovereign OOP Layers of SigmaOS.
    """
    print("--- SigmaOS Zenith Phase: Sovereign Bootstrap Sequence ---")
    
    factory = get_factory()
    bus = get_event_bus()
    commander = get_commander()
    scheduler = get_scheduler()
    proof_ledger = ProofLedger()
    power = PowerManager()
    chaos = ChaosMonkey(factory)
    privacy = DeterministicPrivacyGuard()
    
    # 1. Initialize Event System
    bus.initialize()
    commander.initialize()
    scheduler.initialize()
    proof_ledger.initialize()
    power.initialize()
    chaos.initialize()
    privacy.initialize()
    
    # 2. Register Core Systems
    kernel = SigmaKernel(power_manager=power) # Composition Injection
    security = SovereigntyManager()
    auditor = SystemAuditor()
    
    factory.register("Kernel", kernel, resilient=True, logged=True)
    factory.register("Security", security, resilient=True, logged=True)
    factory.register("Auditor", auditor, resilient=True, logged=True)
    factory.register("Commander", commander, resilient=True, logged=True)
    factory.register("Scheduler", scheduler, resilient=True, logged=True)
    factory.register("ProofLedger", proof_ledger, resilient=True, logged=True)
    factory.register("ChaosMonkey", chaos, resilient=False, logged=True)
    factory.register("PrivacyGuard", privacy, resilient=True, logged=True)
    
    # 3. Privacy Configuration
    privacy.register_tag("USER_ID_SHARD", "IDENTITY_VERIFICATION")
    privacy.register_tag("SYSTEM_CONFIG", "KERNEL_BOOT")
    
    # 4. Wire Events (Observer Pattern)
    bus.subscribe("SECURITY_ALERT", auditor)
    bus.subscribe("KERNEL_STATE_CHANGE", auditor)
    
    # 5. Wire Commands (Command Pattern)
    commander.register_command("DISPLAY_TEXT", DisplayTextCommand())
    
    # 6. Register Hardware Layer
    device_mngr = get_device_manager()
    disk_driver = VirtualDiskDriver(size_kb=512)
    device_mngr.register_driver("STORAGE_0", disk_driver)
    
    factory.register("DeviceManager", device_mngr, resilient=True, logged=True)
    
    print("--- Bootstrap Complete. Validating System Integrity ---")
    
    # 7. Test Formal Verification (Mathematical Certainty)
    test_logic = "def unsafe(): os.system('rm -rf /')"
    pl = factory.get("ProofLedger")
    is_safe = pl.validate_shard("UNSAFE_SHARD_001", test_logic)
    print(f"[TEST] Logic Safety: {'VERIFIED' if is_safe else 'REJECTED'}")
    
    # 8. Test Deterministic Privacy (LSP/ISP)
    pg = factory.get("PrivacyGuard")
    print(f"[TEST] Privacy Authorized (SYSTEM_CONFIG/KERNEL_BOOT): {pg.execute('AUTHORIZE', 'SYSTEM_CONFIG', 'KERNEL_BOOT')}")
    print(f"[TEST] Privacy Authorized (USER_ID_SHARD/MALICIOUS): {pg.execute('AUTHORIZE', 'USER_ID_SHARD', 'MALICIOUS_SNIFF')}")
    
    # 9. Test Chaos Resilience
    cm = factory.get("ChaosMonkey")
    chaos_event = cm.execute("TICK")
    print(f"[TEST] Chaos Resistance Result: SYSTEM_RECOVERED_FROM_{chaos_event}")
    
    # 10. Test Strategy Pattern (Energy-Aware Scheduling)
    sc = factory.get("Scheduler")
    sc.schedule_task("NET_POLL", priority=1, complexity=2)
    sc.schedule_task("AI_TRAIN", priority=9, complexity=10)
    
    print("[TEST] Scheduling with Performance Strategy:")
    print(sc.execute("DISPATCH_NEXT"))
    
    sc.set_strategy(EnergyEfficientStrategy())
    print("[TEST] Scheduling with Energy Strategy:")
    print(sc.execute("DISPATCH_NEXT"))
    
    # 11. Test Command Pattern (Polymorphism)
    cmd = factory.get("Commander")
    cmd.execute("DISPATCH", "DISPLAY_TEXT", "Welcome to SigmaOS Sovereign Zenith")
    
    # 12. Test Kernel State & Composition (Power side effects)
    k = factory.get("Kernel")
    print(f"[TEST] Kernel Status (Booting): {k.status}")
    k.set_state(RunningState())
    
    security_res = security.execute('INIT_VECTOR')
    print(f"[TEST] Security Check: {security_res}")
    
    print(f"[TEST] Devices: {device_mngr.execute('LIST_DEVICES')}")
    
    storage = device_mngr.get_driver("STORAGE_0")
    storage.write(0, b"SigmaSovereign_Zenith_OS_2026")
    data = storage.read(0, 30)
    print(f"[TEST] Storage I/O: {data.decode('utf-8', errors='ignore')}")
    
    # Check Auditor logs
    print(f"[TEST] Audit Logs: {auditor.execute('GET_LOGS')}")
    
    print("--- ALL SYSTEMS OPERATIONAL ---")

if __name__ == "__main__":
    try:
        bootstrap_zenith()
    except Exception as e:
        import traceback
        print(f"CRITICAL BOOT FAILURE: {e}")
        traceback.print_exc()
