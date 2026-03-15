# Generated method: EmailDisco._build_ui
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random
import time

class EmailDisco:
    def _build_ui(self):
        self.toolbar = tk.Frame(self, bg=PAL['bg'], height=60, padx=25)
        self.toolbar.pack(side='top', fill='x')
        tk.Label(self.toolbar, text='EMAIL AGENT', font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(self.toolbar, bg=PAL['bg'])
        btn_fr.pack(side='right')
        tools = [('🔄 SYNC', self._sync_threads), ('🚀 ANALYZE', self._analyze), ('🧹 PURGE', self._purge_spam)]
        for txt, cmd in tools:
            tk.Button(btn_fr, text=txt, font=('Inter', 8, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=cmd).pack(side='left', padx=5)
        self.workspace = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        self.workspace.pack(fill='both', expand=True)
        self.panes = ttk.PanedWindow(self.workspace, orient='horizontal')
        self.panes.pack(fill='both', expand=True)
        self.sidebar = tk.Frame(self.panes, bg=PAL['sidebar'], width=240, padx=20, pady=25)
        self.panes.add(self.sidebar, weight=1)
        self.sidebar.pack_propagate(False)
        tk.Label(self.sidebar, text='INTENT FOLDERS', font=('Inter', 8, 'bold'), fg=PAL['dim'], bg=PAL['sidebar']).pack(anchor='w')
        folders = [('🧠 Mission_Critical', 'accent'), ('🛒 Commerce', 'text'), ('🔍 Forensic_Audit', 'dim')]
        for f, col in folders:
            tk.Label(self.sidebar, text=f'• {f}', font=('Inter', 10), fg=PAL.get(col, PAL['text']), bg=PAL['sidebar'], pady=10, cursor='hand2').pack(anchor='w')
        self.middle_fr = tk.Frame(self.panes, bg=PAL['bg'], padx=15)
        self.panes.add(self.middle_fr, weight=3)
        self.tree = ttk.Treeview(self.middle_fr, columns=('Sender', 'Subject', 'Priority'), show='headings')
        self.tree.heading('Sender', text='SENDER')
        self.tree.heading('Subject', text='SUBJECT')
        self.tree.heading('Priority', text='PRIORITY')
        self.tree.column('Sender', width=120)
        self.tree.column('Subject', width=300)
        self.tree.column('Priority', width=100, anchor='center')
        self.tree.pack(fill='both', expand=True)
        self.right_fr = tk.Frame(self.panes, bg=PAL['sidebar'], width=350, padx=20, pady=25)
        self.panes.add(self.right_fr, weight=2)
        self.right_fr.pack_propagate(False)
        tk.Label(self.right_fr, text='NEURAL ANALYSIS', font=('Inter', 8, 'bold'), fg=PAL['accent'], bg=PAL['sidebar']).pack(anchor='w')
        self.ai_box = tk.Text(self.right_fr, bg='#000', fg=PAL['success'], font=('Consolas', 9), height=15, borderwidth=0, padx=15, pady=15)
        self.ai_box.pack(fill='x', pady=20)
        self.ai_box.insert('1.0', '[AGENT] Awaiting Thread Selection...')
        tk.Button(self.right_fr, text='AUTOMATE REPLY', font=('Inter', 8, 'bold'), bg=PAL['accent'], fg='white', relief='flat', pady=12, command=self._automate).pack(fill='x')
        self.status = tk.Label(self, text='SOVEREIGN EMAIL [V3.0] | P2P MESH: ACTIVE | THREADS: QUANTIZED', bg=PAL['accent'], fg='white', font=('Inter', 8, 'bold'), pady=5)
        self.status.pack(side='bottom', fill='x')