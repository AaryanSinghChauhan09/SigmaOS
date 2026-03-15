"""
Auto-split from ecosystem\sigma_automation_hub.py — SigmaOmniAutomator._automation_engine
"""

from typing import Callable, Dict, List, Any, Optional
import threading
import time
import random
import uuid



class SigmaOmniAutomator:
    def _automation_engine(self):
        """Monitors all triggers, schedules, and pipelines with zero-fail logic."""
        while self._running:
            try:
                now = time.time()
                due = [t for t in self._scheduled if t['time'] <= now]
                for task in due:
                    try:
                        threading.Thread(target=task['func'], daemon=True).start()
                    except Exception as e:
                        self._error_log.append(f"Scheduled task '{task.get('name')}' failed: {e}")
                    finally:
                        if task in self._scheduled:
                            self._scheduled.remove(task)
                for pipe_id, pipe in list(self._active_pipelines.items()):
                    if time.time() - pipe.get('start', time.time()) > 300:
                        pipe['status'] = 'TIMEOUT'
            except Exception as e:
                self._error_log.append(f'Automation engine cycle error: {e}')
            time.sleep(1.0)
