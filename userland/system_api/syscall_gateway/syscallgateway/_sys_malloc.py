# Generated method: SyscallGateway._sys_malloc
from typing import Dict, Any

class SyscallGateway:
    def _sys_malloc(self, p):
        if hasattr(self.kernel, 'memory'):
            return self.kernel.memory.alloc('ring3_app', p.get('size', 1))
        return {'error': 'MemMgr offline'}