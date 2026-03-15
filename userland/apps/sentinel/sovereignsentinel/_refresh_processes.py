"""
Auto-split from userland\apps\sentinel.py — SovereignSentinel._refresh_processes
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess



class SovereignSentinel:
    def _refresh_processes(self):
        self.proc_tree.delete(*self.proc_tree.get_children())
        q = self.proc_search.get().strip().lower()
        procs = [('102', 'sigma_kernel', '0.1', '45', 'RUNNING', '★★★★★'), ('280', 'sigma_gui', '1.2', '128', 'RUNNING', '★★★★★'), ('450', 'sigma_browser', '2.1', '340', 'RUNNING', '★★★★☆'), ('620', 'omni_automator', '0.4', '85', 'RUNNING', '★★★★★'), ('882', 'native_shim', '0.3', '22', 'SLEEPING', '★★★☆☆'), ('1024', 'sovereign_mesh', '0.8', '60', 'RUNNING', '★★★★★')]
        for pid, name, cpu, ram, status, trust in procs:
            if q and q not in name.lower():
                continue
            tag = 'safe' if trust.count('★') >= 4 else 'warn'
            self.proc_tree.insert('', 'end', values=(pid, name, f'{cpu}%', f'{ram} MB', status, trust), tags=(tag,))
        self.proc_tree.tag_configure('safe', foreground=PAL['text'])
        self.proc_tree.tag_configure('warn', foreground=PAL['accent'])
