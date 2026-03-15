# Generated method: SyscallGateway._sys_ls_initrd
from typing import Dict, Any

class SyscallGateway:
    def _sys_ls_initrd(self, p):
        if hasattr(self.kernel, 'fs'):
            return [path for path in self.kernel.fs._inodes.keys() if path.startswith('/initrd/')]
        return {'error': 'FS offline'}