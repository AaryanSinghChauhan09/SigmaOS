# Generated method: AIStudio._build_ui
import tkinter as tk
from tkinter import ttk, messagebox
import random
import time
import threading
import sys
import os
from typing import Dict, Any, List, Optional

class AIStudio:
    def _build_ui(self):
        header = tk.Frame(self, bg=PAL['bg'], height=70, padx=25)
        header.pack(side='top', fill='x', pady=15)
        tk.Label(header, text=f"{ICONS.get('intelligence', '🧠')} OMNI AI STUDIO", font=('Inter', 20, 'bold'), fg=PAL['accent'], bg=PAL['bg']).pack(side='left')
        btn_fr = tk.Frame(header, bg=PAL['bg'])
        btn_fr.pack(side='right')
        nav_btns = [(f"{ICONS.get('viz_engine', '🧬')} XAI EXPLAINER", self._explain_model), (f"{ICONS.get('bootloader', '🚀')} TRAIN", self._train_model)]
        for txt, cmd in nav_btns:
            tk.Button(btn_fr, text=txt, font=('Inter', 9, 'bold'), bg=PAL['sidebar'], fg='white', relief='flat', padx=15, pady=8, command=cmd).pack(side='left', padx=5)
        ws = tk.Frame(self, bg=PAL['bg'], padx=25, pady=10)
        ws.pack(fill='both', expand=True)
        self.tabs = ttk.Notebook(ws, style='Studio.TNotebook')
        self.tabs.pack(fill='both', expand=True)
        self._build_ds_tab(tk.Frame(self.tabs, bg=PAL['bg'], padx=15, pady=15))
        self._build_ml_tab(tk.Frame(self.tabs, bg=PAL['bg'], padx=15, pady=15))
        self._build_dl_tab(tk.Frame(self.tabs, bg=PAL['bg'], padx=15, pady=15))
        prog_fr = tk.Frame(self, bg=PAL['bg'])
        prog_fr.pack(side='bottom', fill='x')
        self.pbar = ttk.Progressbar(prog_fr, style='Studio.Horizontal.TProgressbar', length=100, mode='determinate')
        self.status = tk.Label(prog_fr, text='GPU TENSOR CORES: IDLE | OMNI-AUTOML READY', bg=PAL['accent'], fg='white', font=('Inter', 8, 'bold'), pady=6)
        self.status.pack(fill='x')