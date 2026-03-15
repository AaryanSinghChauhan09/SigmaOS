# Generated method: ProjectFlow._init_tracker
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time

class ProjectFlow:
    def _init_tracker(self, parent):
        body = tk.Frame(parent, bg=PAL['bg'], pady=40)
        body.pack()
        tk.Label(body, text='ACTIVE MODULE TRACKING', font=('Inter', 16, 'bold'), fg=PAL['secondary'], bg=PAL['bg']).pack()
        self.big_time = tk.Label(body, text='02:45:12', font=('JetBrains Mono', 72, 'bold'), fg='white', bg=PAL['bg'])
        self.big_time.pack(pady=40)
        btn_fr = tk.Frame(body, bg=PAL['bg'])
        btn_fr.pack()
        tk.Button(btn_fr, text='PAUSE SESSION', bg=PAL['warning'], fg='black', font=('Inter', 10, 'bold'), relief='flat', padx=30, pady=12).pack(side='left', padx=10)
        tk.Button(btn_fr, text='LOG ACTIVITY', bg=PAL['accent'], fg='white', font=('Inter', 10, 'bold'), relief='flat', padx=30, pady=12).pack(side='left', padx=10)