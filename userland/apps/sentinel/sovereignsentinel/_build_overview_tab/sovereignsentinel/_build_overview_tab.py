# Generated method: SovereignSentinel._build_overview_tab
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess

class SovereignSentinel:
    def _build_overview_tab(self):
        frame = tk.Frame(self.nb, bg=PAL['bg'])
        self.nb.add(frame, text='  📊 Overview  ')
        metrics_fr = tk.Frame(frame, bg=PAL['bg'])
        metrics_fr.pack(fill='x', padx=20, pady=15)
        self._metric_vars = {}
        defs = [('cpu_lbl', 'CPU USAGE', '3.2%', PAL['safe']), ('ram_lbl', 'RAM ALLOC', '0.4 GB', PAL['safe']), ('io_lbl', 'I/O LATENCY', '0.18ms', PAL['accent']), ('mesh_lbl', 'MESH SYNC', '99.9%', PAL['safe']), ('thrt_lbl', 'THREATS', '0', PAL['safe'])]
        for key, label, init, color in defs:
            card = tk.Frame(metrics_fr, bg=PAL['panel'], padx=15, pady=15, highlightthickness=1, highlightbackground=PAL['border'])
            card.pack(side='left', fill='both', expand=True, padx=5)
            tk.Label(card, text=label, font=('Segoe UI', 8, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
            var = tk.StringVar(value=init)
            self._metric_vars[key] = (var, color, card)
            tk.Label(card, textvariable=var, font=('Segoe UI Bold', 20), fg=color, bg=PAL['panel']).pack(anchor='w', pady=5)
        viz_fr = tk.Frame(frame, bg=PAL['bg'])
        viz_fr.pack(fill='both', expand=True, padx=20)
        left_fr = tk.Frame(viz_fr, bg=PAL['panel'], width=450)
        left_fr.pack(side='left', fill='both', padx=(0, 10))
        left_fr.pack_propagate(False)
        tk.Label(left_fr, text='ZERO-TRUST RADAR', font=('Segoe UI', 10, 'bold'), fg='white', bg=PAL['panel'], pady=10).pack()
        self.viz_canvas = tk.Canvas(left_fr, width=420, height=220, bg='#000', highlightthickness=0)
        self.viz_canvas.pack(pady=10)
        self._animate_radar(0)
        right_fr = tk.Frame(viz_fr, bg=PAL['panel'])
        right_fr.pack(side='left', fill='both', expand=True)
        tk.Label(right_fr, text='THREAT INTEL FEED', font=('Segoe UI', 8, 'bold'), fg=PAL['dim'], bg=PAL['panel'], pady=8, padx=10).pack(anchor='w')
        self.threat_log = scrolledtext.ScrolledText(right_fr, bg='#050508', fg=PAL['safe'], font=('Cascadia Code', 9), pady=8, padx=10, borderwidth=0, height=10)
        self.threat_log.pack(fill='both', expand=True)
        self._populate_threat_feed()