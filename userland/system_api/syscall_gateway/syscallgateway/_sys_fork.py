# Generated method: SyscallGateway._sys_fork
from typing import Dict, Any

class SyscallGateway:
    def _sys_fork(self, p):
        if hasattr(self.kernel, 'scheduler'):
            return self.kernel.scheduler.create_task('forked_process')
        return {'error': 'Scheduler offline'}