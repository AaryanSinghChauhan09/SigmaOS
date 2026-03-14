"""
SigmaOS Sovereign Process Orchestrator (v1.0)
==============================================
USP: Hybrid Task Orchestration matching Go-routine efficiency.
Manages high-stakes research workflows and autonomous agent swarms.
"""
import threading
import time
import queue
from typing import Dict, Any, List, Callable
import os
import sys

# Robust System Path Injection
_p = os.path.abspath(__file__)
while _p and not os.path.exists(os.path.join(os.path.dirname(_p), "sigma_core")):
    _p = os.path.dirname(_p)
    if _p == os.path.dirname(_p): break
root = str(os.path.dirname(_p))
if root and root not in sys.path: sys.path.insert(0, root)

from sigma_core.event_bus import EventBus

class ProcessOrchestrator:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.task_queue = queue.PriorityQueue()
        self._running = False
        self._workers: List[threading.Thread] = []
        self.active_tasks = {}

    def start_service(self):
        """USP: Initializes the 'Middle Layer' worker swarm."""
        self._running = True
        for i in range(4): # 4 Core Sovereign Workers
            t = threading.Thread(target=self._worker_loop, name=f"SigmaWorker-{i}", daemon=True)
            t.start()
            self._workers.append(t)
        return "Orchestrator: Swarm Hydrated. Ready for high-concurrency missions."

    def submit_mission(self, name: str, mission_fn: Callable, priority: int = 10):
        """USP: Declarative Mission Submission."""
        mission_id = f"MIS_{int(time.time())}_{name}"
        self.task_queue.put((priority, mission_id, mission_fn))
        self.active_tasks[mission_id] = "QUEUED"
        return mission_id

    def _worker_loop(self):
        while self._running:
            try:
                priority, mission_id, fn = self.task_queue.get(timeout=1)
                self.active_tasks[mission_id] = "EXECUTING"
                
                # Execute Mission
                result = fn()
                
                self.active_tasks[mission_id] = "COMPLETED"
                if self.kernel:
                    self.kernel.bus.emit("mission.complete", {"id": mission_id, "res": result})
                    
            except queue.Empty:
                continue
            except Exception as e:
                print(f"[ORCHESTRATOR] Mission Failure ({mission_id}): {e}")
                self.active_tasks[mission_id] = "FAILED"

    def get_swarm_status(self):
        return {
            "Workers": len(self._workers),
            "Active": len([t for t in self.active_tasks.values() if t == "EXECUTING"]),
            "Queued": self.task_queue.qsize()
        }
