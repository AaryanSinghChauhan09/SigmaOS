# Generated method: OmniToolsApp._build_timer_tab
import tkinter as tk
from tkinter import ttk, messagebox, colorchooser, filedialog
import math
import random
import string
import secrets
import datetime

class OmniToolsApp:
    def _build_timer_tab(self) -> None:
        tk.Label(self.tab_timer, text='Offline Timer & Pomodoro Suite', font=('Inter', 14, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(anchor='w', pady=(0, 10))
        tf = tk.Frame(self.tab_timer, bg=PAL['panel'], padx=15, pady=15)
        tf.pack(fill='x', pady=5)
        tk.Label(tf, text='Countdown (seconds):', font=('Inter', 10), fg=PAL['text'], bg=PAL['panel']).pack(side='left')
        self.timer_entry = tk.Entry(tf, width=7, font=('Inter', 10), bg=PAL['bg'], fg=PAL['accent'], insertbackground=PAL['accent'], relief='flat')
        self.timer_entry.pack(side='left', padx=8)
        tk.Button(tf, text='START', bg=PAL['success'], fg='black', font=('Inter', 9, 'bold'), command=self._start_timer).pack(side='left', padx=5)
        self.timer_label = tk.Label(self.tab_timer, text='Idle', font=('Inter', 16, 'bold'), fg=PAL['dim'], bg=PAL['bg'])
        self.timer_label.pack(pady=15)
        pf = tk.Frame(self.tab_timer, bg=PAL['bg'])
        pf.pack(pady=10)
        for lbl, w, b in [('Classic 25/5', 25, 5), ('Focus 50/10', 50, 10), ('Sprint 15/3', 15, 3)]:
            tk.Button(pf, text=lbl, bg=PAL['accent_dim'], fg='black', font=('Inter', 9, 'bold'), command=lambda ww=w, bb=b: self._run_pomodoro(ww, bb)).pack(side='left', padx=8)