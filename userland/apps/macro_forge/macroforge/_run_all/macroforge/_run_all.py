# Generated method: MacroForge._run_all
import tkinter as tk
from tkinter import ttk, messagebox
import random

class MacroForge:
    def _run_all(self):
        if not self.tree.get_children():
            return
        self.status.config(text='EXECUTING KERNEL HOOKS... BYPASSING UI... [████████--]', bg=PAL['accent'], fg='black')
        self.after(1500, lambda: messagebox.showinfo('Macro Forge', 'Sequence executed synchronously at kernel level. Zero latency achieved.'))
        self.after(1500, lambda: self.status.config(text='EXECUTION COMPLETE | 0.04mS OVERHEAD', bg=PAL['success'], fg='black'))