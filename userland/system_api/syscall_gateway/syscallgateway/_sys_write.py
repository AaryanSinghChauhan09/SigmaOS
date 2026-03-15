# Generated method: SyscallGateway._sys_write
from typing import Dict, Any

class SyscallGateway:
    def _sys_write(self, p):
        if hasattr(self.kernel, 'fs'):
            return self.kernel.fs.create(p.get('path'), p.get('content', b''))
        return {'error': 'FS offline'}