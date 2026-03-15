# Generated method: ProjectFlow._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class ProjectFlow:
    def _build_ui(self):
        head = tk.Frame(self, bg=PAL['bg'], padx=40, pady=30)
        head.pack(fill='x')
        tk.Label(head, text='PROJECTFLOW PRO', font=('Inter', 24, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        self.time_lbl = tk.Label(head, text='SESSION: 02:45:12', font=('JetBrains Mono', 12), fg=PAL['success'], bg=PAL['bg'])
        self.time_lbl.pack(side='right', padx=20)
        self.tabs = ttk.Notebook(self, style='PF.TNotebook')
        self.tabs.pack(fill='both', expand=True, padx=40)
        scrum_tab = tk.Frame(self.tabs, bg=PAL['bg'])
        self.tabs.add(scrum_tab, text=' 📋 SCRUM BOARD ')
        self._init_scrum(scrum_tab)
        gantt_tab = tk.Frame(self.tabs, bg=PAL['bg'])
        self.tabs.add(gantt_tab, text=' 📊 GANTT CHART ')
        self._init_gantt(gantt_tab)
        track_tab = tk.Frame(self.tabs, bg=PAL['bg'])
        self.tabs.add(track_tab, text=' ⏱️ TIME TRACKER ')
        self._init_tracker(track_tab)
        self.status = tk.Label(self, text='', bg=PAL['accent'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(side='bottom', fill='x')