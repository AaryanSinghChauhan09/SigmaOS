"""
Auto-split from userland\apps\sentinel.py — SovereignSentinel._kill_proc
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess



class SovereignSentinel:
    def _kill_proc(self):
        sel = self.proc_tree.selection()
        if not sel:
            messagebox.showwarning('Selection', 'Select a process first.')
            return
        item = self.proc_tree.item(sel[0])
        pid, name = (item['values'][0], item['values'][1])
        if messagebox.askyesno('Terminate', f"Vaporize process '{name}' (PID {pid})? Memory blocks will be wiped."):
            self.proc_tree.delete(sel[0])
            self.scan_log.insert('end', f"\n[GUARDIAN] Process '{name}' ({pid}) vaporized. Memory purged.")
            self.status.config(text=f'Process {name} neutralized.', bg=PAL['accent'])
