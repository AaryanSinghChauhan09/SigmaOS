# Generated method: OmniBrowser._build_ui
import tkinter as tk
from tkinter import ttk, messagebox, simpledialog
import random
import time
from typing import Any, List
from sigma_core.ui.fluid_design import ICONS

class OmniBrowser:
    def _build_ui(self):
        self.tab_container = tk.Frame(self, bg=PAL['bg'], height=40)
        self.tab_container.pack(side='top', fill='x')
        self.tab_nb = ttk.Notebook(self.tab_container, style='Omni.TNotebook')
        self.tab_nb.pack(side='left', fill='x', expand=True)
        for t in self.tabs:
            name = t.split('//')[-1] if '//' in t else t
            self.tab_nb.add(tk.Frame(self.tab_nb), text=f" {ICONS.get('browser', '🌐')} {name} ")
        tk.Button(self.tab_container, text=' + ', bg=PAL['bg'], fg=PAL['accent'], relief='flat', font=('Inter', 12, 'bold')).pack(side='left', padx=10)
        self.addr_fr = tk.Frame(self, bg=PAL['toolbar'], height=60, pady=10, padx=20)
        self.addr_fr.pack(side='top', fill='x')
        nav_fr = tk.Frame(self.addr_fr, bg=PAL['toolbar'])
        nav_fr.pack(side='left')
        nav_controls = [('minimalist', '⏮'), ('code', '⏭'), ('perf', '🔄')]
        for icon_key, fallback in nav_controls:
            tk.Button(nav_fr, text=ICONS.get(icon_key, fallback), font=('Inter', 14), bg=PAL['toolbar'], fg='white', relief='flat', padx=10).pack(side='left')
        self.url_entry = tk.Entry(self.addr_fr, bg='#000000', fg=PAL['text'], font=('Inter', 11), borderwidth=0, insertbackground='white', highlightthickness=1, highlightbackground=PAL['border'])
        self.url_entry.pack(side='left', padx=20, fill='x', expand=True, pady=2)
        self.url_entry.insert(0, 'omni.sigma://secure_search')
        self.url_entry.bind('<Return>', self.navigate)
        self.shield_btn = tk.Button(self.addr_fr, text=f"{ICONS.get('warden', '🛡️')} SAFE", font=('Inter', 9, 'bold'), bg=PAL['success'], fg='white', relief='flat', padx=15, command=self._show_security_panel)
        self.shield_btn.pack(side='left', padx=5)
        tk.Button(self.addr_fr, text=f"{ICONS.get('intelligence', '📤')} BEAM", font=('Inter', 9, 'bold'), bg=PAL['accent'], fg='white', relief='flat', padx=15, command=self._trigger_handoff).pack(side='left', padx=5)
        self.viewport = tk.Frame(self, bg='#FFFFFF', padx=50, pady=50)
        self.viewport.pack(fill='both', expand=True)
        self.render_lbl = tk.Label(self.viewport, text='OMNIBROWSER CORE v3.0\n\n- Neural Ad-Shield Active\n- Zero Trackers Detected\n- Sandboxed File Access Only', bg='white', font=('Inter Light', 24), fg='#111', justify='center')
        self.render_lbl.pack(expand=True)
        self.status = tk.Label(self, text='SOVEREIGN SHIELD: 100% | ENCRYPTION: SHA-3 | LATENCY: 2ms', bg=PAL['accent'], fg='white', font=('Inter', 8, 'bold'), pady=5)
        self.status.pack(side='bottom', fill='x')