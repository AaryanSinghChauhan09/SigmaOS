# Generated method: SyscallGateway._sys_read
from typing import Dict, Any

class SyscallGateway:
    def _sys_read(self, p):
        if hasattr(self.kernel, 'fs'):
            return self.kernel.fs.read(p.get('path'))
        return {'error': 'FS offline'}