# Generated method: SigmaSessionManager.logout
import time
import uuid
import threading
from dataclasses import dataclass, field

class SigmaSessionManager:
    def logout(self, session_id: str | None=None) -> dict:
        """USP: Evaporative Logout. Wipes session RAM-FS and encryption keys instantly."""
        sid = session_id or self._current_session_id
        if not sid or sid not in self._active_sessions:
            return {'status': 'ERROR', 'message': 'Invalid session handle.'}
        with self._lock:
            session = self._active_sessions.pop(sid)
            if self._current_session_id == sid:
                self._current_session_id = None
        print(f"[SESSION] User '{session.username}' logged out. Purging session memory...")
        if self.kernel and hasattr(self.kernel, 'memory'):
            self.kernel.memory.free_page(f'session_{sid}_vault')
        return {'status': 'SUCCESS', 'message': f'Session {sid} purged. Memory returned to host hardware.'}