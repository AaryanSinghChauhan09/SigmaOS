# Generated method: SigmaAuraRemote.health_check
from typing import Dict, List, Any
import time

class SigmaAuraRemote:
    def health_check(self) -> str:
        s = self._stats
        return f"OK — Remotes: {s['remotes_mirrored']}, IoT Ops: {s['iot_commands_sent']}, PC Sessions: {s['pc_remote_sessions']}."