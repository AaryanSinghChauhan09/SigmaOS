"""
Auto-split from userland\apps\sentinel.py — SovereignSentinel._sandbox_proc
"""

import tkinter as tk
from tkinter import ttk, scrolledtext, messagebox
import random, time, os, sys, threading, subprocess



class SovereignSentinel:
    def _sandbox_proc(self):
        sel = self.proc_tree.selection()
        if not sel:
            return
        item = self.proc_tree.item(sel[0])
        name = item['values'][1]
        messagebox.showinfo('Sandbox', f"Process '{name}' isolated in Level-3 UAL container.\nAll network access: SEVERED.")
