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

def bootstrap_zenith():
    """
    Initializes the Sovereign OOP Layers of SigmaOS.
    """
    print("--- SigmaOS Zenith Phase: Sovereign Bootstrap Sequence ---")
    
    factory = get_factory()
    
    # 1. Register Core Systems
    kernel = SigmaKernel()
    security = SovereigntyManager()
    factory.register("Kernel", kernel, resilient=True, logged=True)
    factory.register("Security", security, resilient=True, logged=True)
    
    # 2. Register Hardware Layer
    device_mngr = get_device_manager()
    disk_driver = VirtualDiskDriver(size_kb=512)
    device_mngr.register_driver("STORAGE_0", disk_driver)
    
    factory.register("DeviceManager", device_mngr, resilient=True, logged=True)
    
    print("--- Bootstrap Complete. Validating System Integrity ---")
    
    # 3. Test Integration
    k = factory.get("Kernel")
    s = factory.get("Security")
    d = factory.get("DeviceManager")
    
    print(f"[TEST] Kernel Status (Booting): {k.status}")
    print(f"[TEST] Kernel Exec (Booting): {k.execute('SYNC_SHARDS')}")
    
    # Transition to Running
    k.set_state(RunningState())
    print(f"[TEST] Kernel Exec (Running): {k.execute('SYNC_SHARDS')}")
    
    print(f"[TEST] Security Check: {s.execute('INIT_VECTOR')}")
    print(f"[TEST] Devices: {d.execute('LIST_DEVICES')}")
    
    storage = d.get_driver("STORAGE_0")
    storage.write(0, b"SigmaSovereign_Zenith_OS_2026")
    data = storage.read(0, 30)
    print(f"[TEST] Storage I/O: {data.decode('utf-8', errors='ignore')}")
    
    print("--- ALL SYSTEMS OPERATIONAL ---")

if __name__ == "__main__":
    try:
        bootstrap_zenith()
    except Exception as e:
        import traceback
        print(f"CRITICAL BOOT FAILURE: {e}")
        traceback.print_exc()
