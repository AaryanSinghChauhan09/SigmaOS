# Generated method: AutomationEngine.health_check
import time
import threading
from typing import Dict, Any, List, Callable, Optional

class AutomationEngine:
    def health_check(self) -> str:
        return f'OK — Active Recipes: {len(self.workflows)} | Scheduled: {len(self.scheduled_tasks)}'