"""
SigmaOS Polymorphic Process Shifter v1.0
=========================================
USP: Security through obscuration and rapid identity rotation.
Confuses malware/trackers by frequently changing process signatures.
"""
import random
import time
import threading
from typing import Dict, List

class PolymorphicShifter:
    def __init__(self, kernel):
        self.kernel = kernel
        self.active_processes = {} # Local PID -> { "name": str, "shards": list }
        self.is_running = False
        self._lock = threading.Lock()
        
        self.common_fake_names = [
            "sigma_background_worker",
            "system_telemetry_node",
            "io_buffer_manager",
            "security_pulse_daemon",
            "low_lat_scheduler"
        ]

    def register_process(self, process_id: str, real_name: str):
        """Registers a process for polymorphic protection."""
        with self._lock:
            self.active_processes[process_id] = {
                "real_name": real_name,
                "current_alias": real_name,
                "rotated_at": time.time(),
                "history": [real_name]
            }
        print(f"[SHIFTER] Protected: {real_name} (ID: {process_id})")

    def shift_all(self):
        """Rotates the identity of all protected processes."""
        with self._lock:
            for pid, info in self.active_processes.items():
                new_alias = random.choice(self.common_fake_names) + "_" + str(random.randint(100, 999))
                info["current_alias"] = new_alias
                info["rotated_at"] = time.time()
                info["history"].append(new_alias)
                
                # In a real OS, this would involve renaming the process in /proc or Task Manager
                # Here we notify the kernel of the identity shift
                msg = f"POLYMORPH: '{info['real_name']}' is now masking as '{new_alias}'"
                self.kernel._morphic_island(msg, "#8A2BE2") # High-Viz Purple

    def get_real_identity(self, alias: str) -> str:
        """Resolves a masked name back to the real sovereign process."""
        with self._lock:
            for pid, info in self.active_processes.items():
                if info["current_alias"] == alias:
                    return info["real_name"]
        return "UNKNOWN_PROCESS"

    def start_rotation_thread(self, interval_sec: int = 300):
        """Starts a background thread to automatically shift identities."""
        self.is_running = True
        def loop():
            while self.is_running:
                time.sleep(interval_sec)
                self.shift_all()
        
        t = threading.Thread(target=loop, daemon=True)
        t.start()
        print(f"[SHIFTER] Dynamic Rotation Active (Interval: {interval_sec}s)")

if __name__ == "__main__":
    # Test stub
    class MockKernel:
        def _morphic_island(self, m, c): print(f"UI Island: [{c}] {m}")
    
    shifter = PolymorphicShifter(MockKernel())
    shifter.register_process("p_001", "SigmaCore_Ledger")
    shifter.shift_all()
    print(f"Alias p_001 identity: {shifter.get_real_identity(shifter.active_processes['p_001']['current_alias'])}")
