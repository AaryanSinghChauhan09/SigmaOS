# Generated method: VisionExplorer._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import random
import math
from typing import Dict, Any, List, Optional

class VisionExplorer:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=80, padx=20)
        self.header.pack(side='top', fill='x', pady=20)
        tk.Label(self.header, text='SOVEREIGN VISION EXPLORER', font=('Inter', 22, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        opts = [('🔍 DEEP SCAN', self._deep_scan), ('🔄 RE-MAP', self._remap)]
        for txt, cmd in opts:
            tk.Button(btn_fr, text=txt, font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=cmd).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=20)
        self.workspace.pack(fill='both', expand=True)
        self.index_fr = tk.Frame(self.workspace, bg=PAL['panel'], width=280, padx=15, pady=15)
        self.index_fr.pack(side='left', fill='y', padx=(0, 20))
        self.index_fr.pack_propagate(False)
        tk.Label(self.index_fr, text='ACTIVE SHARDS', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w', pady=(0, 10))
        shards = ['KERNEL', 'HAL', 'MESH', 'STEALTH', 'LEGAL', 'ANALYTIC', 'GAMIFICATION']
        for s in shards:
            f = tk.Frame(self.index_fr, bg=PAL['sidebar'], pady=8, padx=10)
            f.pack(fill='x', pady=3)
            tk.Label(f, text=f'• {s}', font=('Inter', 9), fg=PAL['text'], bg=PAL['sidebar']).pack(anchor='w')
        self.canvas_fr = tk.Frame(self.workspace, bg=PAL['bg'])
        self.canvas_fr.pack(side='left', fill='both', expand=True)
        self.canvas = tk.Canvas(self.canvas_fr, bg=PAL['sidebar'], highlightthickness=0)
        self.canvas.pack(fill='both', expand=True)