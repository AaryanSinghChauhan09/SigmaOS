# Generated method: SigmaAPITranslator.__init__
from enum import Enum
import time
import uuid

class SigmaAPITranslator:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self._stats = {'syscalls_translated': 0, 'latency_ms_avg': 0.05, 'apps_abstracted': 0}
        self._active_mappings = {'RegOpenKeyExW': 'kernel.registry.get_key', 'NSApplicationMain': 'gui.app_init', 'startActivity': 'kernel.event_bus.emit(app.launch)', 'fork': 'kernel.process_manager.spawn'}