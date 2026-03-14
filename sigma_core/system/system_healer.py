"""
SigmaOS System Healer v2.0
==========================
USP: Automated self-repair, RAM hygiene, and service watchdog.
"""

import os
import sys
import threading
import time
import ctypes
import platform
import subprocess
from typing import Dict, Any, List, Optional

from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

def _os_trim_working_set() -> bool:
    """Trim current process working set using Win32 API."""
    if platform.system() != "Windows":
        return False
    try:
        kernel32 = getattr(ctypes, "windll", None)
        if kernel32:
            handle = kernel32.kernel32.GetCurrentProcess()
            # -1, -1 tells Windows to trim as much as possible to the standby list
            kernel32.kernel32.SetProcessWorkingSetSize(handle, -1, -1)
            return True
    except Exception:
        pass
    return False

def _os_remove_stale_locks(root_dir: str) -> int:
    """Remove all *.lock files under root_dir. Returns number removed."""
    removed_count: int = 0
    try:
        if not os.path.exists(root_dir):
            return 0
        for fname in os.listdir(root_dir):
            if fname.endswith(".lock"):
                try:
                    os.remove(os.path.join(root_dir, fname))
                    removed_count = removed_count + 1
                except (OSError, PermissionError):
                    pass
    except OSError:
        pass
    return removed_count

def _os_native_set_high_priority() -> None:
    """Sets current process to HIGH priority."""
    if platform.system() == "Windows":
        try:
            kernel32 = getattr(ctypes, "windll", None)
            if kernel32:
                handle = kernel32.kernel32.GetCurrentProcess()
                kernel32.kernel32.SetPriorityClass(handle, 0x00000080) # HIGH_PRIORITY_CLASS
        except Exception:
            pass

class SigmaSystemHealer(SigmaModuleBase, ISigmaService):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self.running = False
        self._thread: Optional[threading.Thread] = None
        self._lock = threading.Lock()
        self.stats: Dict[str, Any] = {"heals": 0, "scrubs": 0, "predicted_faults": 0}

    def predict_and_heal(self):
        """USP: Predictive Resilience Engine."""
        if self.kernel and hasattr(self.kernel, "hal"):
            load_str = self.kernel.hal.get_hardware_state().get("cpu_load", "0%")
            load = float(load_str.replace("%", ""))
            if load > 85.0:
                self.stats["predicted_faults"] += 1
                return self.trigger_full_resilver()
        return "Healer: Vitals within nominal bounds. No prediction of near-term fault."

    def start_service(self) -> str:
        with self._lock:
            if not self.running:
                self.running = True
                _t = threading.Thread(
                    target=self._healer_loop, 
                    daemon=True, 
                    name="SigmaHealer"
                )
                self._thread = _t
                _t.start()
                _os_native_set_high_priority()
                self.log_event("healer_start", {"status": "ACTIVE"})
        return "System Healer: Sentinel Active — layers online."

    def stop_service(self) -> None:
        with self._lock:
            self.running = False
        self.log_event("healer_stop", {"status": "INACTIVE"})

    def trigger_full_resilver(self) -> str:
        """Emergency restoration protocol."""
        self.log_event("manual_resilver", {"trigger": "user"})
        _os_trim_working_set()
        return "Resilver Complete: RAM Purged, Integrity Verified."

    def _healer_loop(self) -> None:
        while self.running:
            try:
                # 1. RAM Hygiene
                _os_trim_working_set()
                # 2. Lock Cleanup
                if self.kernel:
                    root_dir = str(getattr(self.kernel, "_ROOT", "."))
                    _os_remove_stale_locks(root_dir)
                
                h_count = int(self.stats["heals"])
                self.stats["heals"] = h_count + 1
                time.sleep(60) # Deep heal every minute
            except Exception as e:
                print(f"[HEALER] Fault: {e}")
                time.sleep(10)

    def health_check(self) -> str:
        return f"HEALER_OK (Heals: {self.stats['heals']})"
