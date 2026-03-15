# Generated method: RepoSyncPro._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import subprocess
import threading
import os

class RepoSyncPro:
    def _build_ui(self):
        self.header = tk.Frame(self, bg=PAL['bg'], height=60, padx=20)
        self.header.pack(side='top', fill='x', pady=10)
        tk.Label(self.header, text='REPO SYNC APEX PRO', font=('Inter', 16, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        nav_btns = [('🔄 AUTO-SYNC', self._start_sync), ('🔍 AUDIT LATEST', self._audit_repo)]
        btn_fr = tk.Frame(self.header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        for txt, cmd in nav_btns:
            tk.Button(btn_fr, text=txt, font=('Inter', 8, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=12, pady=6, command=cmd).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=20, pady=10)
        self.workspace.pack(fill='both', expand=True)
        info_f = tk.Frame(self.workspace, bg=PAL['panel'], padx=15, pady=15)
        info_f.pack(fill='x', pady=(0, 15))
        tk.Label(info_f, text='SOVEREIGN LEDGER (GIT)', font=('Inter', 10, 'bold'), fg=PAL['dim'], bg=PAL['panel']).pack(anchor='w')
        tk.Label(info_f, text=f'LOCAL PATH: {self.repo_dir}', font=('Inter', 9, 'bold'), fg=PAL['text'], bg=PAL['panel']).pack(anchor='w', pady=5)
        term_fr = tk.Frame(self.workspace, bg=PAL['border'], padx=2, pady=2)
        term_fr.pack(fill='both', expand=True)
        self.terminal = tk.Text(term_fr, bg=PAL['sidebar'], fg=PAL['success'], font=('Consolas', 9), relief='flat')
        self.terminal.pack(fill='both', expand=True)
        self.terminal.insert(tk.END, '>>> INITIALIZING QUANTUM GIT PROTOCOLS...\n')
        self.terminal.insert(tk.END, f'>>> REPOSITORY PATH VERIFIED: {self.repo_dir}\n')
        self.terminal.config(state=tk.DISABLED)
        self.pbar = ttk.Progressbar(self.workspace, style='Git.TProgressbar', length=800, mode='determinate')
        self.pbar.pack(fill='x', pady=15)
        self.status = tk.Label(self, text='IDLE | AWAITING COMMAND', bg=PAL['accent_dim'], fg='white', font=('Inter', 8, 'bold'), pady=5)
        self.status.pack(side='bottom', fill='x')