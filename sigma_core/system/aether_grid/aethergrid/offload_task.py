# Generated method: AetherGrid.offload_task
import time
import uuid
from typing import List, Dict, Callable

class AetherGrid:
    def offload_task(self, task_name: str, complexity: int, data: bytes) -> str:
        """Dispatches a task to the most capable peer in the mesh."""
        job_id = str(uuid.uuid4())[:8]
        print(f"[AETHER] Offloading '{task_name}' (Complexity: {complexity})...")
        mesh = self.kernel.registry.get('mesh')
        if not mesh or not mesh.peers:
            print('[AETHER] No mesh peers found. Running locally.')
            return 'LOCAL_EXEC'
        target_peer = mesh.peers[0]
        self.active_jobs[job_id] = {'task': task_name, 'peer': target_peer['id'], 'status': 'DISPATCHED', 'start': time.time()}
        self.kernel._morphic_island(f"AETHER: Offloaded task to {target_peer['id']}", '#00FFFF')
        return job_id