"""
Auto-split from userland\system_api\process_manager.py — SigmaProcessManager.predict_all_bursts
"""

import time
import uuid
import threading
from dataclasses import dataclass, field
from enum import Enum, auto



class SigmaProcessManager:
    def predict_all_bursts(self) -> dict:
        results = {}
        for pid in list(self._procs.keys()):
            results[pid] = self.predict_burst(pid)
        high_burst = [r['name'] for r in results.values() if isinstance(r, dict) and r.get('predicted_5s', 0) > 70]
        return {'predictions': results, 'high_burst_procs': high_burst, 'message': f'BurstPredictor: {len(results)} processes analysed. {len(high_burst)} flagged for CPU pre-allocation.'}
