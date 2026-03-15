# Generated method: SovereignAINexus._build_tasks_tab
import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import time, threading, random, os, sys, json

class SovereignAINexus:
    def _build_tasks_tab(self):
        tab = tk.Frame(self.nb, bg=PAL['bg'], padx=15, pady=15)
        self.nb.add(tab, text='  ✅ Task Manager  ')
        cols = ('ID', 'Task', 'Priority', 'Status', 'Source')
        self.task_tree = ttk.Treeview(tab, columns=cols, show='headings', height=15)
        for col in cols:
            self.task_tree.heading(col, text=col)
            self.task_tree.column(col, width=100 if col != 'Task' else 300, anchor='center')
        self.task_tree.pack(fill='both', expand=True)
        btn_fr = tk.Frame(tab, bg=PAL['bg'], pady=10)
        btn_fr.pack(fill='x')
        ttk.Button(btn_fr, text='➕ Add Task', command=self._add_task).pack(side='left', padx=5)
        ttk.Button(btn_fr, text='✔ Mark Done', command=self._mark_done).pack(side='left', padx=5)
        ttk.Button(btn_fr, text='🚀 Execute Auto-Task', command=self._exec_auto).pack(side='left', padx=5)