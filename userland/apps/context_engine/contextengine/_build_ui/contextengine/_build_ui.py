# Generated method: ContextEngine._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import time
import random

class ContextEngine:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='CONTEXT ENGINE (AUTO-MODES)', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='📡 SCAN ENVIRONMENT', font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=self._mock_scan).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.list_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=280, padx=15, pady=15)
        self.list_fr.pack(side='left', fill='y', padx=(0, 20))
        self.list_fr.pack_propagate(False)
        tk.Label(self.list_fr, text='SITUATIONAL VECTORS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 10))
        modes = [('🎮 HYPER-GAME MODE', 'Overclocks GPU, suspends background threads. (Windows Game Mode)', PAL['game']), ('💻 DESKTOP EXTEND (DeX)', 'Adapts UI to 8K external monitor array. (Samsung DeX)', PAL['dex']), ('🛌 NEURAL BEDTIME', 'Gamma shift, mute alerts, underclock CPU. (iOS Bedtime)', PAL['zen']), ('🚗 VELOCITY (DRIVING)', 'Read-aloud telemetry, large touch targets. (Android Auto)', PAL['accent']), ('🔋 ABSOLUTE SURVIVAL', 'Kills 99% processes. CLI only. (Extreme Battery Saver)', PAL['danger']), ('🔒 LOCKED KIOSK', 'Pins Single App. Disables Esc/Alt-Tab. (ChromeOS Kiosk)', '#9D4EDD')]
        self.mode_btns = []
        for title, desc, col in modes:
            f = tk.Frame(self.list_fr, bg=PAL['sidebar'], pady=10, padx=10, cursor='hand2')
            f.pack(fill='x', pady=5)
            tk.Label(f, text=title, font=('Inter', 9, 'bold'), fg=col, bg=PAL['sidebar']).pack(anchor='w')
            tk.Label(f, text=desc, font=('Inter', 8), fg=PAL['dim'], bg=PAL['sidebar'], wraplength=220, justify='left').pack(anchor='w')
            f.bind('<Button-1>', lambda e, m=title, c=col: self._activate_mode(m, c))
            self.mode_btns.append(f)
        self.ai_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.ai_fr.pack(side='left', fill='both', expand=True)
        tk.Label(self.ai_fr, text='CONTEXTUAL INTELLIGENCE (AUTO-TRIGGERS)', font=('Inter', 12, 'bold'), fg=PAL['text'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        self.term = tk.Text(self.ai_fr, bg=PAL['panel'], fg=PAL['success'], font=('Consolas', 10), relief='flat')
        self.term.pack(fill='both', expand=True, pady=10, padx=5)
        self.term.insert(tk.END, '>>> [CONTEXT ENGINE ONLINE]\n')
        self.term.insert(tk.END, '>>> LISTENING TO ACCELEROMETER, BLUETOOTH, & GPS...\n')
        self.term.config(state=tk.DISABLED)
        self.status = tk.Label(self, text=f'CURRENT PARADIGM: {self.active_mode}', bg=PAL['accent_dim'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')