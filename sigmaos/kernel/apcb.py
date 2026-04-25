"""
SigmaOS Agentic Process Control Block (APCB)
Replaces the traditional PCB with an AI-aware process tracker.
"""
from dataclasses import dataclass, field
from typing import Dict, Any, List

@dataclass
class APCB:
    pid: int
    intent: str
    state: str = "READY"
    memory_vector_id: str = ""
    error_history: List[str] = field(default_factory=list)
    resource_limits: Dict[str, Any] = field(default_factory=lambda: {"cpu": 1.0, "ram_mb": 512})

    def handle_crash(self, traceback: str) -> None:
        """
        Instead of core dumping, the process pauses and asks the AI to fix it.
        """
        self.state = "PAUSED_FOR_AI_FIX"
        self.error_history.append(traceback)
        print(f"[APCB] Process {self.pid} crashed with intent '{self.intent}'. Handoff to SigmaAssistant.")
        # Trigger Intent Scheduler to resolve

class ProcessManager:
    def __init__(self):
        self.processes: Dict[int, APCB] = {}

    def spawn(self, pid: int, intent: str) -> APCB:
        process = APCB(pid=pid, intent=intent)
        self.processes[pid] = process
        return process
