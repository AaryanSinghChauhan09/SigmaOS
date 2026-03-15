# Generated method: SigmaSessionManager.health_check
import time
import uuid
import threading
from dataclasses import dataclass, field

class SigmaSessionManager:
    def health_check(self) -> str:
        return f'OK — Active Sessions: {len(self._active_sessions)} | Current: {self._current_session_id}'