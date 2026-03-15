# Generated method: UnivHubPage._build_ui
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO

class UnivHubPage:
    def _build_ui(self):
        body = tk.Frame(self, bg=PAL['bg'])
        body.pack(fill='both', expand=True)
        l_fr = tk.Frame(body, bg=PAL['bg2'], width=460)
        l_fr.pack(side='left', fill='both', padx=5)
        l_fr.pack_propagate(False)
        lin_c = self._card(l_fr, '🐧 Open-Source Parity (Linux Distros)')
        lin_c.master.pack(fill='x', pady=5)
        ttk.Button(lin_c, text='Launch Sovereign Cube', command=lambda: self.gui._log_voice('Launching Sovereign Cube...')).pack(side='left', padx=5)
        mac_c = self._card(l_fr, '🍏 Creative Ease Parity (macOS)')
        mac_c.master.pack(fill='x', pady=5)
        ttk.Button(mac_c, text='Temporal Snapshot', command=lambda: self.gui._log_voice('Taking Temporal Snapshot...')).pack(side='left', padx=5)
        r_fr = tk.Frame(body, bg=PAL['bg'])
        r_fr.pack(side='left', fill='both', expand=True, padx=5)
        console_c = self._card(r_fr, '📟 Universal Parity Runtime Console')
        console_c.master.pack(fill='both', expand=True)
        self.univ_log = self._console(console_c, height=25)
        self.univ_log.pack(fill='both', expand=True)
        self._log(self.univ_log, 'Universal Bridge ARMED. Monitoring cross-platform syscalls...', 'HEAD')