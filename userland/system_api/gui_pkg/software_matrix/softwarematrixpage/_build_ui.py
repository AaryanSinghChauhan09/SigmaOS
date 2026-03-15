# Generated method: SoftwareMatrixPage._build_ui
import tkinter as tk
from tkinter import ttk
from .base_page import SigmaPage
from .styles import PAL, FONT_BOLD, FONT_SMALL

class SoftwareMatrixPage:
    def _build_ui(self):
        feat = tk.Frame(self, bg=PAL['bg'], height=200)
        feat.pack(fill='x', pady=10)
        feat.pack_propagate(False)
        for app in ['Aether Studio', 'Zenith Orchestrator', 'CodeForge Pro', 'Sigma Designer']:
            c = tk.Frame(feat, bg=PAL['bg2'], width=200, padx=10, pady=10)
            c.pack(side='left', padx=5, fill='y')
            tk.Label(c, text='📦', font=('Segoe UI', 32), bg=PAL['bg2']).pack()
            tk.Label(c, text=app, font=FONT_BOLD, fg='white', bg=PAL['bg2']).pack()
            ttk.Button(c, text='DEPLOY', command=lambda a=app: self.gui._log_voice(f'Deploying {a}...')).pack(pady=5)
        cat_f = tk.Frame(self, bg=PAL['bg'])
        cat_f.pack(fill='both', expand=True, pady=20)
        for cat in ['DevTools', 'AI & Math', 'Sovereign Productivity', 'Gaming', 'Security']:
            fr = self._card(cat_f, cat)
            fr.master.pack(side='left', fill='both', expand=True, padx=5)
            tk.Label(fr, text=f'Explore {cat} apps...', bg=PAL['card'], fg=PAL['dim'], font=FONT_SMALL).pack()