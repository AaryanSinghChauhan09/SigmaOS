# Generated method: SigmaAdaptiveKernel.classify_workload
import time
import threading
from enum import Enum, auto

class SigmaAdaptiveKernel:
    def classify_workload(self, active_processes: list[str]) -> WorkloadProfile:
        """
                ML-heuristic classifier: scans active process names for workload signals.
                Returns the dominant WorkloadProfile.
                """
        scores: dict[WorkloadProfile, int] = {p: 0 for p in WorkloadProfile}
        for proc in active_processes:
            proc_lower = proc.lower()
            for keyword, profile in _SIGNAL_MAP.items():
                if keyword in proc_lower:
                    scores[profile] += 1
        scores[WorkloadProfile.IDLE] += 0
        winner = max(scores, key=lambda p: scores[p])
        return winner if scores[winner] > 0 else WorkloadProfile.BALANCED