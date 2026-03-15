# Generated method: ProcessOrchestrator.get_swarm_status
import threading
import time
import queue
from typing import Dict, Any, List, Callable
import os
import sys
from sigma_core.event_bus import EventBus

class ProcessOrchestrator:
    def get_swarm_status(self):
        return {'Workers': len(self._workers), 'Active': len([t for t in self.active_tasks.values() if t == 'EXECUTING']), 'Queued': self.task_queue.qsize()}