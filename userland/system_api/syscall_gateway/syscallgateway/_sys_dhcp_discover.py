# Generated method: SyscallGateway._sys_dhcp_discover
from typing import Dict, Any

class SyscallGateway:
    def _sys_dhcp_discover(self, p):
        if hasattr(self.kernel, 'network'):
            return self.kernel.network.dhcp_discover()
        return {'error': 'Network stack offline'}