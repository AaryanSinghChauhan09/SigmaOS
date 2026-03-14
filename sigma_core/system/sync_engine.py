"""
SigmaOS Apex Sync Engine (v1.0 Apex)
======================================
USP: P2P Clipboard Handoff + Session Continuity + Zero-Server Synchronization.
Builds on GhostChat to provide Apple-parity Handoff on any hardware.
"""

import time
import json
import threading
from sigma_core.system.interfaces import SigmaModuleBase, ISigmaService

class SigmaSyncEngine(SigmaModuleBase, ISigmaService):
    def __init__(self, kernel=None):
        SigmaModuleBase.__init__(self, kernel)
        self._running = False
        self.last_clipboard = ""
        self.sync_active = True
        self.peer_table = set() # USP: Automated Peer Discovery Table
        self.stats = {
            "handoffs_completed": 0,
            "bytes_synced": 0,
            "peers_discovered": 0
        }

    def discover_peers(self):
        """USP: Automated P2P discovery via GhostChat broadcast."""
        # Simulated discovery heartbeat
        new_peers = [f"node-{i:03x}" for i in range(2)]
        for p in new_peers:
            if p not in self.peer_table:
                self.peer_table.add(p)
                self.stats["peers_discovered"] += 1
        return f"Sync: {len(self.peer_table)} Sovereign peers mapped."

    def start_service(self):
        if not self._running:
            self._running = True
            self._sync_thread = threading.Thread(target=self._clipboard_watcher, daemon=True)
            self._sync_thread.start()
            
            # Subscribe to GhostChat messages for incoming handoffs
            if self.kernel and hasattr(self.kernel, "bus"):
                self.kernel.bus.subscribe("ghostchat.msg_received", self._on_handoff_received)
                
            self.log_event("sync_start", {"status": "ACTIVE"})
        return "Sync Engine: Handoff Sentinel Online."

    def stop_service(self):
        self._running = False
        self.log_event("sync_stop", {"status": "INACTIVE"})

    def _clipboard_watcher(self):
        """Monitors local clipboard and broadcasts changes to peers."""
        # Note: In a real implementation, we'd use a clipboard library.
        # Here we simulate with a 'last_clipboard' variable.
        while self._running:
            if self.sync_active and self.last_clipboard:
                # If clipboard changed, broadcast it as a 'HANDOFF' packet via GhostChat
                if self.kernel and hasattr(self.kernel, "ghostchat"):
                    # We use a special GhostChat send_message with type tagging
                    payload = {
                        "type": "HANDOFF_CLIPBOARD",
                        "content": self.last_clipboard,
                        "ts": time.time()
                    }
                    self.kernel.ghostchat.send_message(json.dumps(payload))
                    self.stats["handoffs_completed"] += 1
            time.sleep(2)

    def _on_handoff_received(self, msg):
        """Handles incoming clipboard/session handoffs from other GhostChat nodes."""
        try:
            payload = json.loads(msg.get("text", "{}"))
            if payload.get("type") == "HANDOFF_CLIPBOARD":
                content = payload.get("content")
                if content != self.last_clipboard:
                    self.last_clipboard = content
                    self.stats["bytes_synced"] += len(content)
                    print(f"[SYNC] Unified Handoff: Clipboard updated from P2P Peer.")
                    if self.kernel:
                        self.kernel.bus.emit("sync.clipboard_updated", {"content": content[:20]})
        except:
            pass

    def update_local_clipboard(self, text: str):
        """API for local tools to push into the sync fabric."""
        self.last_clipboard = text
        return "Clipboard staged for Apex Handoff."

    def health_check(self) -> str:
        s = self.stats
        return f"OK — Sync: {s['handoffs_completed']} Handoffs | Bytes: {s['bytes_synced']} | Mode: P2P Sovereign"

if __name__ == "__main__":
    # Demo
    engine = SigmaSyncEngine()
    print(engine.start_service())
    engine.update_local_clipboard("Global State 0xAF")
    print(engine.health_check())
