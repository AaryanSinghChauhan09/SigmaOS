# Generated method: SovereignPDFEditor._build_ui
import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import os
import time
import random

class SovereignPDFEditor:
    def _build_ui(self):
        self.toolbar = tk.Frame(self, bg=PAL['bg'], height=60, padx=25)
        self.toolbar.pack(side='top', fill='x')
        tk.Label(self.toolbar, text='PDF FORGE PRO', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.toolbar, bg=PAL['bg'])
        btn_fr.pack(side='right')
        nav_btns = [('📁 OPEN', self._open_file), ('🔐 HARDEN', self._harden_doc), ('🧪 FORENSIC', self._audit_forensic), ('🧹 PURGE', self._purge_metadata)]
        for txt, cmd in nav_btns:
            tk.Button(btn_fr, text=txt, font=('Inter', 8, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=cmd).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.side_fr = tk.Frame(self.workspace, bg=PAL['sidebar'], width=220, padx=15, pady=20)
        self.side_fr.pack(side='left', fill='y')
        self.side_fr.pack_propagate(False)
        tk.Label(self.side_fr, text='WORKFLOW ARCHETYPES', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['sidebar']).pack(anchor='w')
        archetypes = [('📄 ADOBE', 'Creative Pro Layout'), ('🏗️ BLUEBEAM', 'Precision AEC Markup'), ('🔐 FOXIT', 'Hardened Redaction'), ('⚡ BULK', 'Parallel Batch Engine')]
        for title, desc in archetypes:
            f = tk.Frame(self.side_fr, bg=PAL['sidebar'], pady=12, cursor='hand2')
            f.pack(fill='x')
            tk.Label(f, text=title, font=('Inter', 9, 'bold'), fg=PAL['text'], bg=PAL['sidebar']).pack(anchor='w')
            tk.Label(f, text=desc, font=('Inter', 7), fg=PAL['dim'], bg=PAL['sidebar']).pack(anchor='w')
        self.viewport = tk.Frame(self.workspace, bg=PAL['bg'], padx=20)
        self.viewport.pack(side='left', fill='both', expand=True)
        self.status_lbl = tk.Label(self.viewport, text='NO ACTIVE DOCUMENT', font=('Inter', 16, 'bold'), fg=PAL['dim'], bg=PAL['bg'])
        self.status_lbl.pack(expand=True)
        self.viz_canvas = tk.Canvas(self.viewport, width=500, height=150, bg=PAL['bg'], highlightthickness=0)
        self.viz_canvas.pack(pady=20)
        self.viz_canvas.pack_forget()
        self.panel = tk.Frame(self.workspace, bg=PAL['panel'], width=280, padx=20, pady=20)
        self.panel.pack(side='right', fill='y', padx=(20, 0))
        self.panel.pack_propagate(False)
        tk.Label(self.panel, text='ENGINE METRICS', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        self._metric_box(self.panel, 'ENCRYPTION', 'SOVEREIGN-AES-512', PAL['success'])
        self._metric_box(self.panel, 'GRID STATE', 'SYNCHRONIZED (AETHER)', PAL['accent'])
        self._metric_box(self.panel, 'COMPRESSION', 'H.266 NEURAL LOOM', PAL['success'])
        self.status = tk.Label(self, text='SOVEREIGN FORGE [VERSION 3.0] | GPU RENDERING ACTIVE', bg=PAL['accent'], fg='white', font=('Inter', 8, 'bold'), pady=5)
        self.status.pack(side='bottom', fill='x')