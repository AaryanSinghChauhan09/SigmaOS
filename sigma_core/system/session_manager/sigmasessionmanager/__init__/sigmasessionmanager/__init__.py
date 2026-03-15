# Generated method: SigmaSessionManager.__init__
import time
import uuid
import threading
from dataclasses import dataclass, field

class SigmaSessionManager:
    def __init__(self, kernel=None):
        self.kernel = kernel
        self._active_sessions: dict[str, UserSession] = {}
        self._lock = threading.Lock()
        self._current_session_id: str | None = None