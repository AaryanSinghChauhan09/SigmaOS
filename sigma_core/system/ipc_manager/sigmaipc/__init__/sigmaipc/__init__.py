# Generated method: SigmaIPC.__init__
import mmap
import os
import threading
from typing import Dict, List, Any

class SigmaIPC:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self._shared_buffers: Dict[str, mmap.mmap] = {}
        self._message_queues: Dict[str, List[bytes]] = {}
        self._lock = threading.Lock()