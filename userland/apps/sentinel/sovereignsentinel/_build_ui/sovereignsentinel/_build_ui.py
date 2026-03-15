# Generated method: SovereignSentinel._build_ui
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess

class SovereignSentinel:
    def _build_ui(self):
        head = tk.Frame(self, bg=PAL['bg'], padx=30, pady=20)
        head.pack(fill='x')
        tk.Label(head, text='⚔ SECURITY GUARDIAN', font=('Segoe UI', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        self.global_status = tk.Label(head, text='● SYSTEM HARDENED', font=('Segoe UI', 10, 'bold'), fg=PAL['safe'], bg=PAL['bg'], padx=20)
        self.global_status.pack(side='right')
        self.nb = ttk.Notebook(self)
        self.nb.pack(fill='both', expand=True, padx=20, pady=(0, 10))
        self._build_overview_tab()
        self._build_processes_tab()
        self._build_firewall_tab()
        self._build_scanner_tab()
        self._build_audit_tab()
        self.status = tk.Label(self, text='GUARDIAN ACTIVE | LEDGER: SYNCHRONIZED | PROTECTION: ABSOLUTE', bg=PAL['safe'], fg='white', font=('Segoe UI', 8, 'bold'), pady=5)
        self.status.pack(side='bottom', fill='x')