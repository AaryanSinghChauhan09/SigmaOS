# Generated method: SyscallGateway._sys_zenith_mission
from typing import Dict, Any

class SyscallGateway:
    def _sys_zenith_mission(self, p):
        if hasattr(self.kernel, 'zenith'):
            return self.kernel.zenith.dispatch_mission(p.get('prompt'), p.get('nodes', []))
        return {'error': 'Zenith offline'}