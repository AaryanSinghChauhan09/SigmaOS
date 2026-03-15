"""
Auto-split from userland\apps\omni_tweak_daemon.py — OmniTweakDaemon._build_de_tab
"""

import tkinter as tk
from tkinter import ttk, messagebox
import random
import time



class OmniTweakDaemon:
    def _build_de_tab(self):
        tk.Label(self.tab_de, text='ZERO-DOWNTIME DE HOT-SWAPPING', font=('Inter', 14, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        tk.Label(self.tab_de, text='Instantly switch Window Managers and Compositors without logging out (Usurping X11/Wayland limitations).', font=('Inter', 9), fg=PAL['dim'], bg=PAL['bg']).pack(anchor='w', pady=(0, 20))
        de_list = [('Sovereign Wayland (Default)', 'Hardware accelerated, tear-free compositor via Vulcan.', 'Active'), ('Aura Tiling WM', 'Keyboard-driven window manager (i3/bspwm usurp).', 'Standby'), ('Legacy X11 Matrix', 'Fallback compatibility architecture.', 'Standby'), ('CLI-Only Framebuffer', 'Raw TTY execution. Kills all GUI processes to save RAM.', 'Standby')]
        for name, desc, stat in de_list:
            f = tk.Frame(self.tab_de, bg=PAL['panel'], pady=15, padx=20)
            f.pack(fill='x', pady=5)
            tk.Label(f, text=name, font=('Inter', 11, 'bold'), fg=PAL['text'], bg=PAL['panel']).pack(anchor='w')
            tk.Label(f, text=desc, font=('Inter', 9), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(5, 0))
            btn_col = PAL['success'] if stat == 'Active' else PAL['sidebar']
            btn_txt = 'ACTIVE' if stat == 'Active' else 'HOT-SWAP DE'
            tk.Button(f, text=btn_txt, bg=btn_col, fg='white', font=('Inter', 8, 'bold'), relief='flat', command=lambda n=name: self._swap_de(n)).pack(side='right', pady=5)
