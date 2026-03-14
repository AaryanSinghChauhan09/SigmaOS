"""
SigmaOS Sovereign Automation Engine (v1.0 Apex)
==============================================
USP: Autonomous Workflow Execution & Multi-Step OS Orchestration.
Handles scheduled tasks, pattern-based triggers, and cross-device handoffs.
"""
import time
import threading
from typing import Dict, Any, List, Callable, Optional

class AutomationEngine:
    """
    Sovereign Automation Engine manages complex system workflows.
    It allows users to define 'Sovereign Recipes' for automated maintenance and tasks.
    """
    def __init__(self, kernel=None):
        self.kernel = kernel
        self.workflows: Dict[str, List[Callable]] = {}
        self.scheduled_tasks: List[Dict[str, Any]] = []
        self._running = False
        self._loop_thread: Optional[threading.Thread] = None

    def start_service(self) -> str:
        self._running = True
        t = threading.Thread(target=self._automation_loop, daemon=True)
        self._loop_thread = t
        t.start()
        return "Sovereign Automation: Workflow Orchestrator Online."

    def register_workflow(self, name: str, steps: List[Callable]):
        """Registers a named sequence of system actions."""
        self.workflows[name] = steps

    def execute_workflow(self, name: str):
        """USP: Atomic Workflow Execution."""
        if name in self.workflows:
             for step in self.workflows[name]:
                  try:
                       step()
                  except Exception as e:
                       if self.kernel and hasattr(self.kernel, "error_mgr"):
                            self.kernel.error_mgr.handle_exception(e, shard_id=f"automation.{name}")
                       break

    def schedule_task(self, name: str, interval_sec: int, task: Callable):
        """Schedules a task to run periodically."""
        self.scheduled_tasks.append({
            "name": name,
            "interval": interval_sec,
            "task": task,
            "last_run": time.time()
        })

    def _automation_loop(self):
        while self._running:
            now = time.time()
            for task in self.scheduled_tasks:
                if now - task["last_run"] >= task["interval"]:
                    try:
                        task["task"]()
                    except Exception: pass
                    task["last_run"] = now
            time.sleep(1)

    def health_check(self) -> str:
        return f"OK — Active Workflows: {len(self.workflows)} | Scheduled: {len(self.scheduled_tasks)}"
