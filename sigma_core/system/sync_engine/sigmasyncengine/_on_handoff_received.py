# Generated method: SigmaSyncEngine._on_handoff_received
import time
import json
import threading
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaSyncEngine:
    def _on_handoff_received(self, msg):
        """Handles incoming clipboard/session handoffs from other GhostChat nodes."""
        try:
            payload = json.loads(msg.get('text', '{}'))
            if payload.get('type') == 'HANDOFF_CLIPBOARD':
                content = payload.get('content')
                if content != self.last_clipboard:
                    self.last_clipboard = content
                    self.stats['bytes_synced'] += len(content)
                    print(f'[SYNC] Unified Handoff: Clipboard updated from P2P Peer.')
                    if self.kernel:
                        self.kernel.bus.emit('sync.clipboard_updated', {'content': content[:20]})
        except:
            pass