# Generated method: SigmaAgentSandbox._run_process
import os
import shutil
import uuid
import time
import subprocess
import threading
from typing import Dict, Any, List

class SigmaAgentSandbox:
    def _run_process(self, silo_id: str, script_path: str):
        """Handles the actual subprocess execution with resource monitoring."""
        silo = self._active_silos[silo_id]
        try:
            time.sleep(0.5)
            log_path = os.path.join(silo['path'], 'agent.log')
            with open(log_path, 'w') as log_file:
                process = subprocess.Popen(['python', script_path], cwd=silo['path'], stdout=log_file, stderr=log_file, text=True)
                silo['pid'] = process.pid
                process.wait(timeout=30)
            silo['status'] = 'COMPLETED'
            self.kernel.bus.emit('sandbox.agent_success', {'silo': silo_id})
        except subprocess.TimeoutExpired:
            process.kill()
            silo['status'] = 'KILLED_TIMEOUT'
            silo['violations'].append('EXECUTION_TIMEOUT')
            self.kernel.ledger.commit('SANDBOX', 'VIOLATION', {'silo': silo_id, 'type': 'TIMEOUT'})
        except Exception as e:
            silo['status'] = 'FAILED'
            self.kernel.bus.emit('sandbox.agent_failure', {'silo': silo_id, 'error': str(e)})