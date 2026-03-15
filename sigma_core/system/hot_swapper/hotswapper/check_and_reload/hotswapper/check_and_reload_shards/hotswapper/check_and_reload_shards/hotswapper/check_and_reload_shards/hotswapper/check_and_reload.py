# Part of SigmaOS Omega - (High Cohesion, Loose Coupling)
# Principle: Single Responsibility per File

from abc import ABC, abstractmethod
from sigma_core.security.resilience_guard import resilient_module
from sigma_core.security.resilience_guard import resilient_module
import time
import os
import importlib

class HotSwapper:
    @resilient_module
    def check_and_reload(self):
        reload_count = 0
        for path, last_mtime in self.watch_list.items():
            current_mtime = os.path.getmtime(path)
            if current_mtime > last_mtime:
                self.watch_list[path] = current_mtime
                reload_count += 1
        return reload_count