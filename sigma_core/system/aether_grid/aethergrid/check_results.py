# Generated method: AetherGrid.check_results
import time
import uuid
from typing import List, Dict, Callable

class AetherGrid:
    def check_results(self, job_id: str) -> Dict:
        """Polls for completion of the offloaded task."""
        if job_id not in self.active_jobs:
            return {'status': 'NOT_FOUND'}
        job = self.active_jobs[job_id]
        if time.time() - job['start'] > 2:
            job['status'] = 'COMPLETED'
            return {'status': 'SUCCESS', 'result': 'Aether-Processed Data'}
        return {'status': 'WORKING'}