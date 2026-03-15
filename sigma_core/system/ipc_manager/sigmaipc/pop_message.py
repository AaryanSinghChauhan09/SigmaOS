# Generated method: SigmaIPC.pop_message
import mmap
import os
import threading
from typing import Dict, List, Any

class SigmaIPC:
    def pop_message(self, queue_id: str) -> bytes | None:
        """USP: O(1) Pop for message-driven agents/apps."""
        with self._lock:
            q = self._message_queues.get(queue_id)
            if q and len(q) > 0:
                return q.pop(0)
        return None