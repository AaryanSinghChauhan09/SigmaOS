# Generated method: SigmaIPC.create_shared_memory
import mmap
import os
import threading
from typing import Dict, List, Any

class SigmaIPC:
    def create_shared_memory(self, name: str, size: int=4096) -> bool:
        """
            USP: Direct RAM Allocation for cross-process data.
            Bypasses standard socket/disk I/O.
            """
        try:
            if os.name == 'nt':
                buf = mmap.mmap(-1, size, tagname=f'sigma_ipc_{name}')
            else:
                buf = mmap.mmap(-1, size, flags=mmap.MAP_PRIVATE | mmap.MAP_ANONYMOUS)
            with self._lock:
                self._shared_buffers[name] = buf
            return True
        except Exception as e:
            print(f'[IPC] Error creating shared memory {name}: {e}')
            return False