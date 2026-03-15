# Generated method: SyscallGateway.__init__
from typing import Dict, Any

class SyscallGateway:
    def __init__(self, kernel):
        self.kernel = kernel
        self.handlers = {1: self._sys_exit, 2: self._sys_fork, 3: self._sys_read, 4: self._sys_write, 5: self._sys_open, 6: self._sys_close, 16: self._sys_malloc, 32: self._sys_ping, 33: self._sys_dhcp_discover, 48: self._sys_ls_initrd, 128: self._sys_zenith_mission}