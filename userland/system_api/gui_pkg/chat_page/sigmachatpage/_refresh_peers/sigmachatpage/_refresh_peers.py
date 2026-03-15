# Generated method: SigmaChatPage._refresh_peers
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL

class SigmaChatPage:
    def _refresh_peers(self):
        if not self.engine:
            self.stats_var.set('Engine: OFFLINE\nE2EE: LOCKED')
            return
        self.peer_list.delete(0, 'end')
        for sid in self.engine.peers:
            self.peer_list.insert('end', f'🔒 {sid}')
        self.stats_var.set(f'Active Tunnels: {len(self.engine.peers)}\nE2EE: AES-256-GCM')
        self.after(5000, self._refresh_peers)