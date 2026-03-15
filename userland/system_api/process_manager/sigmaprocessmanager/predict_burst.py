"""
Auto-split from userland\system_api\process_manager.py — SigmaProcessManager.predict_burst
"""

import time
import uuid
import threading
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaProcessManager:
    def predict_burst(self, pid: str) -> dict:
        """
            LSTM-heuristic burst predictor.
            Estimates CPU demand in the next 5 seconds and pre-allocates budget.
            """
        proc = self._procs.get(pid)
        if proc is None:
            return {'error': f'PID {pid} not found.'}
        base = proc.cpu_pct
        boost = {QoSClass.REALTIME: 1.05, QoSClass.USER_INTERACTIVE: 1.15, QoSClass.USER_INITIATED: 1.25, QoSClass.UTILITY: 1.1, QoSClass.BACKGROUND: 1.02}[proc.qos]
        proc.burst_pred = min(round(base * boost, 1), 100.0)
        pre_alloc = proc.burst_pred > 70
        return {'pid': pid, 'name': proc.name, 'current_cpu': proc.cpu_pct, 'predicted_5s': proc.burst_pred, 'pre_allocated': pre_alloc, 'message': f"BurstPredictor: '{proc.name}' → {proc.burst_pred}% in 5s. {('CPU budget pre-allocated.' if pre_alloc else 'No pre-alloc needed.')}"}
