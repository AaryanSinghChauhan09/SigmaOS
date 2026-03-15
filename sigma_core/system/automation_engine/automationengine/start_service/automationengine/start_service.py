# Generated method: AutomationEngine.start_service
import time
import threading
from typing import Dict, Any, List, Callable, Optional

class AutomationEngine:
    def start_service(self) -> str:
        self._running = True
        t = threading.Thread(target=self._automation_loop, daemon=True)
        self._loop_thread = t
        t.start()
        return 'Sovereign Automation: Apex Orchestrator Online.'