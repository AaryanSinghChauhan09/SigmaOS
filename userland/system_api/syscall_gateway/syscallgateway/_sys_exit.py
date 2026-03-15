# Generated method: SyscallGateway._sys_exit
from typing import Dict, Any

class SyscallGateway:
    def _sys_exit(self, p):
        return {'status': 'HALT', 'code': p.get('status', 0)}