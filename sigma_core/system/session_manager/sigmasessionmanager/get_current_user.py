# Generated method: SigmaSessionManager.get_current_user
import time
import uuid
import threading
from dataclasses import dataclass, field

class SigmaSessionManager:
    def get_current_user(self) -> UserSession | None:
        sid = self._current_session_id
        if sid and isinstance(sid, str):
            return self._active_sessions.get(sid)
        return None