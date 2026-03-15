# Generated method: ConfigHubPage._cfg_sovereignty
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_MED, FONT_BOLD, FONT_TITLE, FONT_SMALL

class ConfigHubPage:
    def _cfg_sovereignty(self, parent):
        tk.Label(parent, text='Competitive Absorption & AI Sovereignty', font=FONT_TITLE, fg='white', bg=PAL['bg']).pack(anchor='w', pady=10)
        zen = self.kernel.registry.get('zenith')
        status = zen.health_check() if zen else 'Zenith Core Offline'
        tk.Label(parent, text=f'Zenith Status: {status}', font=FONT_MED, fg=PAL['cyan'], bg=PAL['bg']).pack(anchor='w', pady=5)
        info = self._card(parent, 'Competitive Absorption Bridges')
        info.master.pack(fill='x', pady=5)
        bridges = [('Win32 Bridge', '0ms DLL Emulation', 'ENABLED'), ('macOS Cocoa Proxy', 'Retina Compositing', 'ENABLED'), ('Antigravity Suite', 'Full Native Integration', 'ACTIVE')]
        for b, d, s in bridges:
            f = tk.Frame(info, bg=PAL['card'])
            f.pack(fill='x', pady=2)
            tk.Label(f, text=f'{b}: {d}', font=FONT_SMALL, fg=PAL['text'], bg=PAL['card']).pack(side='left')
            tk.Label(f, text=s, font=('Inter Bold', 7), fg=PAL['gold'], bg=PAL['card']).pack(side='right')