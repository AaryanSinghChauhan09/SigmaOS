# Generated method: ConfigHubPage._cfg_security
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_LOGO, FONT_MED, FONT_BOLD, FONT_TITLE, FONT_SMALL

class ConfigHubPage:
    def _cfg_security(self, parent):
        tk.Label(parent, text='Sovereign Security & Hardening', font=FONT_TITLE, fg='white', bg=PAL['bg']).pack(anchor='w', pady=10)
        sh = self.kernel.registry.get('security_hardening')
        posture = sh.get_security_posture() if sh else {}
        info = self._card(parent, 'Live Security Posture')
        info.master.pack(fill='x', pady=5)
        for k, v in posture.items():
            f = tk.Frame(info, bg=PAL['card'])
            f.pack(fill='x', pady=2)
            tk.Label(f, text=k.replace('_', ' '), font=FONT_SMALL, fg=PAL['dim'], bg=PAL['card']).pack(side='left')
            tk.Label(f, text=v, font=FONT_BOLD, fg=PAL['teal'], bg=PAL['card']).pack(side='right')
        ttk.Button(parent, text='🛡️ Rotate Memory Canaries', command=lambda: self.gui._notify('Security', 'Memory Shadowing Canaries Rotated.', 'OK')).pack(anchor='w', pady=10)