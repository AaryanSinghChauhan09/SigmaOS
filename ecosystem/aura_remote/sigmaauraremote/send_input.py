# Generated method: SigmaAuraRemote.send_input
from typing import Dict, List, Any
import time

class SigmaAuraRemote:
    def send_input(self, session_id: str, input_type: str, data: Any) -> str:
        """Sends Mouse, Keyboard, or Media input to a remote PC."""
        if session_id in self._active_connections:
            return f"⌨ Input Sent: {input_type} ({data}) to {self._active_connections[session_id]['host']}."
        return 'Error: Session expired or invalid.'