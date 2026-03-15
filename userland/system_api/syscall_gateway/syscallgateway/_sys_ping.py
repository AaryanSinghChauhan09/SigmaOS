# Generated method: SyscallGateway._sys_ping
from typing import Dict, Any

class SyscallGateway:
    def _sys_ping(self, p):
        if hasattr(self.kernel, 'network'):
            return self.kernel.network.ping(p.get('target', '8.8.8.8'))
        return {'error': 'Network stack offline'}