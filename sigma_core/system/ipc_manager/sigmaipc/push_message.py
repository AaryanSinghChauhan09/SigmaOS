# Generated method: SigmaIPC.push_message
import mmap
import os
import threading
from typing import Dict, List, Any

class SigmaIPC:
    def push_message(self, queue_id: str, message: bytes):
        """USP: Fast asynchronous message-pumping via queue (Sovereign Pumping)."""
        with self._lock:
            if queue_id not in self._message_queues:
                self._message_queues[queue_id] = []
            self._message_queues[queue_id].append(message)