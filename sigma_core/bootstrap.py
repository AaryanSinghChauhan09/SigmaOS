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
    
    # 1. Initialize Event System
    bus.initialize()
    commander.initialize()
    
    # 2. Register Core Systems
    kernel = SigmaKernel()
    security = SovereigntyManager()
    auditor = SystemAuditor()
    
    factory.register("Kernel", kernel, resilient=True, logged=True)
    factory.register("Security", security, resilient=True, logged=True)
    factory.register("Auditor", auditor, resilient=True, logged=True)
    factory.register("Commander", commander, resilient=True, logged=True)
    
    # 3. Wire Events (Observer Pattern)
    bus.subscribe("SECURITY_ALERT", auditor)
    bus.subscribe("KERNEL_STATE_CHANGE", auditor)
    
    # 4. Wire Commands (Command Pattern)
    commander.register_command("DISPLAY_TEXT", DisplayTextCommand())
    
    # 5. Register Hardware Layer
    device_mngr = get_device_manager()
    disk_driver = VirtualDiskDriver(size_kb=512)
    device_mngr.register_driver("STORAGE_0", disk_driver)
    
    factory.register("DeviceManager", device_mngr, resilient=True, logged=True)
    
    print("--- Bootstrap Complete. Validating System Integrity ---")
    
    # 6. Test Integration
    k = factory.get("Kernel")
    s = factory.get("Security")
    d = factory.get("DeviceManager")
    cmd = factory.get("Commander")
    
    print(f"[TEST] Kernel Status (Booting): {k.status}")
    bus.publish("KERNEL_STATE_CHANGE", {"from": "INITIALIZING", "to": "BOOTING"})
    
    # Transition to Running
    k.set_state(RunningState())
    bus.publish("KERNEL_STATE_CHANGE", {"from": "BOOTING", "to": "RUNNING"})
    
    # Test Command Pattern (Polymorphism)
    cmd.execute("DISPATCH", "DISPLAY_TEXT", "Welcome to SigmaOS Sovereign Zenith")
    
    print(f"[TEST] Security Check: {s.execute('INIT_VECTOR')}")
    bus.publish("SECURITY_ALERT", {"severity": "CRITICAL", "source": "INIT_VECTOR_SCAN"})
    
    print(f"[TEST] Devices: {d.execute('LIST_DEVICES')}")
    
    storage = d.get_driver("STORAGE_0")
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
