"""
Auto-split from userland\apps\nexus_ai.py — SovereignAINexus._build_ui
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import time, threading, random, os, sys, json



class SovereignAINexus:
    def _build_ui(self):
        head = tk.Frame(self, bg=PAL['header_bg'], height=70, padx=25)
        head.pack(side='top', fill='x')
        head.pack_propagate(False)
        tk.Label(head, text='🧬 SOVEREIGN AI NEXUS', font=('Segoe UI Bold', 16), fg=PAL['accent'], bg=PAL['header_bg']).pack(side='left', pady=18)
        self.status_dot = tk.Label(head, text='● AGENT ACTIVE', font=('Segoe UI', 8, 'bold'), fg=PAL['success'], bg=PAL['header_bg'])
        self.status_dot.pack(side='right', padx=10)
        body = tk.Frame(self, bg=PAL['bg'], padx=20, pady=15)
        body.pack(fill='both', expand=True)
        self.nb = ttk.Notebook(body)
        self.nb.pack(fill='both', expand=True)
        self._build_agent_tab()
        self._build_guide_tab()
        self._build_audit_tab()
        self._build_tasks_tab()
