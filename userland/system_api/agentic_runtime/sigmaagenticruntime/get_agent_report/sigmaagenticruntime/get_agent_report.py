# Generated method: SigmaAgenticRuntime.get_agent_report
import time
import uuid
import threading
from typing import List, Dict, Any, Optional

class SigmaAgenticRuntime:
    def get_agent_report(self, job_id: str) -> Dict[str, Any]:
        if job_id not in self._active_agents:
            return {'error': 'Job not found.'}
        res: Dict[str, Any] = self._active_agents[job_id]
        return res