# Generated method: AegisPermissions._build_ui
import tkinter as tk
from tkinter import ttk, messagebox

class AegisPermissions:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        self.header.pack(side='top', fill='x', pady=15)
        tk.Label(self.header, text='AEGIS VAULT', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tk.Button(btn_fr, text='🚨 REVOKE ALL TOKENS', font=('Inter', 9, 'bold'), bg=PAL['danger'], fg='white', relief='flat', padx=15, pady=8, command=self._revoke_all).pack(side='left')
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.tabs = ttk.Notebook(self.workspace, style='Aegis.TNotebook')
        self.tabs.pack(fill='both', expand=True)
        self.tab_apps = tk.Frame(self.tabs, bg=PAL['bg'], padx=15, pady=15)
        self.tabs.add(self.tab_apps, text='VECTORS (APPS)')
        self.tab_perms = tk.Frame(self.tabs, bg=PAL['bg'], padx=15, pady=15)
        self.tabs.add(self.tab_perms, text='HARDENED PERMISSIONS')
        self.tab_logs = tk.Frame(self.tabs, bg=PAL['bg'], padx=15, pady=15)
        self.tabs.add(self.tab_logs, text='AUDIT LEDGER')
        self._build_apps_tab()
        self._build_logs_tab()
        self.status = tk.Label(self, text='ZERO-TRUST ENFORCED | NO ESCALATION VECTORS DETECTED', bg=PAL['success'], fg='black', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')