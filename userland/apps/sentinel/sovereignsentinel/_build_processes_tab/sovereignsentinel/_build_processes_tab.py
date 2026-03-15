# Generated method: SovereignSentinel._build_processes_tab
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess

class SovereignSentinel:
    def _build_processes_tab(self):
        frame = tk.Frame(self.nb, bg=PAL['bg'])
        self.nb.add(frame, text='  🔬 Processes  ')
        ctrl_fr = tk.Frame(frame, bg=PAL['bg'], pady=10, padx=20)
        ctrl_fr.pack(fill='x')
        self.proc_search = ttk.Entry(ctrl_fr, width=30)
        self.proc_search.pack(side='left', padx=(0, 10))
        self.proc_search.insert(0, 'Filter processes...')
        self.proc_search.bind('<KeyRelease>', lambda e: self._refresh_processes())
        ttk.Button(ctrl_fr, text='↻ Refresh', command=self._refresh_processes).pack(side='left', padx=5)
        ttk.Button(ctrl_fr, text='🔴 Terminate Selected', command=self._kill_proc).pack(side='left', padx=5)
        ttk.Button(ctrl_fr, text='🛡 Sandbox Selected', command=self._sandbox_proc).pack(side='left', padx=5)
        cols = ('PID', 'Name', 'CPU%', 'RAM MB', 'Status', 'Trust')
        self.proc_tree = ttk.Treeview(frame, columns=cols, show='headings', height=22)
        for col in cols:
            self.proc_tree.heading(col, text=col, command=lambda c=col: self._sort_proc(c))
            self.proc_tree.column(col, width=120 if col in ('Name',) else 80, anchor='center')
        self.proc_tree.pack(fill='both', expand=True, padx=20)
        vsb = ttk.Scrollbar(frame, orient='vertical', command=self.proc_tree.yview)
        self.proc_tree.configure(yscrollcommand=vsb.set)